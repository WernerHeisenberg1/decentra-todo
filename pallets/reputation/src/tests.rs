use super::*;
use crate::{mock::*, types::*};
use frame_support::{assert_noop, assert_ok};

#[test]
fn evaluate_task_works() {
    new_test_ext().execute_with(|| {
        // 设置任务为已完成状态
        assert_ok!(Reputation::set_test_task_status(
            RuntimeOrigin::root(),
            0,
            2
        ));

        // 评价任务
        assert_ok!(Reputation::evaluate_task(
            RuntimeOrigin::signed(1),
            0,
            4, // Excellent
            b"Great work!".to_vec(),
        ));

        // 检查评价是否保存
        let evaluation = Reputation::task_evaluations(0, 1).unwrap();
        assert_eq!(evaluation.task_id, 0);
        assert_eq!(evaluation.assignee, 2);
        assert_eq!(evaluation.evaluator, 1);
        assert_eq!(evaluation.rating, TaskRating::Excellent);

        // 检查用户声誉是否更新
        let reputation = Reputation::user_reputation(2);
        assert_eq!(reputation.completed_tasks, 1);
        assert_eq!(reputation.total_ratings, 1);
        assert!(reputation.total_score > 0);
    });
}

#[test]
fn cannot_evaluate_own_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(Tasks::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            2,    // Medium priority
            5,    // Difficulty
            100,  // Reward
            None, // No deadline
        ));

        // 分配任务给其他人（而不是自己）
        assert_ok!(Tasks::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 任务状态: Pending -> InProgress -> Completed
        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            1, // InProgress
        ));

        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            2, // Completed
        ));

        // 设置任务为已完成状态
        assert_ok!(Reputation::set_test_task_status(
            RuntimeOrigin::root(),
            0,
            2
        ));

        // 创建者评价执行者的任务 - 这个应该成功
        assert_ok!(Reputation::evaluate_task(
            RuntimeOrigin::signed(1),
            0,
            4, // Excellent
            b"Great work!".to_vec(),
        ));

        // 执行者不能评价自己完成的任务（但由于我们的逻辑是创建者评价执行者，所以这个测试需要调整）
        // 这个测试实际上应该测试执行者试图成为评价者的情况
        assert_noop!(
            Reputation::evaluate_task(
                RuntimeOrigin::signed(2), // 执行者尝试评价
                0,
                4, // Excellent
                b"Great work!".to_vec(),
            ),
            Error::<Test>::NotAuthorizedToEvaluate // 因为执行者不是创建者
        );
    });
}

#[test]
fn cannot_evaluate_incomplete_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(Tasks::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            2,    // Medium priority
            5,    // Difficulty
            100,  // Reward
            None, // No deadline
        ));

        // 分配任务
        assert_ok!(Tasks::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 设置任务为未完成状态（默认为0，但显式设置）
        assert_ok!(Reputation::set_test_task_status(
            RuntimeOrigin::root(),
            0,
            0
        ));

        // 尝试评价未完成的任务应该失败
        assert_noop!(
            Reputation::evaluate_task(
                RuntimeOrigin::signed(1),
                0,
                4, // Excellent
                b"Great work!".to_vec(),
            ),
            Error::<Test>::TaskNotCompleted
        );
    });
}

#[test]
fn cannot_evaluate_twice() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(Tasks::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            2,    // Medium priority
            5,    // Difficulty
            100,  // Reward
            None, // No deadline
        ));

        // 分配任务
        assert_ok!(Tasks::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 任务状态: Pending -> InProgress -> Completed
        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            1, // InProgress
        ));

        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            2, // Completed
        ));

        // 设置任务为已完成状态
        assert_ok!(Reputation::set_test_task_status(
            RuntimeOrigin::root(),
            0,
            2
        ));

        // 第一次评价
        assert_ok!(Reputation::evaluate_task(
            RuntimeOrigin::signed(1),
            0,
            4, // Excellent
            b"Great work!".to_vec(),
        ));

        // 第二次评价应该失败
        assert_noop!(
            Reputation::evaluate_task(
                RuntimeOrigin::signed(1),
                0,
                3, // Good
                b"Still good".to_vec(),
            ),
            Error::<Test>::TaskAlreadyEvaluated
        );
    });
}

