#![cfg_attr(not(feature = "std"), no_std)]

/// NFT成就系统 Pallet
///
/// 基于任务完成情况和声誉等级自动检测里程碑并铸造NFT成就：
/// - 成就定义和管理
/// - 自动里程碑检测
/// - NFT自动铸造
/// - 成就进度跟踪
/// - 用户统计更新
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
    use super::types::*;
    use codec::MaxEncodedLen;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Get, Randomness},
        BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AtLeast32BitUnsigned, Saturating, Zero};
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
            + Zero
            + Saturating;

        /// NFT collection ID type
        type CollectionId: Member + Parameter + Default + Copy + MaxEncodedLen + From<u32>;

        /// NFT item ID type  
        type ItemId: Member + Parameter + Default + Copy + MaxEncodedLen + From<u32>;

        /// 成就名称最大长度
        #[pallet::constant]
        type MaxNameLength: Get<u32>;

        /// 成就描述最大长度
        #[pallet::constant]
        type MaxDescriptionLength: Get<u32>;

        /// 元数据URI最大长度
        #[pallet::constant]
        type MaxMetadataLength: Get<u32>;

        /// 用户最大成就数量
        #[pallet::constant]
        type MaxUserAchievements: Get<u32>;

        /// 系统最大成就定义数量
        #[pallet::constant]
        type MaxAchievements: Get<u32>;

        /// 随机数生成器
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;

        /// NFT处理器类型（可以是任何实现了基本NFT功能的类型）
        type Nfts;
    }

    /// 成就定义存储
    #[pallet::storage]
    #[pallet::getter(fn achievements)]
    pub type Achievements<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32, // achievement_id
        Achievement<BoundedVec<u8, T::MaxNameLength>>,
    >;

    /// 用户成就记录
    #[pallet::storage]
    #[pallet::getter(fn user_achievements)]
    pub type UserAchievements<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // user
        Blake2_128Concat,
        u32, // achievement_id
        UserAchievement<T::AccountId, T::Moment>,
    >;

    /// 用户成就列表
    #[pallet::storage]
    #[pallet::getter(fn user_achievement_list)]
    pub type UserAchievementList<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u32, T::MaxUserAchievements>,
        ValueQuery,
    >;

    /// 用户统计数据
    #[pallet::storage]
    #[pallet::getter(fn user_stats)]
    pub type UserStatsStorage<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, UserStats<T::Moment>, ValueQuery>;

    /// 成就进度跟踪
    #[pallet::storage]
    #[pallet::getter(fn achievement_progress)]
    pub type AchievementProgressStorage<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId, // user
        Blake2_128Concat,
        u32, // achievement_id
        AchievementProgress,
    >;

    /// 下一个成就ID
    #[pallet::storage]
    #[pallet::getter(fn next_achievement_id)]
    pub type NextAchievementId<T> = StorageValue<_, u32, ValueQuery>;

    /// 成就排行榜（按获得成就数量排序）
    #[pallet::storage]
    #[pallet::getter(fn achievement_leaderboard)]
    pub type AchievementLeaderboard<T: Config> =
        StorageValue<_, BoundedVec<(T::AccountId, u32), ConstU32<100>>, ValueQuery>;

    /// 成就NFT集合映射
    #[pallet::storage]
    #[pallet::getter(fn achievement_collections)]
    pub type AchievementCollections<T: Config> =
        StorageMap<_, Blake2_128Concat, u32, T::CollectionId>; // achievement_id -> collection_id

    // Pallets use events to inform users when important changes are made.
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// 新成就已创建
        AchievementCreated {
            achievement_id: u32,
            name: Vec<u8>,
            achievement_type: u8,
            rarity: u8,
        },
        /// 成就已解锁
        AchievementUnlocked {
            user: T::AccountId,
            achievement_id: u32,
            name: Vec<u8>,
            rarity: u8,
        },
        /// 成就NFT已铸造
        AchievementNftMinted {
            user: T::AccountId,
            achievement_id: u32,
            collection_id: T::CollectionId,
            item_id: T::ItemId,
        },
        /// 用户统计已更新
        UserStatsUpdated {
            user: T::AccountId,
            stats_type: Vec<u8>,
            old_value: u32,
            new_value: u32,
        },
        /// 成就进度已更新
        AchievementProgressUpdated {
            user: T::AccountId,
            achievement_id: u32,
            current_progress: u32,
            target_progress: u32,
            completion_percentage: u8,
        },
        /// 成就已更新
        AchievementUpdated { achievement_id: u32, name: Vec<u8> },
        /// 成就已停用
        AchievementDeactivated { achievement_id: u32 },
        /// NFT集合已创建
        AchievementCollectionCreated {
            achievement_id: u32,
            collection_id: T::CollectionId,
        },
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// 成就不存在
        AchievementNotFound,
        /// 成就已存在
        AchievementAlreadyExists,
        /// 用户已获得该成就
        AchievementAlreadyUnlocked,
        /// 成就名称过长
        NameTooLong,
        /// 成就描述过长
        DescriptionTooLong,
        /// 元数据URI过长
        MetadataTooLong,
        /// 用户成就数量达到上限
        TooManyUserAchievements,
        /// 系统成就数量达到上限
        TooManyAchievements,
        /// 无效的成就条件
        InvalidAchievementCondition,
        /// 成就未激活
        AchievementNotActive,
        /// NFT铸造失败
        NftMintingFailed,
        /// NFT集合创建失败
        CollectionCreationFailed,
        /// 无权限操作
        NotAuthorized,
        /// 成就条件未满足
        ConditionNotMet,
        /// 用户统计不存在
        UserStatsNotFound,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// 创建新成就定义（需要root权限）
        #[pallet::weight(10_000)]
        pub fn create_achievement(
            origin: OriginFor<T>,
            name: Vec<u8>,
            description: Vec<u8>,
            achievement_type: u8, // 简化为u8：0=TaskCompletion, 1=Reputation, 2=Community, 3=Special
            rarity: u8,           // 简化为u8：0=Common, 1=Rare, 2=Epic, 3=Legendary
            condition_type: u8,   // 条件类型
            condition_value: u32, // 条件值
            metadata_uri: Vec<u8>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            // 验证输入长度
            ensure!(
                name.len() <= T::MaxNameLength::get() as usize,
                Error::<T>::NameTooLong
            );
            ensure!(
                description.len() <= T::MaxDescriptionLength::get() as usize,
                Error::<T>::DescriptionTooLong
            );
            ensure!(
                metadata_uri.len() <= T::MaxMetadataLength::get() as usize,
                Error::<T>::MetadataTooLong
            );

            let achievement_id = NextAchievementId::<T>::get();
            NextAchievementId::<T>::put(achievement_id.saturating_add(1));

            let bounded_name =
                BoundedVec::try_from(name.clone()).map_err(|_| Error::<T>::NameTooLong)?;
            let bounded_description =
                BoundedVec::try_from(description).map_err(|_| Error::<T>::DescriptionTooLong)?;
            let bounded_metadata =
                BoundedVec::try_from(metadata_uri).map_err(|_| Error::<T>::MetadataTooLong)?;

            // 转换u8到枚举类型
            let achievement_type_enum = match achievement_type {
                0 => AchievementType::TaskCompletion,
                1 => AchievementType::Reputation,
                2 => AchievementType::Community,
                _ => AchievementType::Special,
            };

            let rarity_enum = match rarity {
                0 => AchievementRarity::Common,
                1 => AchievementRarity::Rare,
                2 => AchievementRarity::Epic,
                _ => AchievementRarity::Legendary,
            };

            let condition_enum = match condition_type {
                0 => AchievementCondition::CompleteTasksCount(condition_value),
                1 => AchievementCondition::ReachReputationScore(condition_value),
                2 => AchievementCondition::ConsecutiveTaskCompletion(condition_value),
                3 => AchievementCondition::AverageRating(condition_value as u8),
                4 => AchievementCondition::CommunityVerificationCount(condition_value),
                5 => AchievementCondition::CreateTasksCount(condition_value),
                _ => AchievementCondition::CompleteTaskInTime(condition_value),
            };

            // 简化：直接使用achievement_id作为collection_id
            let collection_id = achievement_id;

            let achievement = Achievement {
                id: achievement_id,
                name: bounded_name,
                description: bounded_description,
                achievement_type: achievement_type_enum,
                rarity: rarity_enum,
                condition: condition_enum,
                collection_id,
                metadata_uri: bounded_metadata,
                is_active: true,
            };

            Achievements::<T>::insert(achievement_id, achievement);

            Self::deposit_event(Event::AchievementCreated {
                achievement_id,
                name,
                achievement_type,
                rarity,
            });

            Ok(())
        }

        /// 更新成就定义（需要root权限）
        #[pallet::weight(10_000)]
        pub fn update_achievement(
            origin: OriginFor<T>,
            achievement_id: u32,
            name: Option<Vec<u8>>,
            description: Option<Vec<u8>>,
            metadata_uri: Option<Vec<u8>>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            Achievements::<T>::try_mutate(achievement_id, |achievement_opt| {
                let achievement = achievement_opt
                    .as_mut()
                    .ok_or(Error::<T>::AchievementNotFound)?;

                if let Some(new_name) = name.clone() {
                    ensure!(
                        new_name.len() <= T::MaxNameLength::get() as usize,
                        Error::<T>::NameTooLong
                    );
                    achievement.name =
                        BoundedVec::try_from(new_name).map_err(|_| Error::<T>::NameTooLong)?;
                }

                if let Some(new_description) = description {
                    ensure!(
                        new_description.len() <= T::MaxDescriptionLength::get() as usize,
                        Error::<T>::DescriptionTooLong
                    );
                    achievement.description = BoundedVec::try_from(new_description)
                        .map_err(|_| Error::<T>::DescriptionTooLong)?;
                }

                if let Some(new_metadata) = metadata_uri {
                    ensure!(
                        new_metadata.len() <= T::MaxMetadataLength::get() as usize,
                        Error::<T>::MetadataTooLong
                    );
                    achievement.metadata_uri = BoundedVec::try_from(new_metadata)
                        .map_err(|_| Error::<T>::MetadataTooLong)?;
                }

                Ok::<(), DispatchError>(())
            })?;

            if let Some(name) = name {
                Self::deposit_event(Event::AchievementUpdated {
                    achievement_id,
                    name,
                });
            }

            Ok(())
        }

        /// 停用成就（需要root权限）
        #[pallet::weight(10_000)]
        pub fn deactivate_achievement(origin: OriginFor<T>, achievement_id: u32) -> DispatchResult {
            ensure_root(origin)?;

            Achievements::<T>::try_mutate(achievement_id, |achievement_opt| {
                let achievement = achievement_opt
                    .as_mut()
                    .ok_or(Error::<T>::AchievementNotFound)?;
                achievement.is_active = false;
                Ok::<(), DispatchError>(())
            })?;

            Self::deposit_event(Event::AchievementDeactivated { achievement_id });

            Ok(())
        }

        /// 手动检查并解锁成就（用于测试或修复）
        #[pallet::weight(10_000)]
        pub fn check_and_unlock_achievements(
            origin: OriginFor<T>,
            user: T::AccountId,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Self::check_all_achievements(&user)?;

            Ok(())
        }
    }

    // Helper functions
    impl<T: Config> Pallet<T> {
        /// 获取当前时间戳
        pub fn current_timestamp() -> T::Moment {
            T::Moment::zero()
        }

        /// 为成就创建NFT集合
        fn create_achievement_collection(
            achievement_id: u32,
        ) -> Result<T::CollectionId, DispatchError> {
            let collection_id = T::CollectionId::default();

            // 这里应该调用NFT pallet创建集合
            // T::Nfts::create_collection(&collection_id, &admin, &config)?;

            Self::deposit_event(Event::AchievementCollectionCreated {
                achievement_id,
                collection_id,
            });

            Ok(collection_id)
        }

        /// 检查所有成就
        pub fn check_all_achievements(user: &T::AccountId) -> DispatchResult {
            let user_stats = UserStatsStorage::<T>::get(user);

            // 遍历所有激活的成就
            for (achievement_id, achievement) in Achievements::<T>::iter() {
                if !achievement.is_active {
                    continue;
                }

                // 检查用户是否已获得该成就
                if UserAchievements::<T>::contains_key(user, achievement_id) {
                    continue;
                }

                // 检查成就条件
                if Self::check_achievement_condition(&achievement.condition, &user_stats)? {
                    Self::unlock_achievement(user, achievement_id)?;
                }
            }

            Ok(())
        }

        /// 检查成就条件是否满足
        fn check_achievement_condition(
            condition: &AchievementCondition,
            user_stats: &UserStats<T::Moment>,
        ) -> Result<bool, DispatchError> {
            let is_met = match condition {
                AchievementCondition::CompleteTasksCount(required) => {
                    user_stats.tasks_completed >= *required
                }
                AchievementCondition::ReachReputationScore(_score) => {
                    // 需要从reputation pallet获取用户分数
                    // 这里先返回false，实际应用中需要集成
                    false
                }
                AchievementCondition::ConsecutiveTaskCompletion(required) => {
                    user_stats.consecutive_completions >= *required
                }
                AchievementCondition::AverageRating(required) => {
                    user_stats.average_rating() >= *required
                }
                AchievementCondition::CommunityVerificationCount(required) => {
                    user_stats.community_verifications >= *required
                }
                AchievementCondition::CreateTasksCount(required) => {
                    user_stats.tasks_created >= *required
                }
                AchievementCondition::CompleteTaskInTime(required_hours) => user_stats
                    .fastest_completion
                    .map_or(false, |time| time <= *required_hours),
            };

            Ok(is_met)
        }

        /// 解锁成就
        fn unlock_achievement(user: &T::AccountId, achievement_id: u32) -> DispatchResult {
            let achievement =
                Achievements::<T>::get(achievement_id).ok_or(Error::<T>::AchievementNotFound)?;

            ensure!(achievement.is_active, Error::<T>::AchievementNotActive);

            // 检查用户是否已获得该成就
            ensure!(
                !UserAchievements::<T>::contains_key(user, achievement_id),
                Error::<T>::AchievementAlreadyUnlocked
            );

            let current_time = Self::current_timestamp();

            // 铸造NFT
            // 简化：使用achievement_id作为item_id
            let item_id = achievement_id;

            // 记录用户成就
            let user_achievement = UserAchievement {
                achievement_id,
                earned_at: current_time,
                nft_item_id: Some(item_id),
                owner: user.clone(),
            };

            UserAchievements::<T>::insert(user, achievement_id, user_achievement);

            // 更新用户成就列表
            UserAchievementList::<T>::try_mutate(user, |list| {
                list.try_push(achievement_id)
                    .map_err(|_| Error::<T>::TooManyUserAchievements)
            })?;

            // 更新排行榜
            Self::update_achievement_leaderboard(user)?;

            // 转换枚举到u8
            let rarity_u8 = match achievement.rarity {
                AchievementRarity::Common => 0,
                AchievementRarity::Rare => 1,
                AchievementRarity::Epic => 2,
                AchievementRarity::Legendary => 3,
            };

            Self::deposit_event(Event::AchievementUnlocked {
                user: user.clone(),
                achievement_id,
                name: achievement.name.to_vec(),
                rarity: rarity_u8,
            });

            Ok(())
        }

        /// 铸造成就NFT
        fn mint_achievement_nft(
            user: &T::AccountId,
            achievement: &Achievement<BoundedVec<u8, T::MaxNameLength>>,
        ) -> Result<T::ItemId, DispatchError> {
            let collection_id = AchievementCollections::<T>::get(achievement.id)
                .ok_or(Error::<T>::AchievementNotFound)?;

            let item_id = T::ItemId::default();

            // 这里应该调用NFT pallet铸造NFT
            // T::Nfts::mint_into(&collection_id, &item_id, user)?;

            Self::deposit_event(Event::AchievementNftMinted {
                user: user.clone(),
                achievement_id: achievement.id,
                collection_id,
                item_id,
            });

            Ok(item_id)
        }

        /// 更新用户统计
        pub fn update_user_stats<F>(user: &T::AccountId, update_fn: F) -> DispatchResult
        where
            F: FnOnce(&mut UserStats<T::Moment>) -> DispatchResult,
        {
            UserStatsStorage::<T>::try_mutate(user, |stats| {
                update_fn(stats)?;

                // 触发成就检查
                Self::check_all_achievements(user)?;

                Ok(())
            })
        }

        /// 更新成就排行榜
        fn update_achievement_leaderboard(user: &T::AccountId) -> DispatchResult {
            let user_achievement_count = UserAchievementList::<T>::get(user).len() as u32;

            AchievementLeaderboard::<T>::mutate(|leaderboard| {
                // 查找用户是否已在排行榜中
                if let Some(pos) = leaderboard.iter().position(|(acc, _)| acc == user) {
                    leaderboard[pos].1 = user_achievement_count;
                } else if leaderboard.len() < 100 {
                    // 添加新用户到排行榜
                    let _ = leaderboard.try_push((user.clone(), user_achievement_count));
                }

                // 按成就数量排序
                leaderboard.sort_by(|a, b| b.1.cmp(&a.1));
            });

            Ok(())
        }

        /// 获取用户成就列表
        pub fn get_user_achievements(user: &T::AccountId) -> Vec<u32> {
            UserAchievementList::<T>::get(user).to_vec()
        }

        /// 获取成就详情
        pub fn get_achievement_details(
            achievement_id: u32,
        ) -> Option<Achievement<BoundedVec<u8, T::MaxNameLength>>> {
            Achievements::<T>::get(achievement_id)
        }

        /// 获取用户统计
        pub fn get_user_stats(user: &T::AccountId) -> UserStats<T::Moment> {
            UserStatsStorage::<T>::get(user)
        }
    }

    // 任务事件处理
    impl<T: Config> Pallet<T> {
        /// 处理任务完成事件
        pub fn on_task_completed(
            user: &T::AccountId,
            _task_id: u32,
            _difficulty: u8,
        ) -> DispatchResult {
            Self::update_user_stats(user, |stats| {
                stats.tasks_completed = stats.tasks_completed.saturating_add(1);
                stats.consecutive_completions = stats.consecutive_completions.saturating_add(1);
                stats.last_activity = Self::current_timestamp();

                Self::deposit_event(Event::UserStatsUpdated {
                    user: user.clone(),
                    stats_type: b"tasks_completed".to_vec(),
                    old_value: stats.tasks_completed.saturating_sub(1),
                    new_value: stats.tasks_completed,
                });

                Ok(())
            })
        }

        /// 处理任务创建事件
        pub fn on_task_created(user: &T::AccountId, _task_id: u32) -> DispatchResult {
            Self::update_user_stats(user, |stats| {
                stats.tasks_created = stats.tasks_created.saturating_add(1);
                stats.last_activity = Self::current_timestamp();

                Self::deposit_event(Event::UserStatsUpdated {
                    user: user.clone(),
                    stats_type: b"tasks_created".to_vec(),
                    old_value: stats.tasks_created.saturating_sub(1),
                    new_value: stats.tasks_created,
                });

                Ok(())
            })
        }

        /// 处理任务取消事件
        pub fn on_task_cancelled(user: &T::AccountId, _task_id: u32) -> DispatchResult {
            Self::update_user_stats(user, |stats| {
                stats.consecutive_completions = 0; // 重置连续完成计数
                stats.last_activity = Self::current_timestamp();

                Self::deposit_event(Event::UserStatsUpdated {
                    user: user.clone(),
                    stats_type: b"consecutive_completions".to_vec(),
                    old_value: stats.consecutive_completions,
                    new_value: 0,
                });

                Ok(())
            })
        }

        /// 处理社区验证参与事件
        pub fn on_community_verification(user: &T::AccountId, _task_id: u32) -> DispatchResult {
            Self::update_user_stats(user, |stats| {
                stats.community_verifications = stats.community_verifications.saturating_add(1);
                stats.last_activity = Self::current_timestamp();

                Self::deposit_event(Event::UserStatsUpdated {
                    user: user.clone(),
                    stats_type: b"community_verifications".to_vec(),
                    old_value: stats.community_verifications.saturating_sub(1),
                    new_value: stats.community_verifications,
                });

                Ok(())
            })
        }

        /// 处理任务评价事件
        pub fn on_task_rated(user: &T::AccountId, _task_id: u32, rating: u8) -> DispatchResult {
            Self::update_user_stats(user, |stats| {
                stats.total_rating_points = stats.total_rating_points.saturating_add(rating as u32);
                stats.rating_count = stats.rating_count.saturating_add(1);
                stats.last_activity = Self::current_timestamp();

                Self::deposit_event(Event::UserStatsUpdated {
                    user: user.clone(),
                    stats_type: b"average_rating".to_vec(),
                    old_value: stats.average_rating() as u32,
                    new_value: stats.average_rating() as u32,
                });

                Ok(())
            })
        }
    }
}
