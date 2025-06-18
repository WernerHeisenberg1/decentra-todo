use super::*;
use crate::mock::*;
use crate::types::{Priority, TaskStatus};
use frame_support::{assert_noop, assert_ok};

// Type alias for the pallet
type TasksPallet = crate::Pallet<Test>;

#[test]
fn test_create_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 验证任务创建
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.creator, 1);
        assert_eq!(task.title, b"Test Task".to_vec());
        assert_eq!(task.description, b"Test Description".to_vec());
        assert_eq!(task.status, TaskStatus::Pending as u8);
        assert_eq!(task.priority, Priority::Medium as u8);
        assert_eq!(task.difficulty, 5);
        assert_eq!(task.reward, 100);
        assert_eq!(task.assignee, None);
    });
}

#[test]
fn test_create_task_with_invalid_difficulty() {
    new_test_ext().execute_with(|| {
        // 测试无效的难度值
        assert_noop!(
            TasksPallet::create_task(
                RuntimeOrigin::signed(1),
                b"Test Task".to_vec(),
                b"Test Description".to_vec(),
                Priority::Medium as u8,
                0, // 无效难度
                100,
                None,
            ),
            Error::<Test>::InvalidDifficulty
        );

        assert_noop!(
            TasksPallet::create_task(
                RuntimeOrigin::signed(1),
                b"Test Task".to_vec(),
                b"Test Description".to_vec(),
                Priority::Medium as u8,
                11, // 无效难度
                100,
                None,
            ),
            Error::<Test>::InvalidDifficulty
        );
    });
}

#[test]
fn test_update_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 更新任务
        assert_ok!(TasksPallet::update_task(
            RuntimeOrigin::signed(1),
            0,
            Some(b"Updated Task".to_vec()),
            Some(b"Updated Description".to_vec()),
            Some(Priority::High as u8),
            Some(7),
            Some(200),
            Some(Some(100)),
        ));

        // 验证任务更新
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.title, b"Updated Task".to_vec());
        assert_eq!(task.description, b"Updated Description".to_vec());
        assert_eq!(task.priority, Priority::High as u8);
        assert_eq!(task.difficulty, 7);
        assert_eq!(task.reward, 200);
        assert_eq!(task.deadline, Some(100));
    });
}

#[test]
fn test_update_task_unauthorized() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 尝试用其他用户更新任务
        assert_noop!(
            TasksPallet::update_task(
                RuntimeOrigin::signed(2),
                0,
                Some(b"Updated Task".to_vec()),
                None,
                None,
                None,
                None,
                None,
            ),
            Error::<Test>::NotAuthorized
        );
    });
}

#[test]
fn test_change_task_status() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 分配任务
        assert_ok!(TasksPallet::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 更改任务状态
        assert_ok!(TasksPallet::change_task_status(
            RuntimeOrigin::signed(2),
            0,
            TaskStatus::InProgress as u8,
        ));

        // 验证状态更改
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.status, TaskStatus::InProgress as u8);
    });
}

#[test]
fn test_invalid_status_transition() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 尝试无效的状态转换
        assert_noop!(
            TasksPallet::change_task_status(
                RuntimeOrigin::signed(1),
                0,
                TaskStatus::Completed as u8, // 直接从Pending跳到Completed
            ),
            Error::<Test>::InvalidStatusTransition
        );
    });
}

#[test]
fn test_assign_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 分配任务
        assert_ok!(TasksPallet::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 验证任务分配
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.assignee, Some(2));
    });
}

#[test]
fn test_assign_task_to_self() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 尝试分配给自己
        assert_noop!(
            TasksPallet::assign_task(RuntimeOrigin::signed(1), 0, 1),
            Error::<Test>::CannotAssignToSelf
        );
    });
}

#[test]
fn test_unassign_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 分配任务
        assert_ok!(TasksPallet::assign_task(RuntimeOrigin::signed(1), 0, 2));

        // 取消分配
        assert_ok!(TasksPallet::unassign_task(RuntimeOrigin::signed(1), 0));

        // 验证任务取消分配
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.assignee, None);
    });
}

#[test]
fn test_delete_task() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 删除任务
        assert_ok!(TasksPallet::delete_task(RuntimeOrigin::signed(1), 0));

        // 验证任务删除
        assert_eq!(crate::Tasks::<Test>::get(0), None);
    });
}

#[test]
fn test_task_queries() {
    new_test_ext().execute_with(|| {
        // 创建不同优先级的任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"High Priority Task".to_vec(),
            b"Description".to_vec(),
            Priority::High as u8,
            5,
            100,
            None,
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Medium Priority Task".to_vec(),
            b"Description".to_vec(),
            Priority::Medium as u8,
            3,
            50,
            None,
        ));

        // 测试按状态查询
        let pending_tasks = TasksPallet::get_tasks_by_status(TaskStatus::Pending as u8);
        assert_eq!(pending_tasks.len(), 2);

        // 测试按优先级查询
        let high_priority_tasks = TasksPallet::get_tasks_by_priority(Priority::High as u8);
        assert_eq!(high_priority_tasks.len(), 1);
        assert_eq!(high_priority_tasks[0].priority, Priority::High as u8);
    });
}

#[test]
fn test_task_helper_methods() {
    new_test_ext().execute_with(|| {
        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Test Task".to_vec(),
            b"Test Description".to_vec(),
            Priority::Medium as u8,
            5,
            100,
            None,
        ));

        // 验证任务创建者
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.creator, 1);

        // 验证任务初始状态
        assert_eq!(task.status, TaskStatus::Pending as u8);

        // 分配任务
        assert_ok!(TasksPallet::assign_task(RuntimeOrigin::signed(1), 0, 2));
        let task = crate::Tasks::<Test>::get(0).unwrap();
        assert_eq!(task.assignee, Some(2));
    });
}
