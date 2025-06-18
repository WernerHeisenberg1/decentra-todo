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
    use super::types::{Task, TaskStatus};
    use codec::MaxEncodedLen;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Get, Randomness},
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
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

        /// The moment type for timestamps (using BlockNumber for simplicity)
        type Moment: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

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
                creator,
                title: bounded_title.to_vec(),
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

            if let Some(reward) = reward {
                task.reward = reward;
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
    }
}
