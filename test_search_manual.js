#!/usr/bin/env node

/**
 * 高级搜索功能手动测试脚本
 * 
 * 这个脚本提供了一系列测试用例来验证搜索功能的正确性
 * 可以在开发环境中运行来验证功能实现
 */

console.log('🔍 高级搜索功能测试套件\n');

// 模拟任务数据
const mockTasks = [
    {
        id: 1,
        title: 'Frontend React Development',
        description: 'Build modern React components with TypeScript',
        creator: 'alice',
        assignee: null,
        status: 0, // Pending
        priority: 3, // High
        difficulty: 7,
        reward: 500,
        deadline: Date.now() + 7 * 24 * 60 * 60 * 1000,
        created_at: Date.now() - 24 * 60 * 60 * 1000,
        updated_at: Date.now(),
    },
    {
        id: 2,
        title: 'Backend API Development',
        description: 'Create REST API endpoints for React frontend',
        creator: 'bob',
        assignee: 'charlie',
        status: 1, // InProgress
        priority: 2, // Medium
        difficulty: 6,
        reward: 400,
        deadline: Date.now() + 5 * 24 * 60 * 60 * 1000,
        created_at: Date.now() - 2 * 24 * 60 * 60 * 1000,
        updated_at: Date.now(),
    },
    {
        id: 3,
        title: 'Database Schema Design',
        description: 'Design PostgreSQL schema for the application',
        creator: 'alice',
        assignee: null,
        status: 0, // Pending
        priority: 1, // Low
        difficulty: 4,
        reward: 200,
        deadline: Date.now() + 10 * 24 * 60 * 60 * 1000,
        created_at: Date.now() - 5 * 24 * 60 * 60 * 1000,
        updated_at: Date.now(),
    },
    {
        id: 4,
        title: 'Mobile App Development',
        description: 'Create React Native mobile application',
        creator: 'charlie',
        assignee: null,
        status: 0, // Pending
        priority: 4, // Urgent
        difficulty: 8,
        reward: 600,
        deadline: Date.now() + 3 * 24 * 60 * 60 * 1000,
        created_at: Date.now() - 3 * 24 * 60 * 60 * 1000,
        updated_at: Date.now(),
    },
    {
        id: 5,
        title: 'UI/UX Design',
        description: 'Design user interface mockups and prototypes',
        creator: 'bob',
        assignee: 'dave',
        status: 2, // Completed
        priority: 2, // Medium
        difficulty: 5,
        reward: 300,
        deadline: Date.now() + 14 * 24 * 60 * 60 * 1000,
        created_at: Date.now() - 14 * 24 * 60 * 60 * 1000,
        updated_at: Date.now(),
    },
];

// 搜索函数实现 (模拟后端逻辑)
function searchTasks(searchParams) {
    let filteredTasks = [...mockTasks];

    // 关键词搜索
    if (searchParams.keyword) {
        filteredTasks = filteredTasks.filter(task => {
            const keyword = searchParams.keyword.toLowerCase();
            return task.title.toLowerCase().includes(keyword) ||
                   task.description.toLowerCase().includes(keyword);
        });
    }

    // 状态筛选
    if (searchParams.status !== undefined) {
        filteredTasks = filteredTasks.filter(task => task.status === searchParams.status);
    }

    // 优先级筛选
    if (searchParams.priority !== undefined) {
        filteredTasks = filteredTasks.filter(task => task.priority === searchParams.priority);
    }

    // 难度范围筛选
    if (searchParams.difficultyRange) {
        const [min, max] = searchParams.difficultyRange;
        filteredTasks = filteredTasks.filter(task => 
            task.difficulty >= min && task.difficulty <= max
        );
    }

    // 奖励范围筛选
    if (searchParams.rewardRange) {
        const [min, max] = searchParams.rewardRange;
        filteredTasks = filteredTasks.filter(task => 
            task.reward >= min && task.reward <= max
        );
    }

    // 创建者筛选
    if (searchParams.creator) {
        filteredTasks = filteredTasks.filter(task => 
            task.creator.toLowerCase().includes(searchParams.creator.toLowerCase())
        );
    }

    // 执行者筛选
    if (searchParams.assignee) {
        filteredTasks = filteredTasks.filter(task => 
            task.assignee && task.assignee.toLowerCase().includes(searchParams.assignee.toLowerCase())
        );
    }

    // 未分配筛选
    if (searchParams.unassignedOnly) {
        filteredTasks = filteredTasks.filter(task => !task.assignee);
    }

    // 排序
    if (searchParams.sortBy) {
        filteredTasks.sort((a, b) => {
            let aValue, bValue;
            
            switch (searchParams.sortBy) {
                case 'created_at':
                    aValue = a.created_at;
                    bValue = b.created_at;
                    break;
                case 'reward':
                    aValue = a.reward;
                    bValue = b.reward;
                    break;
                case 'difficulty':
                    aValue = a.difficulty;
                    bValue = b.difficulty;
                    break;
                case 'priority':
                    aValue = a.priority;
                    bValue = b.priority;
                    break;
                default:
                    aValue = a.created_at;
                    bValue = b.created_at;
            }

            if (searchParams.sortDesc) {
                return bValue - aValue;
            } else {
                return aValue - bValue;
            }
        });
    }

    return filteredTasks;
}

