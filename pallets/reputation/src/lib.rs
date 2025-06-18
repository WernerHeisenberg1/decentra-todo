#![cfg_attr(not(feature = "std"), no_std)]

/// 声誉系统 Pallet
///
/// 提供基于任务完成质量的声誉评分功能：
/// - 任务评价系统
/// - 声誉分数计算
/// - 声誉等级管理
/// - 声誉历史记录
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
    use super::types::{ReputationLevel, TaskEvaluation, TaskRating, UserReputation};
    use codec::MaxEncodedLen;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Get, Randomness},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, SaturatedConversion, Zero};
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

        /// The moment type for timestamps
        type Moment: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Default
            + Copy
            + MaxEncodedLen
            + Zero;

        /// 评价备注最大长度
        #[pallet::constant]
        type MaxCommentLength: Get<u32>;

        /// 每个任务的最大评价数量
        #[pallet::constant]
        type MaxEvaluationsPerTask: Get<u32>;

        /// 声誉历史记录最大数量
        #[pallet::constant]
        type MaxReputationHistory: Get<u32>;

        /// 基础声誉分数（完成任务获得的基础分数）
        #[pallet::constant]
        type BaseReputationScore: Get<u32>;

        /// 难度奖励系数（难度越高获得的额外分数越多）
        #[pallet::constant]
        type DifficultyBonus: Get<u32>;

        /// 取消任务的声誉惩罚
        #[pallet::constant]
        type CancellationPenalty: Get<u32>;

        /// 随机数生成器
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
    }

    /// 用户声誉存储
    #[pallet::storage]
    #[pallet::getter(fn user_reputation)]
    pub type UserReputations<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, UserReputation<T>, ValueQuery>;

    /// 任务评价存储
    #[pallet::storage]
    #[pallet::getter(fn task_evaluations)]
    pub type TaskEvaluations<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        u32, // task_id
        Blake2_128Concat,
        T::AccountId, // evaluator
        TaskEvaluation<T>,
    >;

    /// 任务的所有评价列表
    #[pallet::storage]
    #[pallet::getter(fn task_evaluation_list)]
    pub type TaskEvaluationList<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // task_id
        BoundedVec<T::AccountId, T::MaxEvaluationsPerTask>,
        ValueQuery,
    >;

    /// 声誉历史记录（用于追踪分数变化）
    #[pallet::storage]
    #[pallet::getter(fn reputation_history)]
    pub type ReputationHistory<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<(T::Moment, u32, i32), T::MaxReputationHistory>, // (时间戳, 总分数, 变化量)
        ValueQuery,
    >;

    /// 声誉排行榜（按分数排序的前N名用户）
    #[pallet::storage]
    #[pallet::getter(fn reputation_leaderboard)]
    pub type ReputationLeaderboard<T: Config> =
        StorageValue<_, BoundedVec<(T::AccountId, u32), ConstU32<100>>, ValueQuery>;

    // Pallets use events to inform users when important changes are made.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 任务已被评价
        TaskEvaluated {
            task_id: u32,
            assignee: T::AccountId,
            evaluator: T::AccountId,
            rating: u8,
        },
        /// 声誉已更新
        ReputationUpdated {
            user: T::AccountId,
            old_score: u32,
            new_score: u32,
            old_level: u8,
            new_level: u8,
        },
        /// 任务完成声誉奖励
        TaskCompletionReward {
            user: T::AccountId,
            task_id: u32,
            base_score: u32,
            difficulty_bonus: u32,
            rating_bonus: u32,
            total_reward: u32,
        },
        /// 任务取消声誉惩罚
        TaskCancellationPenalty {
            user: T::AccountId,
            task_id: u32,
            penalty: u32,
        },
        /// 声誉等级提升
        ReputationLevelUp {
            user: T::AccountId,
            old_level: u8,
            new_level: u8,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// 任务不存在
        TaskNotFound,
        /// 任务未完成，无法评价
        TaskNotCompleted,
        /// 无权限评价该任务（只有任务创建者可以评价）
        NotAuthorizedToEvaluate,
        /// 不能评价自己完成的任务
        CannotEvaluateOwnTask,
        /// 任务已被评价
        TaskAlreadyEvaluated,
        /// 评价备注过长
        CommentTooLong,
        /// 无效的评分
        InvalidRating,
        /// 评价数量达到上限
        TooManyEvaluations,
        /// 用户声誉不存在
        ReputationNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 评价已完成的任务
        #[pallet::weight(10_000)]
        pub fn evaluate_task(
            origin: OriginFor<T>,
            task_id: u32,
            rating: u8,
            comment: Vec<u8>,
        ) -> DispatchResult {
            let evaluator = ensure_signed(origin)?;

            // 验证评分有效性
            let task_rating =
                TaskRating::try_from(rating).map_err(|_| Error::<T>::InvalidRating)?;

            // 这里需要通过其他方式获取任务信息，暂时注释掉
            // 在实际实现中，可以通过traits或者其他方式与tasks pallet交互

            // 暂时跳过任务状态检查，在实际部署时需要实现proper的任务验证
            // let task = pallet_tasks::Tasks::<T>::get(task_id).ok_or(Error::<T>::TaskNotFound)?;
            // ensure!(task.status == 2, Error::<T>::TaskNotCompleted); // 2 = Completed

            // 暂时跳过权限检查，在实际实现中需要从task获取创建者信息
            // ensure!(task.creator == evaluator, Error::<T>::NotAuthorizedToEvaluate);

            // 临时方案：假设存在assignee（在实际实现中应该从task获取）
            let assignee = evaluator.clone(); // 这里应该是从任务中获取的真实assignee

            // 不能评价自己的任务
            ensure!(assignee != evaluator, Error::<T>::CannotEvaluateOwnTask);

            // 检查是否已经评价过
            ensure!(
                !TaskEvaluations::<T>::contains_key(task_id, &evaluator),
                Error::<T>::TaskAlreadyEvaluated
            );

            // 验证评论长度
            let bounded_comment: BoundedVec<u8, T::MaxCommentLength> =
                comment.try_into().map_err(|_| Error::<T>::CommentTooLong)?;

            // 获取当前时间戳
            let now = Self::current_timestamp();

            // 创建评价记录
            let evaluation = TaskEvaluation {
                task_id,
                assignee: assignee.clone(),
                evaluator: evaluator.clone(),
                rating: task_rating,
                evaluated_at: now,
                comment: bounded_comment,
            };

            // 存储评价
            TaskEvaluations::<T>::insert(task_id, &evaluator, &evaluation);

            // 更新任务评价列表
            TaskEvaluationList::<T>::mutate(task_id, |evaluators| {
                evaluators
                    .try_push(evaluator.clone())
                    .map_err(|_| Error::<T>::TooManyEvaluations)
            })?;

            // 更新执行者的声誉（暂时使用固定难度值）
            Self::update_user_reputation_with_rating(&assignee, task_id, task_rating, 5)?;

            Self::deposit_event(Event::TaskEvaluated {
                task_id,
                assignee,
                evaluator,
                rating,
            });

            Ok(())
        }

        /// 手动更新用户声誉（仅管理员可用）
        #[pallet::weight(10_000)]
        pub fn update_reputation(
            origin: OriginFor<T>,
            user: T::AccountId,
            score_change: i32,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let mut reputation = UserReputations::<T>::get(&user);
            let old_score = reputation.total_score;
            let old_level = reputation.level;

            // 计算新分数（避免负数）
            let new_score = if score_change >= 0 {
                reputation.total_score.saturating_add(score_change as u32)
            } else {
                reputation
                    .total_score
                    .saturating_sub((-score_change) as u32)
            };

            reputation.update_score(new_score);
            reputation.last_updated = Self::current_timestamp();

            // 更新存储
            UserReputations::<T>::insert(&user, &reputation);

            // 记录历史
            Self::record_reputation_change(&user, new_score, score_change)?;

            // 更新排行榜
            Self::update_leaderboard(&user, new_score)?;

            // 检查等级提升
            if reputation.level > old_level {
                Self::deposit_event(Event::ReputationLevelUp {
                    user: user.clone(),
                    old_level: old_level as u8,
                    new_level: reputation.level as u8,
                });
            }

            Self::deposit_event(Event::ReputationUpdated {
                user,
                old_score,
                new_score,
                old_level: old_level as u8,
                new_level: reputation.level as u8,
            });

            Ok(())
        }
    }

    // 辅助函数
    impl<T: Config> Pallet<T> {
        /// 获取当前时间戳
        ///
        /// 注意：在实际项目中，应该集成 pallet_timestamp 来获取准确的区块时间戳
        /// 目前使用区块号作为时间戳的替代方案
        pub fn current_timestamp() -> T::Moment {
            let block_number = <frame_system::Pallet<T>>::block_number();
            T::Moment::from(block_number.saturated_into::<u32>())
        }

        /// 基于评分和任务难度更新用户声誉
        pub fn update_user_reputation_with_rating(
            user: &T::AccountId,
            task_id: u32,
            rating: TaskRating,
            difficulty: u8,
        ) -> DispatchResult {
            let mut reputation = UserReputations::<T>::get(user);
            let _old_score = reputation.total_score;
            let old_level = reputation.level;

            // 计算基础分数
            let base_score = T::BaseReputationScore::get();

            // 计算难度奖励
            let difficulty_bonus = (difficulty as u32).saturating_mul(T::DifficultyBonus::get());

            // 计算评分奖励（评分越高奖励越多）
            let rating_multiplier = rating.reward_multiplier();
            let rating_bonus = base_score.saturating_mul(rating_multiplier) / 100;
            let total_reward = rating_bonus.saturating_add(difficulty_bonus);

            // 更新统计信息
            reputation.completed_tasks = reputation.completed_tasks.saturating_add(1);
            reputation.total_ratings = reputation.total_ratings.saturating_add(1);
            reputation.rating_sum = reputation.rating_sum.saturating_add(rating as u32);
            reputation.total_score = reputation.total_score.saturating_add(total_reward);
            reputation.level = ReputationLevel::from_score(reputation.total_score);
            reputation.last_updated = Self::current_timestamp();

            // 更新存储
            UserReputations::<T>::insert(user, &reputation);

            // 记录历史
            Self::record_reputation_change(user, reputation.total_score, total_reward as i32)?;

            // 更新排行榜
            Self::update_leaderboard(user, reputation.total_score)?;

            // 检查等级提升
            if reputation.level > old_level {
                Self::deposit_event(Event::ReputationLevelUp {
                    user: user.clone(),
                    old_level: old_level as u8,
                    new_level: reputation.level as u8,
                });
            }

            Self::deposit_event(Event::TaskCompletionReward {
                user: user.clone(),
                task_id,
                base_score,
                difficulty_bonus,
                rating_bonus,
                total_reward,
            });

            Ok(())
        }

        /// 处理任务完成（无评分时的基础奖励）
        pub fn handle_task_completion(
            user: &T::AccountId,
            task_id: u32,
            difficulty: u8,
        ) -> DispatchResult {
            // 给予默认的"良好"评分
            Self::update_user_reputation_with_rating(user, task_id, TaskRating::Good, difficulty)
        }

        /// 处理任务取消的声誉惩罚
        pub fn handle_task_cancellation(user: &T::AccountId, task_id: u32) -> DispatchResult {
            let mut reputation = UserReputations::<T>::get(user);
            let penalty = T::CancellationPenalty::get();

            reputation.cancelled_tasks = reputation.cancelled_tasks.saturating_add(1);
            reputation.total_score = reputation.total_score.saturating_sub(penalty);
            reputation.level = ReputationLevel::from_score(reputation.total_score);
            reputation.last_updated = Self::current_timestamp();

            UserReputations::<T>::insert(user, &reputation);

            // 记录历史
            Self::record_reputation_change(user, reputation.total_score, -(penalty as i32))?;

            // 更新排行榜
            Self::update_leaderboard(user, reputation.total_score)?;

            Self::deposit_event(Event::TaskCancellationPenalty {
                user: user.clone(),
                task_id,
                penalty,
            });

            Ok(())
        }

        /// 记录声誉变化历史
        fn record_reputation_change(
            user: &T::AccountId,
            new_total: u32,
            change: i32,
        ) -> DispatchResult {
            let now = Self::current_timestamp();
            ReputationHistory::<T>::mutate(user, |history| {
                let record = (now, new_total, change);

                // 如果历史记录已满，移除最旧的记录
                if history.len() >= T::MaxReputationHistory::get() as usize {
                    history.remove(0);
                }

                history
                    .try_push(record)
                    .map_err(|_| Error::<T>::TooManyEvaluations)
            })?;

            Ok(())
        }

        /// 更新声誉排行榜
        fn update_leaderboard(user: &T::AccountId, score: u32) -> DispatchResult {
            ReputationLeaderboard::<T>::mutate(|leaderboard| {
                // 移除用户的旧记录（如果存在）
                leaderboard.retain(|(account, _)| account != user);

                // 插入新记录
                let new_entry = (user.clone(), score);

                // 找到插入位置（按分数降序）
                let insert_pos = leaderboard
                    .iter()
                    .position(|(_, s)| *s < score)
                    .unwrap_or(leaderboard.len());

                if insert_pos < 100 && leaderboard.len() < 100 {
                    leaderboard
                        .try_insert(insert_pos, new_entry)
                        .map_err(|_| Error::<T>::TooManyEvaluations)?;
                } else if insert_pos < leaderboard.len() {
                    leaderboard
                        .try_insert(insert_pos, new_entry)
                        .map_err(|_| Error::<T>::TooManyEvaluations)?;
                    if leaderboard.len() > 100 {
                        leaderboard.pop();
                    }
                }

                Ok::<(), Error<T>>(())
            })?;

            Ok(())
        }

        /// 获取任务的平均评分
        pub fn get_task_average_rating(task_id: u32) -> f32 {
            let evaluators = TaskEvaluationList::<T>::get(task_id);
            if evaluators.is_empty() {
                return 0.0;
            }

            let total_rating: u32 = evaluators
                .iter()
                .filter_map(|evaluator| TaskEvaluations::<T>::get(task_id, evaluator))
                .map(|evaluation| evaluation.rating as u32)
                .sum();

            total_rating as f32 / evaluators.len() as f32
        }

        /// 获取用户的声誉等级
        pub fn get_user_level(user: &T::AccountId) -> ReputationLevel {
            UserReputations::<T>::get(user).level
        }

        /// 获取用户的完成率
        pub fn get_user_completion_rate(user: &T::AccountId) -> f32 {
            UserReputations::<T>::get(user).completion_rate()
        }
    }

    /// 实现与tasks pallet的集成 hooks
    impl<T: Config> Pallet<T> {
        /// 当任务状态改变时调用（由tasks pallet调用）
        pub fn on_task_status_changed(
            task_id: u32,
            assignee: Option<&T::AccountId>,
            _old_status: u8,
            new_status: u8,
            difficulty: u8,
        ) -> DispatchResult {
            // 任务完成时给予基础声誉奖励 (状态码2表示Completed)
            if new_status == 2 {
                if let Some(user) = assignee {
                    Self::handle_task_completion(user, task_id, difficulty)?;
                }
            }

            // 任务取消时扣除声誉 (状态码3表示Cancelled)
            if new_status == 3 {
                if let Some(user) = assignee {
                    Self::handle_task_cancellation(user, task_id)?;
                }
            }

            Ok(())
        }
    }
}
