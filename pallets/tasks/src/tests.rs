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

#[test]
fn create_task_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let title = b"Test Task".to_vec();
        let description = b"Test Description".to_vec();
        let priority = 2;
        let difficulty = 5;
        let reward = 100;
        let deadline = Some(1000);

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            title.clone(),
            description.clone(),
            priority,
            difficulty,
            reward,
            deadline,
        ));

        // 验证任务创建成功
        let task = Tasks::<Test>::get(1).unwrap();
        assert_eq!(task.title, title);
        assert_eq!(task.description, description);
        assert_eq!(task.priority, priority);
        assert_eq!(task.difficulty, difficulty);
        assert_eq!(task.reward, reward);
        assert_eq!(task.deadline, deadline);
    });
}

#[test]
fn search_tasks_by_keyword_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建测试任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Frontend Development".to_vec(),
            b"Build React components".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(2),
            b"Backend API".to_vec(),
            b"Create REST endpoints".to_vec(),
            3,
            7,
            200,
            Some(2000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Database Design".to_vec(),
            b"Design schema for React app".to_vec(),
            1,
            4,
            150,
            Some(1500),
        ));

        // 测试关键词搜索
        let search_params = TaskSearchParams::<Test> {
            keyword: b"React".to_vec(),
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 2); // 应该找到包含"React"的两个任务

        // 验证搜索结果
        assert!(results
            .iter()
            .any(|task| task.title == b"Frontend Development".to_vec()));
        assert!(results
            .iter()
            .any(|task| task.title == b"Database Design".to_vec()));
    });
}

#[test]
fn search_tasks_by_status_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建不同状态的任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description 1".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(2),
            b"Task 2".to_vec(),
            b"Description 2".to_vec(),
            3,
            7,
            200,
            Some(2000),
        ));

        // 改变第二个任务的状态
        assert_ok!(TasksPallet::change_task_status(
            RuntimeOrigin::signed(2),
            2,
            1, // InProgress
        ));

        // 测试按状态搜索
        let search_params = TaskSearchParams::<Test> {
            status: Some(0), // Pending
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
        assert_eq!(results[0].status, 0);

        // 测试搜索进行中的任务
        let search_params = TaskSearchParams::<Test> {
            status: Some(1), // InProgress
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 2);
        assert_eq!(results[0].status, 1);
    });
}

#[test]
fn search_tasks_by_difficulty_range_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建不同难度的任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Easy Task".to_vec(),
            b"Simple task".to_vec(),
            1,
            2,
            50,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Medium Task".to_vec(),
            b"Medium difficulty".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Hard Task".to_vec(),
            b"Challenging task".to_vec(),
            3,
            8,
            200,
            Some(1000),
        ));

        // 测试难度范围搜索
        let search_params = TaskSearchParams::<Test> {
            difficulty_range: Some((3, 6)),
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].difficulty, 5);
    });
}

#[test]
fn search_tasks_by_reward_range_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建不同奖励的任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Low Reward Task".to_vec(),
            b"Small reward".to_vec(),
            1,
            3,
            50,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"High Reward Task".to_vec(),
            b"Big reward".to_vec(),
            3,
            7,
            500,
            Some(1000),
        ));

        // 测试奖励范围搜索
        let search_params = TaskSearchParams::<Test> {
            reward_range: Some((100, 1000)),
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].reward, 500);
    });
}

#[test]
fn search_tasks_by_creator_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 不同用户创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task by User 1".to_vec(),
            b"Created by user 1".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(2),
            b"Task by User 2".to_vec(),
            b"Created by user 2".to_vec(),
            3,
            6,
            150,
            Some(1000),
        ));

        // 测试按创建者搜索
        let search_params = TaskSearchParams::<Test> {
            creator: Some(1),
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].creator, 1);
    });
}

#[test]
fn search_tasks_with_sorting_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建多个任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task A".to_vec(),
            b"Description A".to_vec(),
            1,
            3,
            50,
            Some(1000),
        ));

        System::set_block_number(2);
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task B".to_vec(),
            b"Description B".to_vec(),
            2,
            5,
            150,
            Some(1000),
        ));

        System::set_block_number(3);
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task C".to_vec(),
            b"Description C".to_vec(),
            3,
            7,
            100,
            Some(1000),
        ));

        // 测试按奖励降序排序
        let search_params = TaskSearchParams::<Test> {
            sort_by: TaskSortBy::Reward,
            sort_desc: true,
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].reward, 150); // 最高奖励
        assert_eq!(results[1].reward, 100);
        assert_eq!(results[2].reward, 50); // 最低奖励

        // 测试按难度升序排序
        let search_params = TaskSearchParams::<Test> {
            sort_by: TaskSortBy::Difficulty,
            sort_desc: false,
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results[0].difficulty, 3); // 最低难度
        assert_eq!(results[1].difficulty, 5);
        assert_eq!(results[2].difficulty, 7); // 最高难度
    });
}

