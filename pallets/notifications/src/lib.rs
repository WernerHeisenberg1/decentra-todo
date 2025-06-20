#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    codec::{Decode, Encode, MaxEncodedLen},
    dispatch::DispatchResult,
    pallet_prelude::*,
    scale_info::TypeInfo,
    traits::{Get, Hooks, BuildGenesisConfig},
    BoundedVec,
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::{AtLeast32BitUnsigned, UniqueSaturatedInto};
use sp_std::vec::Vec;

pub use pallet::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum NotificationType {
    TaskCreated,
    TaskAssigned,
    TaskStatusChanged,
    CommunityVerification,
    ReputationLevelUp,
    AchievementUnlocked,
    RewardReceived,
    SystemAnnouncement,
    Other,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
}

impl Default for NotificationPriority {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct Notification<AccountId, Moment> {
    pub id: u32,
    pub user: AccountId,
    pub notification_type: NotificationType,
    pub title: BoundedVec<u8, ConstU32<64>>,
    pub content: BoundedVec<u8, ConstU32<256>>,
    pub priority: NotificationPriority,
    pub is_read: bool,
    pub created_at: Moment,
    pub read_at: Option<Moment>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct NotificationPreferences {
    pub task_notifications: bool,
    pub community_notifications: bool,
    pub reputation_notifications: bool,
    pub achievement_notifications: bool,
    pub reward_notifications: bool,
    pub system_notifications: bool,
}

impl Default for NotificationPreferences {
    fn default() -> Self {
        Self {
            task_notifications: true,
            community_notifications: true,
            reputation_notifications: true,
            achievement_notifications: true,
            reward_notifications: true,
            system_notifications: true,
        }
    }
}

impl NotificationPreferences {
    pub fn is_enabled(&self, notification_type: &NotificationType) -> bool {
        match notification_type {
            NotificationType::TaskCreated
            | NotificationType::TaskAssigned
            | NotificationType::TaskStatusChanged => self.task_notifications,
            NotificationType::CommunityVerification => self.community_notifications,
            NotificationType::ReputationLevelUp => self.reputation_notifications,
            NotificationType::AchievementUnlocked => self.achievement_notifications,
            NotificationType::RewardReceived => self.reward_notifications,
            NotificationType::SystemAnnouncement => self.system_notifications,
            NotificationType::Other => true,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct NotificationStatistics {
    pub total_received: u32,
    pub total_read: u32,
    pub unread_count: u32,
    pub last_notification_at: Option<u32>,
}

impl Default for NotificationStatistics {
    fn default() -> Self {
        Self {
            total_received: 0,
            total_read: 0,
            unread_count: 0,
            last_notification_at: None,
        }
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Moment: Parameter + Default + AtLeast32BitUnsigned + Copy + MaxEncodedLen;

        #[pallet::constant]
        type MaxUserNotifications: Get<u32>;

        #[pallet::constant]
        type NotificationRetentionPeriod: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn user_notifications)]
    pub type UserNotifications<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        u32,
        Notification<T::AccountId, T::Moment>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn user_notification_list)]
    pub type UserNotificationList<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u32, T::MaxUserNotifications>,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn user_notification_preferences)]
    pub type UserNotificationPreferences<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, NotificationPreferences, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn notification_stats)]
    pub type NotificationStats<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, NotificationStatistics, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_notification_id)]
    pub type NextNotificationId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        NotificationCreated {
            user: T::AccountId,
            notification_id: u32,
            notification_type: NotificationType,
        },
        NotificationRead {
            user: T::AccountId,
            notification_id: u32,
        },
        NotificationDeleted {
            user: T::AccountId,
            notification_id: u32,
        },
        NotificationPreferencesUpdated {
            user: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotificationNotFound,
        TitleTooLong,
        ContentTooLong,
        TooManyNotifications,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn mark_notification_read(
            origin: OriginFor<T>,
            notification_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            UserNotifications::<T>::try_mutate(&who, &notification_id, |notification| {
                match notification {
                    Some(ref mut n) => {
                        if !n.is_read {
                            n.is_read = true;
                            n.read_at = Some(Self::current_timestamp());

                            NotificationStats::<T>::mutate(&who, |stats| {
                                stats.total_read = stats.total_read.saturating_add(1);
                                stats.unread_count = stats.unread_count.saturating_sub(1);
                            });

                            Self::deposit_event(Event::NotificationRead {
                                user: who.clone(),
                                notification_id,
                            });
                        }
                        Ok(())
                    }
                    None => Err(Error::<T>::NotificationNotFound),
                }
            })?;

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn delete_notification(origin: OriginFor<T>, notification_id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let notification = UserNotifications::<T>::get(&who, &notification_id)
                .ok_or(Error::<T>::NotificationNotFound)?;

            UserNotifications::<T>::remove(&who, &notification_id);

            UserNotificationList::<T>::mutate(&who, |list| {
                list.retain(|&id| id != notification_id);
            });

            if !notification.is_read {
                NotificationStats::<T>::mutate(&who, |stats| {
                    stats.unread_count = stats.unread_count.saturating_sub(1);
                });
            }

            Self::deposit_event(Event::NotificationDeleted {
                user: who,
                notification_id,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000, 0))]
        pub fn update_notification_preferences(
            origin: OriginFor<T>,
            preferences: NotificationPreferences,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            UserNotificationPreferences::<T>::insert(&who, &preferences);

            Self::deposit_event(Event::NotificationPreferencesUpdated { user: who });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn current_timestamp() -> T::Moment {
            let timestamp: u32 = 0; // Placeholder - would get from pallet_timestamp
            timestamp.into()
        }

        pub fn create_notification(
            user: T::AccountId,
            notification_type: NotificationType,
            title: Vec<u8>,
            content: Vec<u8>,
            priority: NotificationPriority,
        ) -> DispatchResult {
            let preferences = UserNotificationPreferences::<T>::get(&user);
            if !preferences.is_enabled(&notification_type) {
                return Ok(());
            }

            let title_bounded: BoundedVec<u8, ConstU32<64>> =
                title.try_into().map_err(|_| Error::<T>::TitleTooLong)?;
            let content_bounded: BoundedVec<u8, ConstU32<256>> =
                content.try_into().map_err(|_| Error::<T>::ContentTooLong)?;

            let notification_id = NextNotificationId::<T>::get();
            NextNotificationId::<T>::put(notification_id.saturating_add(1));

            let mut notification_list = UserNotificationList::<T>::get(&user);
            notification_list
                .try_push(notification_id)
                .map_err(|_| Error::<T>::TooManyNotifications)?;

            if notification_list.len() > T::MaxUserNotifications::get() as usize {
                notification_list.remove(0);
            }

            let notification = Notification {
                id: notification_id,
                user: user.clone(),
                notification_type: notification_type.clone(),
                title: title_bounded,
                content: content_bounded,
                priority,
                is_read: false,
                created_at: Self::current_timestamp(),
                read_at: None,
            };

            UserNotifications::<T>::insert(&user, notification_id, &notification);
            UserNotificationList::<T>::insert(&user, &notification_list);

            NotificationStats::<T>::mutate(&user, |stats| {
                stats.total_received = stats.total_received.saturating_add(1);
                stats.unread_count = stats.unread_count.saturating_add(1);
                stats.last_notification_at = Some(notification_id);
            });

            Self::deposit_event(Event::NotificationCreated {
                user,
                notification_id,
                notification_type,
            });

            Ok(())
        }

        pub fn on_task_created(user: T::AccountId, _task_id: u32) -> DispatchResult {
            Self::create_notification(
                user,
                NotificationType::TaskCreated,
                b"Task Created".to_vec(),
                b"A new task has been created".to_vec(),
                NotificationPriority::Medium,
            )
        }

        pub fn on_task_assigned(user: T::AccountId, _task_id: u32) -> DispatchResult {
            Self::create_notification(
                user,
                NotificationType::TaskAssigned,
                b"Task Assigned".to_vec(),
                b"A task has been assigned to you".to_vec(),
                NotificationPriority::High,
            )
        }

        pub fn on_task_status_changed(user: T::AccountId, _task_id: u32) -> DispatchResult {
            Self::create_notification(
                user,
                NotificationType::TaskStatusChanged,
                b"Task Status Changed".to_vec(),
                b"Task status has been updated".to_vec(),
                NotificationPriority::Medium,
            )
        }

        pub fn on_reputation_level_up(
            user: T::AccountId,
            _old_level: u8,
            _new_level: u8,
        ) -> DispatchResult {
            Self::create_notification(
                user,
                NotificationType::ReputationLevelUp,
                b"Reputation Level Up".to_vec(),
                b"Your reputation level has increased".to_vec(),
                NotificationPriority::High,
            )
        }

        pub fn on_achievement_unlocked(user: T::AccountId, _achievement_id: u32) -> DispatchResult {
            Self::create_notification(
                user,
                NotificationType::AchievementUnlocked,
                b"Achievement Unlocked".to_vec(),
                b"You have unlocked a new achievement".to_vec(),
                NotificationPriority::High,
            )
        }
    }
}