// 快速搜索函数
function quickSearch(keyword) {
    if (!keyword) return [];
    
    return mockTasks.filter(task => {
        const keywordLower = keyword.toLowerCase();
        return task.title.toLowerCase().includes(keywordLower) ||
               task.description.toLowerCase().includes(keywordLower);
    });
}

// 统计函数
function getTaskStatistics() {
    const totalTasks = mockTasks.length;
    const pendingCount = mockTasks.filter(t => t.status === 0).length;
    const inProgressCount = mockTasks.filter(t => t.status === 1).length;
    const completedCount = mockTasks.filter(t => t.status === 2).length;
    const totalReward = mockTasks.reduce((sum, task) => sum + task.reward, 0);
    const avgDifficulty = totalTasks > 0 ? 
        Math.round(mockTasks.reduce((sum, task) => sum + task.difficulty, 0) / totalTasks) : 0;

    return {
        totalTasks,
        pendingCount,
        inProgressCount,
        completedCount,
        totalReward,
        avgDifficulty,
    };
}

// 测试用例
const testCases = [
    {
        name: '关键词搜索 - "React"',
        params: { keyword: 'React' },
        expectedCount: 3,
    },
    {
        name: '状态筛选 - 待处理',
        params: { status: 0 },
        expectedCount: 3,
    },
    {
        name: '优先级筛选 - 高优先级',
        params: { priority: 3 },
        expectedCount: 1,
    },
    {
        name: '难度范围筛选 - 5-7',
        params: { difficultyRange: [5, 7] },
        expectedCount: 3,
    },
    {
        name: '奖励范围筛选 - 300-500',
        params: { rewardRange: [300, 500] },
        expectedCount: 3,
    },
    {
        name: '创建者筛选 - alice',
        params: { creator: 'alice' },
        expectedCount: 2,
    },
    {
        name: '未分配任务筛选',
        params: { unassignedOnly: true },
        expectedCount: 3,
    },
    {
        name: '复合搜索 - React + 待处理',
        params: { keyword: 'React', status: 0 },
        expectedCount: 2,
    },
    {
        name: '排序 - 按奖励降序',
        params: { sortBy: 'reward', sortDesc: true },
        expectedCount: 5,
        checkOrder: true,
    },
];

