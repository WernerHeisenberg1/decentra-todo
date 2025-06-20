use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

/// 成就类型
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum AchievementType {
    /// 任务完成相关成就
    TaskCompletion,
    /// 声誉相关成就  
    Reputation,
    /// 社区贡献相关成就
    Community,
    /// 特殊事件成就
    Special,
}

/// 成就稀有度
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum AchievementRarity {
    /// 普通 - 完成基础任务即可获得
    Common,
    /// 稀有 - 需要达到一定要求
    Rare,
    /// 史诗 - 需要达到较高要求
    Epic,
    /// 传说 - 需要达到极高要求
    Legendary,
}

/// 成就条件
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum AchievementCondition {
    /// 完成指定数量的任务
    CompleteTasksCount(u32),
    /// 达到指定声誉分数
    ReachReputationScore(u32),
    /// 连续完成任务（不能有取消或失败）
    ConsecutiveTaskCompletion(u32),
    /// 获得指定平均评分
    AverageRating(u8), // 1-5分，用u8*10表示，如45表示4.5分
    /// 参与社区验证次数
    CommunityVerificationCount(u32),
    /// 创建指定数量的任务
    CreateTasksCount(u32),
    /// 在指定时间内完成任务
    CompleteTaskInTime(u32), // 以小时为单位
}

/// 成就定义
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct Achievement<BoundedString> {
    /// 成就ID
    pub id: u32,
    /// 成就名称
    pub name: BoundedString,
    /// 成就描述
    pub description: BoundedString,
    /// 成就类型
    pub achievement_type: AchievementType,
    /// 成就稀有度
    pub rarity: AchievementRarity,
    /// 达成条件
    pub condition: AchievementCondition,
    /// 奖励NFT的collection ID（简化为u32）
    pub collection_id: u32,
    /// NFT元数据URI
    pub metadata_uri: BoundedString,
    /// 是否激活
    pub is_active: bool,
}

/// 用户成就记录
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct UserAchievement<AccountId, Moment> {
    /// 成就ID
    pub achievement_id: u32,
    /// 获得时间
    pub earned_at: Moment,
    /// NFT项目ID（简化为u32）
    pub nft_item_id: Option<u32>,
    /// 铸造给的账户
    pub owner: AccountId,
}

/// 成就进度
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct AchievementProgress {
    /// 成就ID
    pub achievement_id: u32,
    /// 当前进度
    pub current_progress: u32,
    /// 目标进度
    pub target_progress: u32,
    /// 完成百分比
    pub completion_percentage: u8,
}

/// 用户统计数据（用于成就检测）
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen, Default)]
pub struct UserStats<Moment> {
    /// 完成的任务总数
    pub tasks_completed: u32,
    /// 创建的任务总数
    pub tasks_created: u32,
    /// 连续完成任务数（当前连击）
    pub consecutive_completions: u32,
    /// 参与社区验证次数
    pub community_verifications: u32,
    /// 获得的总评分
    pub total_rating_points: u32,
    /// 被评价次数
    pub rating_count: u32,
    /// 最后活动时间
    pub last_activity: Moment,
    /// 最快完成任务时间（小时）
    pub fastest_completion: Option<u32>,
}

impl<Moment: Zero> UserStats<Moment> {
    /// 计算平均评分（乘以10，如45表示4.5分）
    pub fn average_rating(&self) -> u8 {
        if self.rating_count == 0 {
            0
        } else {
            ((self.total_rating_points * 10) / self.rating_count) as u8
        }
    }
}

/// 成就解锁事件数据
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct AchievementUnlocked<AccountId, Moment> {
    /// 用户账户
    pub user: AccountId,
    /// 成就ID
    pub achievement_id: u32,
    /// 解锁时间
    pub unlocked_at: Moment,
    /// 相关数据（如任务ID、声誉分数等）
    pub context_data: Option<Vec<u8>>,
}
