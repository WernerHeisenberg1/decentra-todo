use crate::{mock::*, types::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_achievement_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let name = b"First Task".to_vec();
        let description = b"Complete your first task".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_ok!(Achievements::create_achievement(
            RuntimeOrigin::root(),
            name.clone(),
            description.clone(),
            0u8,  // TaskCompletion type
            0u8,  // Common rarity
            0u8,  // CompleteTasksCount condition type
            1u32, // condition value
            metadata_uri.clone(),
        ));

        // Check that the achievement was created
        assert_eq!(Achievements::next_achievement_id(), 1);
        assert!(Achievements::achievements(0).is_some());

        // Check the event was emitted
        System::assert_last_event(
            Event::AchievementCreated {
                achievement_id: 0,
                name: name.clone(),
                achievement_type: 0u8,
                rarity: 0u8,
            }
            .into(),
        );
    });
}

#[test]
fn create_achievement_fails_with_name_too_long() {
    new_test_ext().execute_with(|| {
        let name = vec![b'a'; 65]; // Exceeds MaxNameLength (64)
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_noop!(
            Achievements::create_achievement(
                RuntimeOrigin::root(),
                name,
                description,
                0u8,  // TaskCompletion type
                0u8,  // Common rarity
                0u8,  // CompleteTasksCount condition type
                1u32, // condition value
                metadata_uri,
            ),
            Error::<Test>::NameTooLong
        );
    });
}

#[test]
fn unlock_achievement_on_task_completion() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create an achievement that requires 1 completed task
        let name = b"First Task".to_vec();
        let description = b"Complete your first task".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_ok!(Achievements::create_achievement(
            RuntimeOrigin::root(),
            name.clone(),
            description,
            0u8,  // TaskCompletion type
            0u8,  // Common rarity
            0u8,  // CompleteTasksCount condition type
            1u32, // condition value
            metadata_uri,
        ));

        let user = 1u64;

        // Initially user has no achievements
        assert!(!Achievements::user_achievements(&user, 0).is_some());

        // Simulate task completion
        assert_ok!(Achievements::on_task_completed(&user, 1, 5));

        // Trigger achievement check (this ensures the achievement system works)
        assert_ok!(Achievements::check_and_unlock_achievements(
            RuntimeOrigin::signed(user),
            user
        ));

        // Check that the achievement was unlocked
        assert!(Achievements::user_achievements(&user, 0).is_some());

        // Check the event was emitted
        System::assert_has_event(
            Event::AchievementUnlocked {
                user,
                achievement_id: 0,
                name: name.clone(),
                rarity: 0u8,
            }
            .into(),
        );
    });
}

#[test]
fn update_achievement_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create an achievement first
        let name = b"Test Achievement".to_vec();
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_ok!(Achievements::create_achievement(
            RuntimeOrigin::root(),
            name,
            description,
            0u8,  // TaskCompletion type
            0u8,  // Common rarity
            0u8,  // CompleteTasksCount condition type
            1u32, // condition value
            metadata_uri,
        ));

        let new_name = b"Updated Achievement".to_vec();

        assert_ok!(Achievements::update_achievement(
            RuntimeOrigin::root(),
            0,
            Some(new_name.clone()),
            None,
            None,
        ));

        let achievement = Achievements::achievements(0).unwrap();
        assert_eq!(achievement.name.to_vec(), new_name);
    });
}

#[test]
fn deactivate_achievement_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create an achievement first
        let name = b"Test Achievement".to_vec();
        let description = b"Test description".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_ok!(Achievements::create_achievement(
            RuntimeOrigin::root(),
            name,
            description,
            0u8,  // TaskCompletion type
            0u8,  // Common rarity
            0u8,  // CompleteTasksCount condition type
            1u32, // condition value
            metadata_uri,
        ));

        assert_ok!(Achievements::deactivate_achievement(
            RuntimeOrigin::root(),
            0
        ));

        let achievement = Achievements::achievements(0).unwrap();
        assert!(!achievement.is_active);
    });
}

#[test]
fn consecutive_task_completion_achievement() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // Create an achievement that requires 3 consecutive completions
        let name = b"Triple Combo".to_vec();
        let description = b"Complete 3 tasks in a row".to_vec();
        let metadata_uri = b"https://example.com/metadata/1".to_vec();

        assert_ok!(Achievements::create_achievement(
            RuntimeOrigin::root(),
            name.clone(),
            description,
            0u8,  // TaskCompletion type
            1u8,  // Rare rarity
            2u8,  // ConsecutiveTaskCompletion condition type
            3u32, // condition value
            metadata_uri,
        ));

        let user = 1u64;

        // Complete 2 tasks - should not unlock yet
        assert_ok!(Achievements::on_task_completed(&user, 1, 5));
        assert_ok!(Achievements::on_task_completed(&user, 2, 5));

        // Trigger achievement check after 2 tasks
        assert_ok!(Achievements::check_and_unlock_achievements(
            RuntimeOrigin::signed(user),
            user
        ));
        assert!(!Achievements::user_achievements(&user, 0).is_some());

        // Complete 3rd task - should unlock achievement
        assert_ok!(Achievements::on_task_completed(&user, 3, 5));

        // Trigger achievement check after 3 tasks
        assert_ok!(Achievements::check_and_unlock_achievements(
            RuntimeOrigin::signed(user),
            user
        ));
        assert!(Achievements::user_achievements(&user, 0).is_some());

        // Check the event was emitted
        System::assert_has_event(
            Event::AchievementUnlocked {
                user,
                achievement_id: 0,
                name: name.clone(),
                rarity: 1u8, // Rare
            }
            .into(),
        );
    });
}

#[test]
fn task_cancellation_resets_consecutive_completions() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let user = 1u64;

        // Complete 2 tasks
        assert_ok!(Achievements::on_task_completed(&user, 1, 5));
        assert_ok!(Achievements::on_task_completed(&user, 2, 5));

        // Cancel a task - should reset consecutive completions
        assert_ok!(Achievements::on_task_cancelled(&user, 3));

        let stats = Achievements::user_stats(&user);
        assert_eq!(stats.consecutive_completions, 0);
    });
}
