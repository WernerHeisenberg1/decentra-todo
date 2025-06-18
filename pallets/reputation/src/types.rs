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

    /// 获取等级所需的最低分数
    pub fn min_score(&self) -> u32 {
        match self {
            ReputationLevel::Newcomer => 0,
            ReputationLevel::Apprentice => 101,
            ReputationLevel::Skilled => 301,
            ReputationLevel::Expert => 601,
            ReputationLevel::Master => 1001,
            ReputationLevel::Legendary => 1501,
        }
    }

    /// 获取等级的最高分数
    pub fn max_score(&self) -> Option<u32> {
        match self {
            ReputationLevel::Newcomer => Some(100),
            ReputationLevel::Apprentice => Some(300),
            ReputationLevel::Skilled => Some(600),
            ReputationLevel::Expert => Some(1000),
            ReputationLevel::Master => Some(1500),
            ReputationLevel::Legendary => None, // 无上限
        }
    }

    /// 获取下一个等级
    pub fn next_level(&self) -> Option<ReputationLevel> {
        match self {
            ReputationLevel::Newcomer => Some(ReputationLevel::Apprentice),
            ReputationLevel::Apprentice => Some(ReputationLevel::Skilled),
            ReputationLevel::Skilled => Some(ReputationLevel::Expert),
            ReputationLevel::Expert => Some(ReputationLevel::Master),
            ReputationLevel::Master => Some(ReputationLevel::Legendary),
            ReputationLevel::Legendary => None, // 已经是最高等级
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

impl TaskRating {
    /// 获取评分名称
    pub fn name(&self) -> &'static str {
        match self {
            TaskRating::Poor => "差评",
            TaskRating::Fair => "一般",
            TaskRating::Good => "良好",
            TaskRating::Excellent => "优秀",
            TaskRating::Perfect => "完美",
        }
    }

    /// 获取评分的奖励乘数（百分比）
    pub fn reward_multiplier(&self) -> u32 {
        match self {
            TaskRating::Poor => 50,       // 50% 的基础分数
            TaskRating::Fair => 75,       // 75% 的基础分数
            TaskRating::Good => 100,      // 100% 的基础分数
            TaskRating::Excellent => 125, // 125% 的基础分数
            TaskRating::Perfect => 150,   // 150% 的基础分数
        }
    }

    /// 检查是否为正面评价
    pub fn is_positive(&self) -> bool {
        *self >= TaskRating::Good
    }

    /// 检查是否为负面评价
    pub fn is_negative(&self) -> bool {
        *self <= TaskRating::Fair
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

    /// 检查用户是否为新手
    pub fn is_newcomer(&self) -> bool {
        matches!(self.level, ReputationLevel::Newcomer)
    }

    /// 检查用户是否为专家级别或以上
    pub fn is_expert_or_above(&self) -> bool {
        self.level >= ReputationLevel::Expert
    }

    /// 获取距离下一等级还需要的分数
    pub fn points_to_next_level(&self) -> Option<u32> {
        if let Some(next_level) = self.level.next_level() {
            Some(next_level.min_score().saturating_sub(self.total_score))
        } else {
            None // 已经是最高等级
        }
    }

    /// 获取用户在当前等级的进度百分比
    pub fn level_progress(&self) -> f32 {
        let current_min = self.level.min_score();

        if let Some(current_max) = self.level.max_score() {
            let range = current_max - current_min;
            let progress = self.total_score.saturating_sub(current_min);

            if range == 0 {
                100.0
            } else {
                (progress as f32 / range as f32) * 100.0
            }
        } else {
            // 传奇等级没有上限
            100.0
        }
    }

    /// 获取声誉等级描述
    pub fn level_description(&self) -> &'static str {
        match self.level {
            ReputationLevel::Newcomer => "刚开始在平台上活跃的新用户",
            ReputationLevel::Apprentice => "正在学习和提升技能的用户",
            ReputationLevel::Skilled => "具有一定经验和技能的用户",
            ReputationLevel::Expert => "在某些领域具有专业知识的用户",
            ReputationLevel::Master => "拥有丰富经验和高质量表现的用户",
            ReputationLevel::Legendary => "平台上最顶级的杰出用户",
        }
    }

    /// 检查用户是否有良好的任务完成记录
    pub fn has_good_track_record(&self) -> bool {
        self.completion_rate() >= 0.8 && self.average_rating() >= 3.5
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

impl<T: crate::pallet::Config> TaskEvaluation<T> {
    /// 创建新的任务评价
    pub fn new(
        task_id: u32,
        assignee: T::AccountId,
        evaluator: T::AccountId,
        rating: TaskRating,
        evaluated_at: T::Moment,
        comment: BoundedVec<u8, T::MaxCommentLength>,
    ) -> Self {
        Self {
            task_id,
            assignee,
            evaluator,
            rating,
            evaluated_at,
            comment,
        }
    }

    /// 检查评价是否为正面评价
    pub fn is_positive(&self) -> bool {
        self.rating.is_positive()
    }

    /// 检查评价是否为负面评价
    pub fn is_negative(&self) -> bool {
        self.rating.is_negative()
    }

    /// 获取评价的分数值
    pub fn score(&self) -> u8 {
        self.rating as u8
    }

    /// 获取评价备注的字符串形式
    pub fn comment_string(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.comment)
    }

    /// 检查评价是否有备注
    pub fn has_comment(&self) -> bool {
        !self.comment.is_empty()
    }
}
