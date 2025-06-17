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

#[frame_support::pallet]
pub mod pallet {
    use codec::{Decode, Encode};
    use frame_support::{
        dispatch::{DispatchError, DispatchResult},
        pallet_prelude::*,
        traits::{Get, Randomness},
    };
    use frame_system::pallet_prelude::*;
    use scale_info::TypeInfo;
    use sp_runtime::traits::{Saturating, Zero};
    use sp_std::vec::Vec;

    /// 任务状态枚举
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum TaskStatus {
        /// 待处理
        Pending,
        /// 进行中
        InProgress,
        /// 已完成
        Completed,
        /// 已取消
        Cancelled,
        /// 需要验证
        PendingVerification,
    }

    impl Default for TaskStatus {
        fn default() -> Self {
            TaskStatus::Pending
        }
    }

    /// 任务优先级
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum Priority {
        Low = 1,
        Medium = 2,
        High = 3,
        Urgent = 4,
    }

    impl Default for Priority {
        fn default() -> Self {
            Priority::Medium
        }
    }

    /// 任务结构体
    #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct Task<T: Config> {
        /// 任务ID
        pub id: u32,
        /// 创建者
        pub creator: T::AccountId,
        /// 分配给的执行者（可选）
        pub assignee: Option<T::AccountId>,
        /// 任务标题
        pub title: BoundedVec<u8, T::MaxTitleLength>,
        /// 任务描述
        pub description: BoundedVec<u8, T::MaxDescriptionLength>,
        /// 任务状态
        pub status: TaskStatus,
        /// 优先级
        pub priority: Priority,
        /// 难度等级 (1-10)
        pub difficulty: u8,
        /// 预计奖励
        pub reward: T::Balance,
        /// 创建时间戳
        pub created_at: T::Moment,
        /// 更新时间戳
        pub updated_at: T::Moment,
        /// 截止时间（可选）
        pub deadline: Option<T::Moment>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The balance type
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy + MaxEncodedLen;

        /// The moment type for timestamps
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