#[test]
fn search_tasks_with_pagination_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建多个任务
        for i in 1..=5 {
            assert_ok!(TasksPallet::create_task(
                RuntimeOrigin::signed(1),
                format!("Task {}", i).into_bytes(),
                b"Description".to_vec(),
                1,
                3,
                100,
                Some(1000),
            ));
        }

        // 测试分页 - 第一页
        let search_params = TaskSearchParams::<Test> {
            page: 0,
            page_size: 2,
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params.clone());
        assert_eq!(results.len(), 2);

        // 测试分页 - 第二页
        let search_params = TaskSearchParams::<Test> {
            page: 1,
            page_size: 2,
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 2);

        // 测试计数功能
        let search_params = TaskSearchParams::<Test>::default();
        let total = TasksPallet::count_search_results(search_params);
        assert_eq!(total, 5);
    });
}

#[test]
fn quick_search_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建测试任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"React Development".to_vec(),
            b"Build UI components".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(2),
            b"Backend Service".to_vec(),
            b"React API integration".to_vec(),
            3,
            6,
            150,
            Some(1000),
        ));

        // 测试快速搜索
        let results = TasksPallet::quick_search(b"React".to_vec());
        assert_eq!(results.len(), 2);

        // 测试空关键词
        let results = TasksPallet::quick_search(b"".to_vec());
        assert_eq!(results.len(), 0);
    });
}

#[test]
fn get_task_statistics_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建不同状态的任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 1".to_vec(),
            b"Description 1".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Task 2".to_vec(),
            b"Description 2".to_vec(),
            3,
            7,
            200,
            Some(1000),
        ));

        // 改变任务状态
        assert_ok!(TasksPallet::change_task_status(
            RuntimeOrigin::signed(1),
            2,
            2, // Completed
        ));

        // 获取统计信息
        let stats = TasksPallet::get_task_statistics();
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.pending_count, 1);
        assert_eq!(stats.completed_count, 1);
        assert_eq!(stats.total_reward, 300);
        assert_eq!(stats.avg_difficulty, 6); // (5 + 7) / 2
    });
}

#[test]
fn search_tasks_combined_filters_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建多个任务用于复合搜索测试
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Frontend React Task".to_vec(),
            b"Build React components".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Backend API Task".to_vec(),
            b"Create React integration".to_vec(),
            3,
            7,
            200,
            Some(2000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(2),
            b"React Mobile App".to_vec(),
            b"Mobile React Native".to_vec(),
            2,
            6,
            150,
            Some(1500),
        ));

        // 测试复合搜索：关键词 + 优先级 + 难度范围
        let search_params = TaskSearchParams::<Test> {
            keyword: b"React".to_vec(),
            priority: Some(2),
            difficulty_range: Some((4, 6)),
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 2); // 应该匹配两个任务

        // 验证结果都符合条件
        for task in results {
            assert!(task.priority == 2);
            assert!(task.difficulty >= 4 && task.difficulty <= 6);
            // 检查标题或描述包含"React"
            let title_match = std::str::from_utf8(&task.title)
                .unwrap()
                .to_lowercase()
                .contains("react");
            let desc_match = std::str::from_utf8(&task.description)
                .unwrap()
                .to_lowercase()
                .contains("react");
            assert!(title_match || desc_match);
        }
    });
}

#[test]
fn search_tasks_unassigned_only_works() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        // 创建任务
        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"Unassigned Task".to_vec(),
            b"No one assigned".to_vec(),
            2,
            5,
            100,
            Some(1000),
        ));

        assert_ok!(TasksPallet::create_task(
            RuntimeOrigin::signed(1),
            b"To Be Assigned".to_vec(),
            b"Will assign later".to_vec(),
            3,
            6,
            150,
            Some(1000),
        ));

        // 分配第二个任务
        assert_ok!(TasksPallet::assign_task(
            RuntimeOrigin::signed(1),
            2,
            2, // 分配给用户2
        ));

        // 测试只搜索未分配的任务
        let search_params = TaskSearchParams::<Test> {
            unassigned_only: true,
            ..Default::default()
        };

        let results = TasksPallet::search_tasks(search_params);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
        assert!(results[0].assignee.is_none());
    });
}
