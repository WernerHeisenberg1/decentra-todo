use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;

/// 声誉等级枚举
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
pub enum ReputationLevel {
    /// 新手 (0-100)
    Newcomer,
    /// 学徒 (101-300)
    Apprentice,
    /// 熟练 (301-600)
    Skilled,
    /// 专家 (601-1000)
    Expert,
    /// 大师 (1001-1500)
    Master,
    /// 传奇 (1501+)
    Legendary,
}

impl Default for ReputationLevel {
    fn default() -> Self {
        ReputationLevel::Newcomer
    }
}

impl ReputationLevel {
    /// 根据声誉分数计算等级
    pub fn from_score(score: u32) -> Self {
        match score {
            0..=100 => ReputationLevel::Newcomer,
            101..=300 => ReputationLevel::Apprentice,
            301..=600 => ReputationLevel::Skilled,
            601..=1000 => ReputationLevel::Expert,
            1001..=1500 => ReputationLevel::Master,
            _ => ReputationLevel::Legendary,
        }
    }

    /// 获取等级名称
    pub fn name(&self) -> &'static str {
        match self {
            ReputationLevel::Newcomer => "新手",
            ReputationLevel::Apprentice => "学徒",
            ReputationLevel::Skilled => "熟练",
            ReputationLevel::Expert => "专家",
            ReputationLevel::Master => "大师",
            ReputationLevel::Legendary => "传奇",
        }
    }
}

/// 任务评分枚举
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
pub enum TaskRating {
    /// 差评 (1分)
    Poor = 1,
    /// 一般 (2分)
    Fair = 2,
    /// 良好 (3分)
    Good = 3,
    /// 优秀 (4分)
    Excellent = 4,
    /// 完美 (5分)
    Perfect = 5,
}

impl Default for TaskRating {
    fn default() -> Self {
        TaskRating::Good
    }
}

impl From<TaskRating> for u8 {
    fn from(rating: TaskRating) -> u8 {
        rating as u8
    }
}

impl TryFrom<u8> for TaskRating {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(TaskRating::Poor),
            2 => Ok(TaskRating::Fair),
            3 => Ok(TaskRating::Good),
            4 => Ok(TaskRating::Excellent),
            5 => Ok(TaskRating::Perfect),
            _ => Err(()),
        }
    }
}

/// 用户声誉信息
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct UserReputation<T: crate::pallet::Config> {
    /// 总声誉分数
    pub total_score: u32,
    /// 声誉等级
    pub level: ReputationLevel,
    /// 完成的任务总数
    pub completed_tasks: u32,
    /// 取消的任务总数
    pub cancelled_tasks: u32,
    /// 获得的评分总数
    pub total_ratings: u32,
    /// 评分总和（用于计算平均分）
    pub rating_sum: u32,
    /// 最后更新时间
    pub last_updated: T::Moment,
}

impl<T: crate::pallet::Config> Default for UserReputation<T> {
    fn default() -> Self {
        Self {
            total_score: 0,
            level: ReputationLevel::Newcomer,
            completed_tasks: 0,
            cancelled_tasks: 0,
            total_ratings: 0,
            rating_sum: 0,
            last_updated: T::Moment::zero(),
        }
    }
}

impl<T: crate::pallet::Config> UserReputation<T> {
    /// 计算平均评分
    pub fn average_rating(&self) -> f32 {
        if self.total_ratings == 0 {
            0.0
        } else {
            self.rating_sum as f32 / self.total_ratings as f32
        }
    }

    /// 计算完成率
    pub fn completion_rate(&self) -> f32 {
        let total_tasks = self.completed_tasks + self.cancelled_tasks;
        if total_tasks == 0 {
            0.0
        } else {
            self.completed_tasks as f32 / total_tasks as f32
        }
    }

    /// 更新声誉分数和等级
    pub fn update_score(&mut self, new_score: u32) {
        self.total_score = new_score;
        self.level = ReputationLevel::from_score(new_score);
    }
}

/// 任务评价记录
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
pub struct TaskEvaluation<T: crate::pallet::Config> {
    /// 任务ID
    pub task_id: u32,
    /// 执行者
    pub assignee: T::AccountId,
    /// 评价者
    pub evaluator: T::AccountId,
    /// 评分
    pub rating: TaskRating,
    /// 评价时间
    pub evaluated_at: T::Moment,
    /// 评价备注
    pub comment: BoundedVec<u8, T::MaxCommentLength>,
}