// 运行测试
function runTests() {
    console.log('📊 开始运行搜索功能测试...\n');

    let passedTests = 0;
    let totalTests = testCases.length;

    testCases.forEach((testCase, index) => {
        console.log(`${index + 1}. ${testCase.name}`);
        
        try {
            const results = searchTasks(testCase.params);
            
            // 检查数量
            if (results.length === testCase.expectedCount) {
                console.log(`   ✅ 通过 - 找到 ${results.length} 个结果`);
                
                // 检查排序
                if (testCase.checkOrder && results.length > 1) {
                    const isCorrectOrder = results[0].reward >= results[1].reward;
                    if (isCorrectOrder) {
                        console.log(`   ✅ 排序正确`);
                    } else {
                        console.log(`   ❌ 排序错误`);
                        return;
                    }
                }
                
                passedTests++;
            } else {
                console.log(`   ❌ 失败 - 期望 ${testCase.expectedCount} 个结果，实际得到 ${results.length} 个`);
                console.log(`   结果:`, results.map(r => r.title));
            }
        } catch (error) {
            console.log(`   ❌ 错误 - ${error.message}`);
        }
        
        console.log('');
    });

    // 测试快速搜索
    console.log('🔍 测试快速搜索功能...');
    const quickResults = quickSearch('API');
    if (quickResults.length === 1) {
        console.log('   ✅ 快速搜索通过');
        passedTests++;
        totalTests++;
    } else {
        console.log('   ❌ 快速搜索失败');
        totalTests++;
    }

    // 测试统计功能
    console.log('\n📈 测试统计功能...');
    const stats = getTaskStatistics();
    console.log('   统计结果:', stats);
    
    if (stats.totalTasks === 5 && stats.pendingCount === 3) {
        console.log('   ✅ 统计功能通过');
        passedTests++;
        totalTests++;
    } else {
        console.log('   ❌ 统计功能失败');
        totalTests++;
    }

    // 总结
    console.log('\n' + '='.repeat(50));
    console.log(`测试完成: ${passedTests}/${totalTests} 通过`);
    
    if (passedTests === totalTests) {
        console.log('🎉 所有测试通过！高级搜索功能工作正常。');
    } else {
        console.log('⚠️  部分测试失败，请检查实现。');
    }
    
    return passedTests === totalTests;
}

// 性能测试
function runPerformanceTest() {
    console.log('\n⚡ 性能测试...');
    
    // 创建大量测试数据
    const largeMockTasks = [];
    for (let i = 0; i < 1000; i++) {
        largeMockTasks.push({
            id: i + 1,
            title: `Task ${i + 1}`,
            description: `Description for task ${i + 1}`,
            creator: `user${i % 10}`,
            assignee: i % 3 === 0 ? null : `assignee${i % 5}`,
            status: i % 3,
            priority: (i % 4) + 1,
            difficulty: (i % 10) + 1,
            reward: (i % 10 + 1) * 100,
            deadline: Date.now() + (i % 30) * 24 * 60 * 60 * 1000,
            created_at: Date.now() - (i % 100) * 24 * 60 * 60 * 1000,
            updated_at: Date.now(),
        });
    }
    
    // 临时替换数据
    const originalTasks = mockTasks.slice();
    mockTasks.length = 0;
    mockTasks.push(...largeMockTasks);
    
    const startTime = Date.now();
    
    // 执行复杂搜索
    const results = searchTasks({
        keyword: 'Task',
        status: 0,
        difficultyRange: [5, 8],
        rewardRange: [300, 700],
        sortBy: 'reward',
        sortDesc: true
    });
    
    const endTime = Date.now();
    const duration = endTime - startTime;
    
    console.log(`   搜索 1000 个任务用时: ${duration}ms`);
    console.log(`   找到 ${results.length} 个结果`);
    
    if (duration < 100) {
        console.log('   ✅ 性能测试通过 (< 100ms)');
    } else {
        console.log('   ⚠️  性能可能需要优化');
    }
    
    // 恢复原始数据
    mockTasks.length = 0;
    mockTasks.push(...originalTasks);
}

// 运行所有测试
function main() {
    const success = runTests();
    runPerformanceTest();
    
    console.log('\n📝 测试建议:');
    console.log('1. 在真实环境中测试搜索功能');
    console.log('2. 验证前端组件的交互');
    console.log('3. 测试分页功能');
    console.log('4. 检查搜索性能');
    console.log('5. 验证错误处理');
    
    process.exit(success ? 0 : 1);
}

// 如果直接运行脚本
if (require.main === module) {
    main();
}

module.exports = {
    searchTasks,
    quickSearch,
    getTaskStatistics,
    mockTasks,
}; 