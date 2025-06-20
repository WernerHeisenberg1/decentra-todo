use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

/// 任务状态枚举
#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
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

impl From<TaskStatus> for u8 {
    fn from(status: TaskStatus) -> u8 {
        match status {
            TaskStatus::Pending => 0,
            TaskStatus::InProgress => 1,
            TaskStatus::Completed => 2,
            TaskStatus::Cancelled => 3,
            TaskStatus::PendingVerification => 4,
        }
    }
}

impl From<u8> for TaskStatus {
    fn from(value: u8) -> Self {
        match value {
            0 => TaskStatus::Pending,
            1 => TaskStatus::InProgress,
            2 => TaskStatus::Completed,
            3 => TaskStatus::Cancelled,
            4 => TaskStatus::PendingVerification,
            _ => TaskStatus::Pending, // 默认值
        }
    }
}

/// 任务优先级
#[derive(
    Encode,
    Decode,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    RuntimeDebug,
    TypeInfo,
    MaxEncodedLen,
)]
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

impl From<Priority> for u8 {
    fn from(priority: Priority) -> u8 {
        match priority {
            Priority::Low => 1,
            Priority::Medium => 2,
            Priority::High => 3,
            Priority::Urgent => 4,
        }
    }
}

impl From<u8> for Priority {
    fn from(value: u8) -> Self {
        match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            4 => Priority::Urgent,
            _ => Priority::Medium, // 默认值
        }
    }
}

/// 任务结构体
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct Task<T: crate::pallet::Config> {
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
    pub status: u8,
    /// 优先级
    pub priority: u8,
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

impl<T: crate::pallet::Config> Task<T> {
    /// 创建新任务
    pub fn new(
        id: u32,
        creator: T::AccountId,
        title: BoundedVec<u8, T::MaxTitleLength>,
        description: BoundedVec<u8, T::MaxDescriptionLength>,
        priority: u8,
        difficulty: u8,
        reward: T::Balance,
        created_at: T::Moment,
        deadline: Option<T::Moment>,
    ) -> Self {
        Self {
            id,
            creator,
            assignee: None,
            title,
            description,
            status: 0, // Pending
            priority,
            difficulty,
            reward,
            created_at,
            updated_at: created_at,
            deadline,
        }
    }

    /// 检查任务是否可以被指定用户修改
    pub fn can_be_modified_by(&self, account: &T::AccountId) -> bool {
        self.creator == *account
    }

    /// 检查任务是否可以被指定用户操作（修改或状态变更）
    pub fn can_be_operated_by(&self, account: &T::AccountId) -> bool {
        self.creator == *account || self.assignee.as_ref() == Some(account)
    }

    /// 检查任务是否已分配
    pub fn is_assigned(&self) -> bool {
        self.assignee.is_some()
    }

    /// 检查任务是否处于终态
    pub fn is_final_state(&self) -> bool {
        self.status == 2 || self.status == 3 // Completed or Cancelled
    }

    /// 检查任务是否过期
    pub fn is_expired(&self, current_time: T::Moment) -> bool {
        if let Some(deadline) = self.deadline {
            deadline <= current_time
        } else {
            false
        }
    }

    /// 检查任务是否即将过期（在指定时间内）
    pub fn is_expiring_soon(&self, current_time: T::Moment, threshold: T::Moment) -> bool {
        if let Some(deadline) = self.deadline {
            deadline > current_time && deadline <= current_time + threshold
        } else {
            false
        }
    }

    /// 获取任务状态的字符串表示
    pub fn status_string(&self) -> &'static str {
        match TaskStatus::from(self.status) {
            TaskStatus::Pending => "Pending",
            TaskStatus::InProgress => "InProgress",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
            TaskStatus::PendingVerification => "PendingVerification",
        }
    }

    /// 获取优先级的字符串表示
    pub fn priority_string(&self) -> &'static str {
        match Priority::from(self.priority) {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
            Priority::Urgent => "Urgent",
        }
    }

    /// 检查任务是否可以开始执行
    pub fn can_start(&self) -> bool {
        self.status == 0 && self.assignee.is_some() // Pending and assigned
    }

    /// 检查任务是否可以完成
    pub fn can_complete(&self) -> bool {
        self.status == 1 // InProgress
    }

    /// 检查任务是否需要验证
    pub fn needs_verification(&self) -> bool {
        self.status == 4 // PendingVerification
    }
}

/// 任务搜索参数
#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct TaskSearchParams<T: crate::pallet::Config> {
    /// 关键词搜索（标题和描述）
    pub keyword: Vec<u8>,
    /// 状态筛选
    pub status: Option<u8>,
    /// 优先级筛选
    pub priority: Option<u8>,
    /// 难度范围筛选 (min, max)
    pub difficulty_range: Option<(u8, u8)>,
    /// 奖励范围筛选 (min, max)
    pub reward_range: Option<(T::Balance, T::Balance)>,
    /// 创建者筛选
    pub creator: Option<T::AccountId>,
    /// 执行者筛选
    pub assignee: Option<T::AccountId>,
    /// 截止日期范围筛选 (start, end)
    pub deadline_range: Option<(T::Moment, T::Moment)>,
    /// 创建时间范围筛选 (start, end)
    pub created_time_range: Option<(T::Moment, T::Moment)>,
    /// 只显示有截止日期的任务
    pub has_deadline: bool,
    /// 只显示未分配的任务
    pub unassigned_only: bool,
    /// 排序方式
    pub sort_by: TaskSortBy,
    /// 是否降序排列
    pub sort_desc: bool,
    /// 页码（从0开始）
    pub page: u32,
    /// 每页大小
    pub page_size: u32,
}

impl<T: crate::pallet::Config> Default for TaskSearchParams<T> {
    fn default() -> Self {
        Self {
            keyword: Vec::new(),
            status: None,
            priority: None,
            difficulty_range: None,
            reward_range: None,
            creator: None,
            assignee: None,
            deadline_range: None,
            created_time_range: None,
            has_deadline: false,
            unassigned_only: false,
            sort_by: TaskSortBy::CreatedAt,
            sort_desc: true,
            page: 0,
            page_size: 10,
        }
    }
}

/// 任务排序方式
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum TaskSortBy {
    /// 按创建时间排序
    CreatedAt,
    /// 按更新时间排序
    UpdatedAt,
    /// 按截止时间排序
    Deadline,
    /// 按奖励金额排序
    Reward,
    /// 按难度排序
    Difficulty,
    /// 按优先级排序
    Priority,
}

impl Default for TaskSortBy {
    fn default() -> Self {
        TaskSortBy::CreatedAt
    }
}

/// 任务统计信息
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct TaskStatistics<T: crate::pallet::Config> {
    /// 总任务数
    pub total_tasks: u32,
    /// 待处理任务数
    pub pending_count: u32,
    /// 进行中任务数
    pub in_progress_count: u32,
    /// 已完成任务数
    pub completed_count: u32,
    /// 已取消任务数
    pub cancelled_count: u32,
    /// 待验证任务数
    pub pending_verification_count: u32,
    /// 总奖励金额
    pub total_reward: T::Balance,
    /// 平均难度
    pub avg_difficulty: u32,
    /// 逾期任务数
    pub overdue_count: u32,
}