#[test]
fn reputation_levels_work() {
    new_test_ext().execute_with(|| {
        assert_eq!(ReputationLevel::from_score(50), ReputationLevel::Newcomer);
        assert_eq!(
            ReputationLevel::from_score(200),
            ReputationLevel::Apprentice
        );
        assert_eq!(ReputationLevel::from_score(500), ReputationLevel::Skilled);
        assert_eq!(ReputationLevel::from_score(800), ReputationLevel::Expert);
        assert_eq!(ReputationLevel::from_score(1200), ReputationLevel::Master);
        assert_eq!(
            ReputationLevel::from_score(2000),
            ReputationLevel::Legendary
        );
    });
}

#[test]
fn reputation_calculation_works() {
    new_test_ext().execute_with(|| {
        // 创建高难度任务
        assert_ok!(Tasks::create_task(
            RuntimeOrigin::signed(1),
            b"Hard Task".to_vec(),
            b"Very difficult task".to_vec(),
            4,    // Urgent priority
            8,    // High difficulty
            200,  // High reward
            None, // No deadline
        ));

        // 分配和完成任务
        assert_ok!(Tasks::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 任务状态: Pending -> InProgress -> Completed
        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            1, // InProgress
        ));

        assert_ok!(Tasks::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            2, // Completed
        ));

        // 设置任务为已完成状态
        assert_ok!(Reputation::set_test_task_status(
            RuntimeOrigin::root(),
            0,
            2
        ));

        // 给予完美评分
        assert_ok!(Reputation::evaluate_task(
            RuntimeOrigin::signed(1),
            0,
            5, // Perfect
            b"Outstanding work!".to_vec(),
        ));

        let reputation = Reputation::user_reputation(2);

        // 检查分数计算
        // 基础分数(10) * 1.5(完美评分) + 难度奖励(8 * 2) = 15 + 16 = 31
        let expected_score =
            (BaseReputationScore::get() * 150 / 100) + (8 * DifficultyBonus::get());
        assert_eq!(reputation.total_score, expected_score);

        // 检查统计信息
        assert_eq!(reputation.completed_tasks, 1);
        assert_eq!(reputation.total_ratings, 1);
        assert_eq!(reputation.rating_sum, 5);
        assert_eq!(reputation.average_rating(), 5.0);
    });
}

#[test]
fn task_cancellation_penalty_works() {
    new_test_ext().execute_with(|| {
        // 先给用户一些声誉分数
        assert_ok!(Reputation::update_reputation(
            RuntimeOrigin::root(),
            2,
            100, // 增加100分
        ));

        let initial_reputation = Reputation::user_reputation(2);
        assert_eq!(initial_reputation.total_score, 100);

        // 模拟任务取消
        assert_ok!(Reputation::handle_task_cancellation(&2, 0));

        let final_reputation = Reputation::user_reputation(2);
        assert_eq!(
            final_reputation.total_score,
            100 - CancellationPenalty::get()
        );
        assert_eq!(final_reputation.cancelled_tasks, 1);
    });
}

#[test]
fn user_reputation_stats_work() {
    new_test_ext().execute_with(|| {
        let mut reputation = UserReputation::<Test>::default();

        // 测试完成率计算
        assert_eq!(reputation.completion_rate(), 0.0);

        reputation.completed_tasks = 8;
        reputation.cancelled_tasks = 2;
        assert_eq!(reputation.completion_rate(), 0.8);

        // 测试平均评分计算
        assert_eq!(reputation.average_rating(), 0.0);

        reputation.total_ratings = 3;
        reputation.rating_sum = 12; // 平均4分
        assert_eq!(reputation.average_rating(), 4.0);
    });
}
