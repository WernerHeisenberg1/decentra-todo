#![cfg_attr(not(feature = "std"), no_std)]

/// 任务管理 Pallet
///
/// 提供基础的任务管理功能：
/// - 创建任务
/// - 编辑任务
/// - 删除任务
/// - 任务状态管理
/// - 任务优先级和难度评估
pub use pallet::*;

// 导入子模块
#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
pub mod types;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::types::{Task, TaskSearchParams, TaskSortBy, TaskStatistics, TaskStatus};
    use codec::MaxEncodedLen;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, Get, Randomness, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, SaturatedConversion};
    use sp_std::vec::Vec;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaxEncodedLen
            + core::fmt::Debug;

        /// The moment type for timestamps (using BlockNumber for simplicity)
        type Moment: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaxEncodedLen
            + core::fmt::Debug;

        /// Currency type for handling token transfers
        type Currency: Currency<Self::AccountId, Balance = Self::Balance>
            + ReservableCurrency<Self::AccountId>;

        /// 任务标题最大长度
        #[pallet::constant]
        type MaxTitleLength: Get<u32>;

        /// 任务描述最大长度
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        /// 每个用户最大任务数量
        #[pallet::constant]
        type MaxTasksPerUser: Get<u32>;

        /// 每个状态下的最大任务数量
        #[pallet::constant]
        type MaxTasksPerStatus: Get<u32>;

        /// 每个优先级下的最大任务数量
        #[pallet::constant]
        type MaxTasksPerPriority: Get<u32>;

        /// 每个截止日期下的最大任务数量
        #[pallet::constant]
        type MaxTasksPerDeadline: Get<u32>;

        /// 随机数生成器
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// 社区验证所需的最小投票数
        #[pallet::constant]
        type MinVerificationVotes: Get<u32>;

        /// 社区验证通过所需的最小赞成票比例（百分比）
        #[pallet::constant]
        type MinApprovalPercentage: Get<u32>;

        /// 验证投票期限（区块数）
        #[pallet::constant]
        type VerificationPeriod: Get<BlockNumberFor<Self>>;
    }

    /// 任务存储映射
    #[pallet::storage]
    #[pallet::getter(fn tasks)]
    pub type Tasks<T: Config> = StorageMap<_, Blake2_128Concat, u32, Task<T>>;

    /// 用户创建的任务列表
    #[pallet::storage]
    #[pallet::getter(fn user_created_tasks)]
    pub type UserCreatedTasks<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u32, T::MaxTasksPerUser>,
        ValueQuery,
    >;

    /// 用户分配的任务列表
    #[pallet::storage]
    #[pallet::getter(fn user_assigned_tasks)]
    pub type UserAssignedTasks<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u32, T::MaxTasksPerUser>,
        ValueQuery,
    >;

    /// 下一个任务ID
    #[pallet::storage]
    #[pallet::getter(fn next_task_id)]
    pub type NextTaskId<T> = StorageValue<_, u32, ValueQuery>;

    /// 任务状态索引
    #[pallet::storage]
    #[pallet::getter(fn tasks_by_status)]
    pub type TasksByStatus<T: Config> =
        StorageMap<_, Blake2_128Concat, u8, BoundedVec<u32, T::MaxTasksPerStatus>, ValueQuery>;

    /// 任务优先级索引
    #[pallet::storage]
    #[pallet::getter(fn tasks_by_priority)]
    pub type TasksByPriority<T: Config> =
        StorageMap<_, Blake2_128Concat, u8, BoundedVec<u32, T::MaxTasksPerPriority>, ValueQuery>;

    /// 任务截止日期索引
    #[pallet::storage]
    #[pallet::getter(fn tasks_by_deadline)]
    pub type TasksByDeadline<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::Moment,
        BoundedVec<u32, T::MaxTasksPerDeadline>,
        ValueQuery,
    >;

    /// 社区验证投票存储
    #[pallet::storage]
    #[pallet::getter(fn verification_votes)]
    pub type VerificationVotes<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // task_id
        Blake2_128Concat,
        T::AccountId, // voter
        bool,         // true for approve, false for reject
    >;

    /// 任务验证状态
    #[pallet::storage]
    #[pallet::getter(fn verification_status)]
    pub type VerificationStatus<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,                           // task_id
        (BlockNumberFor<T>, u32, u32), // (end_block, approve_votes, reject_votes)
    >;

    /// 已参与验证投票的用户列表
    #[pallet::storage]
    #[pallet::getter(fn verification_voters)]
    pub type VerificationVoters<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,                                      // task_id
        BoundedVec<T::AccountId, ConstU32<1000>>, // 假设最多1000个投票者
        ValueQuery,
    >;

    // Pallets use events to inform users when important changes are made.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 任务已创建
        TaskCreated {
            task_id: u32,
            creator: T::AccountId,
            title: Vec<u8>,
        },
        /// 任务已更新
        TaskUpdated { task_id: u32, updater: T::AccountId },
        /// 任务状态已更改
        TaskStatusChanged {
            task_id: u32,
            old_status: u8,
            new_status: u8,
            assignee: Option<T::AccountId>,
        },
        /// 任务已分配给执行者
        TaskAssigned {
            task_id: u32,
            assignee: T::AccountId,
        },
        /// 任务分配已取消
        TaskUnassigned {
            task_id: u32,
            previous_assignee: T::AccountId,
        },
        /// 任务已删除
        TaskDeleted {
            task_id: u32,
            deleted_by: T::AccountId,
        },
        /// 任务完成奖励已发放
        TaskRewardPaid {
            task_id: u32,
            assignee: T::AccountId,
            creator: T::AccountId,
            reward: T::Balance,
        },
        /// 任务奖励预留（创建任务时锁定奖励）
        TaskRewardReserved {
            task_id: u32,
            creator: T::AccountId,
            reward: T::Balance,
        },
        /// 任务奖励释放（任务取消时释放锁定的奖励）
        TaskRewardReleased {
            task_id: u32,
            creator: T::AccountId,
            reward: T::Balance,
        },
        /// 社区验证已开始
        CommunityVerificationStarted {
            task_id: u32,
            end_block: BlockNumberFor<T>,
        },
        /// 社区验证投票已提交
        VerificationVoteSubmitted {
            task_id: u32,
            voter: T::AccountId,
            approve: bool,
        },
        /// 社区验证已完成
        CommunityVerificationCompleted {
            task_id: u32,
            approved: bool,
            approve_votes: u32,
            reject_votes: u32,
        },
        /// 社区验证已超时
        CommunityVerificationExpired { task_id: u32 },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// 任务不存在
        TaskNotFound,
        /// 无权限操作该任务
        NotAuthorized,
        /// 任务标题过长
        TitleTooLong,
        /// 任务描述过长
        DescriptionTooLong,
        /// 任务数量达到上限
        TooManyTasks,
        /// 优先级值无效（必须在1-4之间）
        InvalidPriority,
        /// 难度值无效（必须在1-10之间）
        InvalidDifficulty,
        /// 任务状态转换无效
        InvalidStatusTransition,
        /// 任务已经分配给其他人
        TaskAlreadyAssigned,
        /// 任务未分配
        TaskNotAssigned,
        /// 不能分配给自己创建的任务
        CannotAssignToSelf,
        /// 任务已过期
        TaskExpired,
        /// 余额不足，无法创建任务或支付奖励
        InsufficientBalance,
        /// 奖励金额无效（必须大于0）
        InvalidReward,
        /// 奖励转移失败
        RewardTransferFailed,
        /// 任务不在待验证状态
        TaskNotPendingVerification,
        /// 验证投票期间未结束
        VerificationPeriodNotEnded,
        /// 验证投票期间已结束
        VerificationPeriodEnded,
        /// 已经投过票了
        AlreadyVoted,
        /// 不能为自己创建或执行的任务投票
        CannotVoteOwnTask,
        /// 验证投票数量不足
        InsufficientVerificationVotes,
        /// 用户声誉不足，无法参与验证投票
        InsufficientReputationToVote,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建新任务
        #[pallet::weight(10_000)]
        pub fn create_task(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            priority: u8,
            difficulty: u8,
            reward: T::Balance,
            deadline: Option<T::Moment>,
        ) -> DispatchResult {
            let creator = ensure_signed(origin)?;

            // 验证奖励金额
            ensure!(!reward.is_zero(), Error::<T>::InvalidReward);

            // 验证创建者有足够余额支付奖励
            ensure!(
                T::Currency::can_reserve(&creator, reward),
                Error::<T>::InsufficientBalance
            );

            // 验证优先级值（1-4: Low=1, Medium=2, High=3, Urgent=4）
            ensure!(priority >= 1 && priority <= 4, Error::<T>::InvalidPriority);

            // 验证难度值（1-10）
            ensure!(
                difficulty >= 1 && difficulty <= 10,
                Error::<T>::InvalidDifficulty
            );

            // 验证截止日期
            Self::validate_deadline(deadline)?;

            // 验证标题长度
            let bounded_title: BoundedVec<_, _> =
                title.try_into().map_err(|_| Error::<T>::TitleTooLong)?;

            // 验证描述长度
            let bounded_description: BoundedVec<_, _> = description
                .try_into()
                .map_err(|_| Error::<T>::DescriptionTooLong)?;

            // 预留奖励金额（锁定创建者的代币）
            T::Currency::reserve(&creator, reward).map_err(|_| Error::<T>::InsufficientBalance)?;

            // 获取下一个任务ID
            let task_id = NextTaskId::<T>::get();
            NextTaskId::<T>::put(task_id + 1);

            // 创建任务
            let now = Self::current_timestamp();
            let task = Task::new(
                task_id,
                creator.clone(),
                bounded_title.clone(),
                bounded_description,
                priority,
                difficulty,
                reward,
                now,
                deadline,
            );

            // 存储任务
            Tasks::<T>::insert(task_id, &task);

            // 更新用户创建的任务列表
            UserCreatedTasks::<T>::mutate(creator.clone(), |tasks| {
                tasks
                    .try_push(task_id)
                    .map_err(|_| Error::<T>::TooManyTasks)
            })?;

            // 更新任务索引
            Self::update_task_indices(&task)?;

            // 发出事件
            Self::deposit_event(Event::TaskCreated {
                task_id,
                creator: creator.clone(),
                title: bounded_title.to_vec(),
            });

            // 发出奖励预留事件
            Self::deposit_event(Event::TaskRewardReserved {
                task_id,
                creator,
                reward,
            });

            Ok(())
        }

        /// 更新任务
        #[pallet::weight(10_000)]
        pub fn update_task(
            origin: OriginFor<T>,
            task_id: u32,
            title: Option<Vec<u8>>,
            description: Option<Vec<u8>>,
            priority: Option<u8>,
            difficulty: Option<u8>,
            reward: Option<T::Balance>,
            deadline: Option<Option<T::Moment>>,
        ) -> DispatchResult {
            let updater = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 验证权限
            ensure!(task.can_be_modified_by(&updater), Error::<T>::NotAuthorized);

            // 更新任务字段
            if let Some(title) = title {
                let bounded_title: BoundedVec<_, _> =
                    title.try_into().map_err(|_| Error::<T>::TitleTooLong)?;
                task.title = bounded_title;
            }

            if let Some(description) = description {
                let bounded_description: BoundedVec<_, _> = description
                    .try_into()
                    .map_err(|_| Error::<T>::DescriptionTooLong)?;
                task.description = bounded_description;
            }

            if let Some(priority) = priority {
                // 验证优先级值
                ensure!(priority >= 1 && priority <= 4, Error::<T>::InvalidPriority);
                task.priority = priority;
            }

            if let Some(difficulty) = difficulty {
                // 验证难度值
                ensure!(
                    difficulty >= 1 && difficulty <= 10,
                    Error::<T>::InvalidDifficulty
                );
                task.difficulty = difficulty;
            }

            if let Some(new_reward) = reward {
                // 验证新的奖励金额
                ensure!(!new_reward.is_zero(), Error::<T>::InvalidReward);

                // 只有在任务未完成且未取消的情况下才能修改奖励
                ensure!(
                    task.status != 2 && task.status != 3, // 不是 Completed 或 Cancelled
                    Error::<T>::InvalidStatusTransition
                );

                // 计算奖励差额
                let old_reward = task.reward;
                if new_reward > old_reward {
                    // 增加奖励：需要额外预留资金
                    let additional = new_reward - old_reward;
                    ensure!(
                        T::Currency::can_reserve(&task.creator, additional),
                        Error::<T>::InsufficientBalance
                    );
                    T::Currency::reserve(&task.creator, additional)
                        .map_err(|_| Error::<T>::InsufficientBalance)?;
                } else if new_reward < old_reward {
                    // 减少奖励：释放多余的预留资金
                    let excess = old_reward - new_reward;
                    T::Currency::unreserve(&task.creator, excess);
                }

                task.reward = new_reward;
            }

            if let Some(deadline) = deadline {
                task.deadline = deadline;
            }

            // 更新任务
            task.updated_at = Self::current_timestamp();
            Tasks::<T>::insert(task_id, &task);

            // 更新任务索引
            Self::update_task_indices(&task)?;

            // 发出事件
            Self::deposit_event(Event::TaskUpdated { task_id, updater });

            Ok(())
        }

        /// 更改任务状态
        #[pallet::weight(10_000)]
        pub fn change_task_status(
            origin: OriginFor<T>,
            task_id: u32,
            new_status: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 验证权限
            ensure!(task.can_be_operated_by(&who), Error::<T>::NotAuthorized);

            // 验证状态转换
            Self::validate_status_transition(&task.status, &new_status)?;

            // 如果转换到PendingVerification状态，启动社区验证
            if new_status == 4 {
                // PendingVerification
                Self::start_community_verification(task_id)?;
            }

            // 处理奖励发放和释放
            Self::handle_reward_on_status_change(&task, task.status, new_status)?;

            // 更新状态
            let old_status = task.status;
            task.status = new_status;
            task.updated_at = Self::current_timestamp();

            // 存储更新后的任务
            Tasks::<T>::insert(task_id, &task);

            // 更新任务索引
            Self::update_task_indices(&task)?;

            // TODO: 集成声誉系统 - 当任务状态变更时通知声誉模块
            // 例如：当任务完成或取消时，应该更新执行者的声誉
            // if let Some(assignee) = &task.assignee {
            //     // 调用声誉模块的状态变更处理函数
            //     pallet_reputation::Pallet::<T>::on_task_status_changed(
            //         task_id,
            //         Some(assignee),
            //         old_status,
            //         new_status,
            //         task.difficulty,
            //     )?;
            // }

            // 发出事件
            Self::deposit_event(Event::TaskStatusChanged {
                task_id,
                old_status,
                new_status,
                assignee: task.assignee.clone(),
            });

            Ok(())
        }

        /// 分配任务
        #[pallet::weight(10_000)]
        pub fn assign_task(
            origin: OriginFor<T>,
            task_id: u32,
            assignee: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 验证权限
            ensure!(task.can_be_modified_by(&who), Error::<T>::NotAuthorized);

            // 检查任务是否过期
            Self::check_task_expired(&task)?;

            // 验证任务状态
            ensure!(
                task.status == 0, // Pending
                Error::<T>::InvalidStatusTransition
            );

            // 验证任务未分配
            ensure!(!task.is_assigned(), Error::<T>::TaskAlreadyAssigned);

            // 验证不能分配给自己
            ensure!(task.creator != assignee, Error::<T>::CannotAssignToSelf);

            // 分配任务
            task.assignee = Some(assignee.clone());
            task.updated_at = Self::current_timestamp();

            // 存储更新后的任务
            Tasks::<T>::insert(task_id, &task);

            // 更新分配者的任务列表
            UserAssignedTasks::<T>::mutate(&assignee, |tasks| {
                tasks
                    .try_push(task_id)
                    .map_err(|_| Error::<T>::TooManyTasks)
            })?;

            // 发出事件
            Self::deposit_event(Event::TaskAssigned { task_id, assignee });

            Ok(())
        }

        /// 取消分配任务
        #[pallet::weight(10_000)]
        pub fn unassign_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 验证权限
            ensure!(task.can_be_modified_by(&who), Error::<T>::NotAuthorized);

            // 验证任务已分配
            let previous_assignee = task.assignee.take().ok_or(Error::<T>::TaskNotAssigned)?;

            // 更新任务状态为Pending
            task.status = 0; // Pending
            task.updated_at = Self::current_timestamp();

            // 存储更新后的任务
            Tasks::<T>::insert(task_id, &task);

            // 从分配者的任务列表中移除
            UserAssignedTasks::<T>::mutate(&previous_assignee, |tasks| {
                tasks.retain(|&x| x != task_id);
            });

            // 发出事件
            Self::deposit_event(Event::TaskUnassigned {
                task_id,
                previous_assignee,
            });

            Ok(())
        }

        /// 删除任务
        #[pallet::weight(10_000)]
        pub fn delete_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 验证权限
            ensure!(task.can_be_modified_by(&who), Error::<T>::NotAuthorized);

            // 如果任务还没有完成或取消，释放预留的奖励
            if task.status != 2 && task.status != 3 {
                // 不是 Completed 或 Cancelled
                T::Currency::unreserve(&task.creator, task.reward);

                // 发出奖励释放事件
                Self::deposit_event(Event::TaskRewardReleased {
                    task_id,
                    creator: task.creator.clone(),
                    reward: task.reward,
                });
            }

            // 从存储中移除任务
            Tasks::<T>::remove(task_id);

            // 清理任务索引
            Self::remove_task_indices(&task)?;

            // 从创建者的任务列表中移除
            UserCreatedTasks::<T>::mutate(&task.creator, |tasks| {
                tasks.retain(|&x| x != task_id);
            });

            // 如果任务有分配者，从分配者的任务列表中移除
            if let Some(assignee) = &task.assignee {
                UserAssignedTasks::<T>::mutate(assignee, |tasks| {
                    tasks.retain(|&x| x != task_id);
                });
            }

            // 发出事件
            Self::deposit_event(Event::TaskDeleted {
                task_id,
                deleted_by: who,
            });

            Ok(())
        }

        /// 提交社区验证投票
        #[pallet::weight(10_000)]
        pub fn submit_verification_vote(
            origin: OriginFor<T>,
            task_id: u32,
            approve: bool,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 检查任务是否存在
            let task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 检查任务是否在待验证状态
            ensure!(task.status == 4, Error::<T>::TaskNotPendingVerification);

            // 检查验证期间是否还未结束
            let (end_block, approve_votes, reject_votes) = VerificationStatus::<T>::get(task_id)
                .ok_or(Error::<T>::TaskNotPendingVerification)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
            ensure!(
                current_block <= end_block,
                Error::<T>::VerificationPeriodEnded
            );

            // 检查是否已经投过票
            ensure!(
                !VerificationVotes::<T>::contains_key(task_id, &who),
                Error::<T>::AlreadyVoted
            );

            // 检查不能为自己创建或执行的任务投票
            ensure!(task.creator != who, Error::<T>::CannotVoteOwnTask);
            if let Some(assignee) = &task.assignee {
                ensure!(*assignee != who, Error::<T>::CannotVoteOwnTask);
            }

            // TODO: 检查用户声誉是否足够参与验证（可选）
            // 这里可以添加声誉要求，例如要求用户达到一定等级才能参与验证

            // 记录投票
            VerificationVotes::<T>::insert(task_id, &who, approve);

            // 更新投票计数
            let new_approve_votes = if approve {
                approve_votes + 1
            } else {
                approve_votes
            };
            let new_reject_votes = if approve {
                reject_votes
            } else {
                reject_votes + 1
            };

            VerificationStatus::<T>::insert(
                task_id,
                (end_block, new_approve_votes, new_reject_votes),
            );

            // 添加到投票者列表
            VerificationVoters::<T>::mutate(task_id, |voters| {
                voters
                    .try_push(who.clone())
                    .map_err(|_| Error::<T>::TooManyTasks)
            })?;

            // 发出事件
            Self::deposit_event(Event::VerificationVoteSubmitted {
                task_id,
                voter: who,
                approve,
            });

            // 检查是否达到最小投票数，如果达到则尝试完成验证
            let total_votes = new_approve_votes + new_reject_votes;
            if total_votes >= T::MinVerificationVotes::get() {
                Self::try_complete_verification(task_id)?;
            }

            Ok(())
        }

        /// 完成社区验证
        #[pallet::weight(10_000)]
        pub fn complete_verification(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Self::try_complete_verification(task_id)
        }
    }

    // Helper methods
    impl<T: Config> Pallet<T> {
        /// 获取当前时间戳
        ///
        /// 注意：在实际项目中，应该集成 pallet_timestamp 来获取准确的区块时间戳
        /// 目前使用区块号作为时间戳的替代方案
        pub fn current_timestamp() -> T::Moment {
            let block_number = <frame_system::Pallet<T>>::block_number();
            T::Moment::from(block_number.saturated_into::<u32>())
        }

        /// 根据状态获取任务
        pub fn get_tasks_by_status(status: u8) -> Vec<Task<T>> {
            TasksByStatus::<T>::get(status)
                .iter()
                .filter_map(|&task_id| Tasks::<T>::get(task_id))
                .collect()
        }

        /// 根据优先级获取任务
        pub fn get_tasks_by_priority(priority: u8) -> Vec<Task<T>> {
            TasksByPriority::<T>::get(priority)
                .iter()
                .filter_map(|&task_id| Tasks::<T>::get(task_id))
                .collect()
        }

        /// 根据截止日期获取任务
        pub fn get_tasks_by_deadline(deadline: T::Moment) -> Vec<Task<T>> {
            TasksByDeadline::<T>::get(deadline)
                .iter()
                .filter_map(|&task_id| Tasks::<T>::get(task_id))
                .collect()
        }

        /// 获取用户创建的任务
        pub fn get_user_created_tasks(user: &T::AccountId) -> Vec<Task<T>> {
            UserCreatedTasks::<T>::get(user)
                .iter()
                .filter_map(|&task_id| Tasks::<T>::get(task_id))
                .collect()
        }

        /// 获取用户被分配的任务
        pub fn get_user_assigned_tasks(user: &T::AccountId) -> Vec<Task<T>> {
            UserAssignedTasks::<T>::get(user)
                .iter()
                .filter_map(|&task_id| Tasks::<T>::get(task_id))
                .collect()
        }

        /// 获取所有待处理的任务
        pub fn get_pending_tasks() -> Vec<Task<T>> {
            Self::get_tasks_by_status(0) // Pending
        }

        /// 获取所有进行中的任务
        pub fn get_in_progress_tasks() -> Vec<Task<T>> {
            Self::get_tasks_by_status(1) // InProgress
        }

        /// 获取所有已完成的任务
        pub fn get_completed_tasks() -> Vec<Task<T>> {
            Self::get_tasks_by_status(2) // Completed
        }

        /// 获取即将到期的任务（未来指定时间内）
        pub fn get_expiring_tasks(within_blocks: T::Moment) -> Vec<Task<T>> {
            let now = Self::current_timestamp();
            let threshold = now + within_blocks;

            // 这里需要遍历所有任务，实际应用中可能需要更高效的索引
            let mut expiring_tasks = Vec::new();

            // 注意：这是一个简化的实现，实际应用中应该使用更高效的查询方式
            for task in Tasks::<T>::iter_values() {
                if let Some(deadline) = task.deadline {
                    if deadline <= threshold && deadline > now {
                        expiring_tasks.push(task);
                    }
                }
            }

            expiring_tasks
        }

        /// 高级搜索：根据多个条件搜索任务
        pub fn search_tasks(search_params: TaskSearchParams<T>) -> Vec<Task<T>> {
            let mut all_tasks: Vec<Task<T>> = Tasks::<T>::iter_values().collect();

            // 应用所有过滤条件
            if !search_params.keyword.is_empty() {
                all_tasks.retain(|task| {
                    let keyword_lower = sp_std::str::from_utf8(&search_params.keyword)
                        .unwrap_or("")
                        .to_lowercase();

                    let title_match = sp_std::str::from_utf8(&task.title)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    let desc_match = sp_std::str::from_utf8(&task.description)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    title_match || desc_match
                });
            }

            // 按状态筛选
            if let Some(status) = search_params.status {
                all_tasks.retain(|task| task.status == status);
            }

            // 按优先级筛选
            if let Some(priority) = search_params.priority {
                all_tasks.retain(|task| task.priority == priority);
            }

            // 按难度范围筛选
            if let Some((min_difficulty, max_difficulty)) = search_params.difficulty_range {
                all_tasks.retain(|task| {
                    task.difficulty >= min_difficulty && task.difficulty <= max_difficulty
                });
            }

            // 按奖励范围筛选
            if let Some((min_reward, max_reward)) = search_params.reward_range {
                all_tasks.retain(|task| task.reward >= min_reward && task.reward <= max_reward);
            }

            // 按创建者筛选
            if let Some(ref creator) = search_params.creator {
                all_tasks.retain(|task| task.creator == *creator);
            }

            // 按执行者筛选
            if let Some(ref assignee) = search_params.assignee {
                all_tasks.retain(|task| {
                    if let Some(ref task_assignee) = task.assignee {
                        task_assignee == assignee
                    } else {
                        false
                    }
                });
            }

            // 按截止日期范围筛选
            if let Some((start_date, end_date)) = search_params.deadline_range {
                all_tasks.retain(|task| {
                    if let Some(deadline) = task.deadline {
                        deadline >= start_date && deadline <= end_date
                    } else {
                        false
                    }
                });
            }

            // 按创建时间范围筛选
            if let Some((start_time, end_time)) = search_params.created_time_range {
                all_tasks
                    .retain(|task| task.created_at >= start_time && task.created_at <= end_time);
            }

            // 只显示有截止日期的任务
            if search_params.has_deadline {
                all_tasks.retain(|task| task.deadline.is_some());
            }

            // 只显示未分配的任务
            if search_params.unassigned_only {
                all_tasks.retain(|task| task.assignee.is_none());
            }

            // 排序
            match search_params.sort_by {
                TaskSortBy::CreatedAt => {
                    all_tasks.sort_by(|a, b| {
                        if search_params.sort_desc {
                            b.created_at.cmp(&a.created_at)
                        } else {
                            a.created_at.cmp(&b.created_at)
                        }
                    });
                }
                TaskSortBy::UpdatedAt => {
                    all_tasks.sort_by(|a, b| {
                        if search_params.sort_desc {
                            b.updated_at.cmp(&a.updated_at)
                        } else {
                            a.updated_at.cmp(&b.updated_at)
                        }
                    });
                }
                TaskSortBy::Deadline => {
                    all_tasks.sort_by(|a, b| {
                        let a_deadline = a.deadline.unwrap_or(T::Moment::from(u32::MAX));
                        let b_deadline = b.deadline.unwrap_or(T::Moment::from(u32::MAX));
                        if search_params.sort_desc {
                            b_deadline.cmp(&a_deadline)
                        } else {
                            a_deadline.cmp(&b_deadline)
                        }
                    });
                }
                TaskSortBy::Reward => {
                    all_tasks.sort_by(|a, b| {
                        if search_params.sort_desc {
                            b.reward.cmp(&a.reward)
                        } else {
                            a.reward.cmp(&b.reward)
                        }
                    });
                }
                TaskSortBy::Difficulty => {
                    all_tasks.sort_by(|a, b| {
                        if search_params.sort_desc {
                            b.difficulty.cmp(&a.difficulty)
                        } else {
                            a.difficulty.cmp(&b.difficulty)
                        }
                    });
                }
                TaskSortBy::Priority => {
                    all_tasks.sort_by(|a, b| {
                        if search_params.sort_desc {
                            b.priority.cmp(&a.priority)
                        } else {
                            a.priority.cmp(&b.priority)
                        }
                    });
                }
            }

            // 分页
            let start_index = (search_params.page * search_params.page_size) as usize;
            let page_size = search_params.page_size as usize;

            if start_index < all_tasks.len() {
                all_tasks
                    .into_iter()
                    .skip(start_index)
                    .take(page_size)
                    .collect()
            } else {
                Vec::new()
            }
        }

        /// 获取搜索结果总数（用于分页）
        pub fn count_search_results(search_params: TaskSearchParams<T>) -> u32 {
            let mut all_tasks: Vec<Task<T>> = Tasks::<T>::iter_values().collect();

            // 应用过滤条件（除了分页和排序）
            if !search_params.keyword.is_empty() {
                all_tasks.retain(|task| {
                    let keyword_lower = sp_std::str::from_utf8(&search_params.keyword)
                        .unwrap_or("")
                        .to_lowercase();

                    let title_match = sp_std::str::from_utf8(&task.title)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    let desc_match = sp_std::str::from_utf8(&task.description)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    title_match || desc_match
                });
            }

            if let Some(status) = search_params.status {
                all_tasks.retain(|task| task.status == status);
            }

            if let Some(priority) = search_params.priority {
                all_tasks.retain(|task| task.priority == priority);
            }

            if let Some((min_difficulty, max_difficulty)) = search_params.difficulty_range {
                all_tasks.retain(|task| {
                    task.difficulty >= min_difficulty && task.difficulty <= max_difficulty
                });
            }

            if let Some((min_reward, max_reward)) = search_params.reward_range {
                all_tasks.retain(|task| task.reward >= min_reward && task.reward <= max_reward);
            }

            if let Some(ref creator) = search_params.creator {
                all_tasks.retain(|task| task.creator == *creator);
            }

            if let Some(ref assignee) = search_params.assignee {
                all_tasks.retain(|task| {
                    if let Some(ref task_assignee) = task.assignee {
                        task_assignee == assignee
                    } else {
                        false
                    }
                });
            }

            if let Some((start_date, end_date)) = search_params.deadline_range {
                all_tasks.retain(|task| {
                    if let Some(deadline) = task.deadline {
                        deadline >= start_date && deadline <= end_date
                    } else {
                        false
                    }
                });
            }

            if let Some((start_time, end_time)) = search_params.created_time_range {
                all_tasks
                    .retain(|task| task.created_at >= start_time && task.created_at <= end_time);
            }

            if search_params.has_deadline {
                all_tasks.retain(|task| task.deadline.is_some());
            }

            if search_params.unassigned_only {
                all_tasks.retain(|task| task.assignee.is_none());
            }

            all_tasks.len() as u32
        }

        /// 快速搜索：仅按关键词搜索标题和描述
        pub fn quick_search(keyword: Vec<u8>) -> Vec<Task<T>> {
            if keyword.is_empty() {
                return Vec::new();
            }

            let keyword_lower = sp_std::str::from_utf8(&keyword)
                .unwrap_or("")
                .to_lowercase();

            Tasks::<T>::iter_values()
                .filter(|task| {
                    let title_match = sp_std::str::from_utf8(&task.title)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    let desc_match = sp_std::str::from_utf8(&task.description)
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&keyword_lower);

                    title_match || desc_match
                })
                .collect()
        }

        /// 获取任务统计信息
        pub fn get_task_statistics() -> TaskStatistics<T> {
            let all_tasks: Vec<Task<T>> = Tasks::<T>::iter_values().collect();

            let total_tasks = all_tasks.len() as u32;
            let pending_count = all_tasks.iter().filter(|t| t.status == 0).count() as u32;
            let in_progress_count = all_tasks.iter().filter(|t| t.status == 1).count() as u32;
            let completed_count = all_tasks.iter().filter(|t| t.status == 2).count() as u32;
            let cancelled_count = all_tasks.iter().filter(|t| t.status == 3).count() as u32;
            let pending_verification_count =
                all_tasks.iter().filter(|t| t.status == 4).count() as u32;

            let total_reward = all_tasks
                .iter()
                .map(|t| t.reward)
                .fold(T::Balance::default(), |acc, reward| acc + reward);
            let avg_difficulty = if total_tasks > 0 {
                all_tasks.iter().map(|t| t.difficulty as u32).sum::<u32>() / total_tasks
            } else {
                0
            };

            let now = Self::current_timestamp();
            let overdue_count = all_tasks
                .iter()
                .filter(|t| {
                    if let Some(deadline) = t.deadline {
                        deadline <= now && t.status != 2 && t.status != 3 // 不是已完成或已取消
                    } else {
                        false
                    }
                })
                .count() as u32;

            TaskStatistics {
                total_tasks,
                pending_count,
                in_progress_count,
                completed_count,
                cancelled_count,
                pending_verification_count,
                total_reward,
                avg_difficulty,
                overdue_count,
            }
        }

        /// 更新任务索引
        fn update_task_indices(task: &Task<T>) -> DispatchResult {
            // 更新状态索引
            TasksByStatus::<T>::mutate(task.status, |tasks| {
                if !tasks.contains(&task.id) {
                    tasks
                        .try_push(task.id)
                        .map_err(|_| Error::<T>::TooManyTasks)
                } else {
                    Ok(())
                }
            })?;

            // 更新优先级索引
            TasksByPriority::<T>::mutate(task.priority, |tasks| {
                if !tasks.contains(&task.id) {
                    tasks
                        .try_push(task.id)
                        .map_err(|_| Error::<T>::TooManyTasks)
                } else {
                    Ok(())
                }
            })?;

            // 更新截止日期索引
            if let Some(deadline) = task.deadline {
                TasksByDeadline::<T>::mutate(deadline, |tasks| {
                    if !tasks.contains(&task.id) {
                        tasks
                            .try_push(task.id)
                            .map_err(|_| Error::<T>::TooManyTasks)
                    } else {
                        Ok(())
                    }
                })?;
            }

            Ok(())
        }

        /// 移除任务索引
        fn remove_task_indices(task: &Task<T>) -> DispatchResult {
            // 从状态索引中移除
            TasksByStatus::<T>::mutate(task.status, |tasks| {
                tasks.retain(|&x| x != task.id);
            });

            // 从优先级索引中移除
            TasksByPriority::<T>::mutate(task.priority, |tasks| {
                tasks.retain(|&x| x != task.id);
            });

            // 从截止日期索引中移除
            if let Some(deadline) = task.deadline {
                TasksByDeadline::<T>::mutate(deadline, |tasks| {
                    tasks.retain(|&x| x != task.id);
                });
            }

            Ok(())
        }

        /// 验证状态转换
        fn validate_status_transition(old_status: &u8, new_status: &u8) -> Result<(), Error<T>> {
            use TaskStatus::*;

            let old = TaskStatus::from(*old_status);
            let new = TaskStatus::from(*new_status);

            match (old, new) {
                // 从 Pending 可以转换到 InProgress, Cancelled
                (Pending, InProgress) | (Pending, Cancelled) => Ok(()),

                // 从 InProgress 可以转换到 Completed, Cancelled, PendingVerification
                (InProgress, Completed)
                | (InProgress, Cancelled)
                | (InProgress, PendingVerification) => Ok(()),

                // 从 PendingVerification 可以转换到 Completed, InProgress
                (PendingVerification, Completed) | (PendingVerification, InProgress) => Ok(()),

                // 同状态转换允许
                _ if old_status == new_status => Ok(()),

                // 其他转换都是无效的
                _ => Err(Error::<T>::InvalidStatusTransition),
            }
        }

        /// 验证截止日期
        ///
        /// 确保截止日期在未来，防止创建已过期的任务
        fn validate_deadline(deadline: Option<T::Moment>) -> DispatchResult {
            if let Some(deadline) = deadline {
                let now = Self::current_timestamp();
                ensure!(deadline > now, Error::<T>::TaskExpired);
            }
            Ok(())
        }

        /// 检查任务是否已过期
        ///
        /// 用于在操作任务前检查是否过期
        fn check_task_expired(task: &Task<T>) -> Result<(), Error<T>> {
            if let Some(deadline) = task.deadline {
                let now = Self::current_timestamp();
                ensure!(deadline > now, Error::<T>::TaskExpired);
            }
            Ok(())
        }

        /// 处理任务状态变更时的奖励发放和释放
        fn handle_reward_on_status_change(
            task: &Task<T>,
            _old_status: u8,
            new_status: u8,
        ) -> DispatchResult {
            // 任务完成时发放奖励
            if new_status == 2 {
                // Completed
                if let Some(assignee) = &task.assignee {
                    // 释放创建者的预留资金
                    T::Currency::unreserve(&task.creator, task.reward);

                    // 转账给执行者
                    T::Currency::transfer(
                        &task.creator,
                        assignee,
                        task.reward,
                        frame_support::traits::ExistenceRequirement::KeepAlive,
                    )
                    .map_err(|_| Error::<T>::RewardTransferFailed)?;

                    // 发出奖励发放事件
                    Self::deposit_event(Event::TaskRewardPaid {
                        task_id: task.id,
                        assignee: assignee.clone(),
                        creator: task.creator.clone(),
                        reward: task.reward,
                    });
                }
            }
            // 任务取消时释放预留的奖励
            else if new_status == 3 {
                // Cancelled
                // 释放创建者的预留资金
                T::Currency::unreserve(&task.creator, task.reward);

                // 发出奖励释放事件
                Self::deposit_event(Event::TaskRewardReleased {
                    task_id: task.id,
                    creator: task.creator.clone(),
                    reward: task.reward,
                });
            }

            Ok(())
        }
    }

    // 社区验证辅助函数
    impl<T: Config> Pallet<T> {
        /// 启动社区验证
        fn start_community_verification(task_id: u32) -> DispatchResult {
            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = current_block + T::VerificationPeriod::get();

            // 初始化验证状态
            VerificationStatus::<T>::insert(task_id, (end_block, 0u32, 0u32));

            // 发出事件
            Self::deposit_event(Event::CommunityVerificationStarted { task_id, end_block });

            Ok(())
        }

        /// 尝试完成验证（当达到最小投票数时自动触发）
        fn try_complete_verification(task_id: u32) -> DispatchResult {
            let (end_block, approve_votes, reject_votes) = VerificationStatus::<T>::get(task_id)
                .ok_or(Error::<T>::TaskNotPendingVerification)?;

            let current_block = <frame_system::Pallet<T>>::block_number();
            let total_votes = approve_votes + reject_votes;

            // 只有在达到最小投票数时才自动完成
            if total_votes >= T::MinVerificationVotes::get() {
                // 检查是否在投票期间内
                if current_block <= end_block {
                    Self::finalize_verification(task_id, approve_votes, reject_votes)?;
                }
            }

            Ok(())
        }

        /// 完成验证并更新任务状态
        fn finalize_verification(
            task_id: u32,
            approve_votes: u32,
            reject_votes: u32,
        ) -> DispatchResult {
            let total_votes = approve_votes + reject_votes;

            // 检查是否有足够的投票
            ensure!(
                total_votes >= T::MinVerificationVotes::get(),
                Error::<T>::InsufficientVerificationVotes
            );

            // 计算通过率
            let approval_percentage = (approve_votes * 100) / total_votes;
            let approved = approval_percentage >= T::MinApprovalPercentage::get();

            // 获取任务并更新状态
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            let new_status = if approved {
                2u8 // Completed
            } else {
                1u8 // InProgress - 回到进行中状态，需要重新提交
            };

            let old_status = task.status;
            task.status = new_status;
            task.updated_at = Self::current_timestamp();

            // 存储更新后的任务
            Tasks::<T>::insert(task_id, &task);

            // 更新任务索引
            Self::update_task_indices(&task)?;

            // 如果验证通过，处理奖励发放
            if approved {
                Self::handle_reward_on_status_change(&task, old_status, new_status)?;
            }

            // 清理验证相关数据
            Self::cleanup_verification_data(task_id)?;

            // 发出事件
            Self::deposit_event(Event::CommunityVerificationCompleted {
                task_id,
                approved,
                approve_votes,
                reject_votes,
            });

            Self::deposit_event(Event::TaskStatusChanged {
                task_id,
                old_status,
                new_status,
                assignee: task.assignee.clone(),
            });

            Ok(())
        }

        /// 处理验证过期
        fn handle_verification_expired(task_id: u32) -> DispatchResult {
            // 获取任务并回退到进行中状态
            let mut task = Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;

            let old_status = task.status;
            task.status = 1u8; // InProgress
            task.updated_at = Self::current_timestamp();

            // 存储更新后的任务
            Tasks::<T>::insert(task_id, &task);

            // 更新任务索引
            Self::update_task_indices(&task)?;

            // 清理验证相关数据
            Self::cleanup_verification_data(task_id)?;

            // 发出事件
            Self::deposit_event(Event::CommunityVerificationExpired { task_id });

            Self::deposit_event(Event::TaskStatusChanged {
                task_id,
                old_status,
                new_status: task.status,
                assignee: task.assignee.clone(),
            });

            Ok(())
        }

        /// 清理验证相关数据
        fn cleanup_verification_data(task_id: u32) -> DispatchResult {
            // 删除验证状态
            VerificationStatus::<T>::remove(task_id);

            // 清理投票记录
            let voters = VerificationVoters::<T>::get(task_id);
            for voter in voters.iter() {
                VerificationVotes::<T>::remove(task_id, voter);
            }

            // 清理投票者列表
            VerificationVoters::<T>::remove(task_id);

            Ok(())
        }

        /// 获取任务的验证状态
        pub fn get_verification_status(task_id: u32) -> Option<(BlockNumberFor<T>, u32, u32)> {
            VerificationStatus::<T>::get(task_id)
        }

        /// 获取任务的投票者列表
        pub fn get_verification_voters(task_id: u32) -> Vec<T::AccountId> {
            VerificationVoters::<T>::get(task_id).into_inner()
        }

        /// 检查用户是否已为任务投票
        pub fn has_voted(task_id: u32, voter: &T::AccountId) -> bool {
            VerificationVotes::<T>::contains_key(task_id, voter)
        }

        /// 获取用户对任务的投票
        pub fn get_vote(task_id: u32, voter: &T::AccountId) -> Option<bool> {
            VerificationVotes::<T>::get(task_id, voter)
        }
    }
}