    /// 任务总数统计
    #[pallet::storage]
    #[pallet::getter(fn task_count_by_status)]
    pub type TaskCountByStatus<T> = StorageMap<_, Blake2_128Concat, TaskStatus, u32, ValueQuery>;

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
            old_status: TaskStatus,
            new_status: TaskStatus,
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
    }

    // Dispatchable functions allow users to interact with the pallet and invoke state changes.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建新任务
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_task(
            origin: OriginFor<T>,
            title: Vec<u8>,
            description: Vec<u8>,
            priority: Priority,
            difficulty: u8,
            reward: T::Balance,
            deadline: Option<T::Moment>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 验证输入参数
            ensure!(
                title.len() <= T::MaxTitleLength::get() as usize,
                Error::<T>::TitleTooLong
            );
            ensure!(
                description.len() <= T::MaxDescriptionLength::get() as usize,
                Error::<T>::DescriptionTooLong
            );
            ensure!(
                difficulty >= 1 && difficulty <= 10,
                Error::<T>::InvalidDifficulty
            );

            // 检查用户任务数量限制
            let user_tasks = UserCreatedTasks::<T>::get(&who);
            ensure!(
                user_tasks.len() < T::MaxTasksPerUser::get() as usize,
                Error::<T>::TooManyTasks
            );

            // 生成任务ID
            let task_id = NextTaskId::<T>::get();
            NextTaskId::<T>::put(task_id.saturating_add(1));

            // 获取当前时间戳（这里简化处理，实际应该从时间戳 pallet 获取）
            let now = T::Moment::zero(); // 实际应该替换为真实时间戳

            // 创建任务
            let task = Task {
                id: task_id,
                creator: who.clone(),
                assignee: None,
                title: title
                    .clone()
                    .try_into()
                    .map_err(|_| Error::<T>::TitleTooLong)?,
                description: description
                    .try_into()
                    .map_err(|_| Error::<T>::DescriptionTooLong)?,
                status: TaskStatus::Pending,
                priority,
                difficulty,
                reward,
                created_at: now,
                updated_at: now,
                deadline,
            };

            // 存储任务
            Tasks::<T>::insert(&task_id, &task);

            // 更新用户创建的任务列表
            UserCreatedTasks::<T>::try_mutate(&who, |tasks| {
                tasks
                    .try_push(task_id)
                    .map_err(|_| Error::<T>::TooManyTasks)
            })?;

            // 更新状态统计
            TaskCountByStatus::<T>::mutate(TaskStatus::Pending, |count| {
                *count = count.saturating_add(1);
            });

            // 触发事件
            Self::deposit_event(Event::TaskCreated {
                task_id,
                creator: who,
                title,
            });

            Ok(())
        }

        /// 更新任务信息
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn update_task(
            origin: OriginFor<T>,
            task_id: u32,
            title: Option<Vec<u8>>,
            description: Option<Vec<u8>>,
            priority: Option<Priority>,
            difficulty: Option<u8>,
            reward: Option<T::Balance>,
            deadline: Option<Option<T::Moment>>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 权限检查：只有创建者可以更新任务
            ensure!(task.creator == who, Error::<T>::NotAuthorized);

            // 更新字段
            if let Some(new_title) = title {
                ensure!(
                    new_title.len() <= T::MaxTitleLength::get() as usize,
                    Error::<T>::TitleTooLong
                );
                task.title = new_title.try_into().map_err(|_| Error::<T>::TitleTooLong)?;
            }

            if let Some(new_description) = description {
                ensure!(
                    new_description.len() <= T::MaxDescriptionLength::get() as usize,
                    Error::<T>::DescriptionTooLong
                );
                task.description = new_description
                    .try_into()
                    .map_err(|_| Error::<T>::DescriptionTooLong)?;
            }

            if let Some(new_priority) = priority {
                task.priority = new_priority;
            }

            if let Some(new_difficulty) = difficulty {
                ensure!(
                    new_difficulty >= 1 && new_difficulty <= 10,
                    Error::<T>::InvalidDifficulty
                );
                task.difficulty = new_difficulty;
            }

            if let Some(new_reward) = reward {
                task.reward = new_reward;
            }

            if let Some(new_deadline) = deadline {
                task.deadline = new_deadline;
            }

            // 更新时间戳
            task.updated_at = T::Moment::zero(); // 实际应该替换为真实时间戳

            // 保存任务
            Tasks::<T>::insert(&task_id, &task);

            // 触发事件
            Self::deposit_event(Event::TaskUpdated {
                task_id,
                updater: who,
            });

            Ok(())
        }

        /// 更改任务状态
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn change_task_status(
            origin: OriginFor<T>,
            task_id: u32,
            new_status: TaskStatus,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 权限检查：创建者或分配者可以更改状态
            ensure!(
                task.creator == who || task.assignee == Some(who.clone()),
                Error::<T>::NotAuthorized
            );

            let old_status = task.status.clone();

            // 验证状态转换的合法性
            Self::validate_status_transition(&old_status, &new_status)?;

            // 更新统计
            TaskCountByStatus::<T>::mutate(&old_status, |count| {
                *count = count.saturating_sub(1);
            });
            TaskCountByStatus::<T>::mutate(&new_status, |count| {
                *count = count.saturating_add(1);
            });

            // 更新任务状态和时间戳
            task.status = new_status.clone();
            task.updated_at = T::Moment::zero(); // 实际应该替换为真实时间戳

            // 保存任务
            Tasks::<T>::insert(&task_id, &task);

            // 触发事件
            Self::deposit_event(Event::TaskStatusChanged {
                task_id,
                old_status,
                new_status,
            });

            Ok(())
        }

        /// 分配任务给执行者
        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn assign_task(
            origin: OriginFor<T>,
            task_id: u32,
            assignee: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 权限检查：只有创建者可以分配任务
            ensure!(task.creator == who, Error::<T>::NotAuthorized);

            // 不能分配给自己
            ensure!(task.creator != assignee, Error::<T>::CannotAssignToSelf);

            // 检查任务是否已经分配
            ensure!(task.assignee.is_none(), Error::<T>::TaskAlreadyAssigned);

            // 检查分配者的任务数量限制
            let assignee_tasks = UserAssignedTasks::<T>::get(&assignee);
            ensure!(
                assignee_tasks.len() < T::MaxTasksPerUser::get() as usize,
                Error::<T>::TooManyTasks
            );

            // 分配任务
            task.assignee = Some(assignee.clone());
            task.updated_at = T::Moment::zero(); // 实际应该替换为真实时间戳

            // 保存任务
            Tasks::<T>::insert(&task_id, &task);

            // 更新分配者的任务列表
            UserAssignedTasks::<T>::try_mutate(&assignee, |tasks| {
                tasks
                    .try_push(task_id)
                    .map_err(|_| Error::<T>::TooManyTasks)
            })?;

            // 触发事件
            Self::deposit_event(Event::TaskAssigned { task_id, assignee });

            Ok(())
        }

        /// 取消任务分配
        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn unassign_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let mut task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 权限检查：只有创建者可以取消分配
            ensure!(task.creator == who, Error::<T>::NotAuthorized);

            // 检查任务是否已分配
            let previous_assignee = task.assignee.take().ok_or(Error::<T>::TaskNotAssigned)?;

            // 更新时间戳
            task.updated_at = T::Moment::zero(); // 实际应该替换为真实时间戳

            // 保存任务
            Tasks::<T>::insert(&task_id, &task);

            // 从分配者的任务列表中移除
            UserAssignedTasks::<T>::mutate(&previous_assignee, |tasks| {
                tasks.retain(|&id| id != task_id);
            });

            // 触发事件
            Self::deposit_event(Event::TaskUnassigned {
                task_id,
                previous_assignee,
            });

            Ok(())
        }

        /// 删除任务
        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn delete_task(origin: OriginFor<T>, task_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // 获取任务
            let task = Tasks::<T>::get(&task_id).ok_or(Error::<T>::TaskNotFound)?;

            // 权限检查：只有创建者可以删除任务
            ensure!(task.creator == who, Error::<T>::NotAuthorized);

            // 从存储中删除任务
            Tasks::<T>::remove(&task_id);

            // 更新统计
            TaskCountByStatus::<T>::mutate(&task.status, |count| {
                *count = count.saturating_sub(1);
            });

            // 从创建者的任务列表中移除
            UserCreatedTasks::<T>::mutate(&task.creator, |tasks| {
                tasks.retain(|&id| id != task_id);
            });

            // 如果有分配者，也从分配者的任务列表中移除
            if let Some(assignee) = &task.assignee {
                UserAssignedTasks::<T>::mutate(assignee, |tasks| {
                    tasks.retain(|&id| id != task_id);
                });
            }

            // 触发事件
            Self::deposit_event(Event::TaskDeleted {
                task_id,
                deleted_by: who,
            });

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// 验证任务状态转换的合法性
        fn validate_status_transition(
            old_status: &TaskStatus,
            new_status: &TaskStatus,
        ) -> Result<(), Error<T>> {
            use TaskStatus::*;

            let valid = match (old_status, new_status) {
                // 从 Pending 可以转换到任何状态
                (Pending, _) => true,
                // 从 InProgress 可以转换到 Completed, Cancelled, PendingVerification
                (InProgress, Completed) => true,
                (InProgress, Cancelled) => true,
                (InProgress, PendingVerification) => true,
                // 从 PendingVerification 可以转换到 Completed 或 InProgress
                (PendingVerification, Completed) => true,
                (PendingVerification, InProgress) => true,
                // Completed 和 Cancelled 是终态，不能再转换
                (Completed, _) => false,
                (Cancelled, _) => false,
                // 其他转换都不允许
                _ => false,
            };

            if valid {
                Ok(())
            } else {
                Err(Error::<T>::InvalidStatusTransition)
            }
        }

        /// 获取任务统计信息
        pub fn get_task_statistics() -> (u32, u32, u32, u32, u32) {
            (
                TaskCountByStatus::<T>::get(TaskStatus::Pending),
                TaskCountByStatus::<T>::get(TaskStatus::InProgress),
                TaskCountByStatus::<T>::get(TaskStatus::Completed),
                TaskCountByStatus::<T>::get(TaskStatus::Cancelled),
                TaskCountByStatus::<T>::get(TaskStatus::PendingVerification),
            )
        }
    }
}
