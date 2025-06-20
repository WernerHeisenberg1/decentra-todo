//! Benchmarking setup for pallet-achievements

use super::*;

#[allow(unused)]
use crate::Pallet as Achievements;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_achievement() {
        let name = b"First Task".to_vec();
        let description = b"Complete your first task".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            name,
            description,
            AchievementType::TaskCompletion,
            AchievementRarity::Common,
            AchievementCondition::CompleteTasksCount(1),
            metadata_uri,
        );

        assert_eq!(NextAchievementId::<T>::get(), 1);
    }

    #[benchmark]
    fn update_achievement() {
        // Setup: create an achievement first
        let name = b"Test Achievement".to_vec();
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        let _ = Achievements::<T>::create_achievement(
            RawOrigin::Root.into(),
            name,
            description,
            AchievementType::TaskCompletion,
            AchievementRarity::Common,
            AchievementCondition::CompleteTasksCount(1),
            metadata_uri,
        );

        let new_name = b"Updated Achievement".to_vec();

        #[extrinsic_call]
        _(RawOrigin::Root, 0u32, Some(new_name), None, None);

        assert!(Achievements::<T>::get(0).is_some());
    }

    #[benchmark]
    fn deactivate_achievement() {
        // Setup: create an achievement first
        let name = b"Test Achievement".to_vec();
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        let _ = Achievements::<T>::create_achievement(
            RawOrigin::Root.into(),
            name,
            description,
            AchievementType::TaskCompletion,
            AchievementRarity::Common,
            AchievementCondition::CompleteTasksCount(1),
            metadata_uri,
        );

        #[extrinsic_call]
        _(RawOrigin::Root, 0u32);

        let achievement = Achievements::<T>::get(0).unwrap();
        assert!(!achievement.is_active);
    }

    #[benchmark]
    fn check_and_unlock_achievements() {
        let caller: T::AccountId = whitelisted_caller();

        // Setup: create an achievement
        let name = b"Test Achievement".to_vec();
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        let _ = Achievements::<T>::create_achievement(
            RawOrigin::Root.into(),
            name,
            description,
            AchievementType::TaskCompletion,
            AchievementRarity::Common,
            AchievementCondition::CompleteTasksCount(1),
            metadata_uri,
        );

        // Setup user stats to meet the condition
        UserStats::<T>::mutate(&caller, |stats| {
            stats.tasks_completed = 1;
        });

        #[extrinsic_call]
        _(RawOrigin::Signed(caller.clone()), caller.clone());

        assert!(UserAchievements::<T>::contains_key(&caller, 0));
    }

    impl_benchmark_test_suite!(Achievements, crate::mock::new_test_ext(), crate::mock::Test);
}
