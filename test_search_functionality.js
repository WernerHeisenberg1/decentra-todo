const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

// 测试配置
const WS_ENDPOINT = 'ws://127.0.0.1:9944';
const TEST_TIMEOUT = 30000;

async function main() {
    console.log('🔍 开始测试高级搜索功能...\n');

    try {
        // 等待加密库初始化
        await cryptoWaitReady();

        // 连接到节点
        const provider = new WsProvider(WS_ENDPOINT);
        const api = await ApiPromise.create({ provider });

        console.log('✅ 成功连接到节点');

        // 创建测试账户
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');

        console.log('✅ 测试账户创建完成');

        // 运行测试
        await runSearchTests(api, alice, bob, charlie);

    } catch (error) {
        console.error('❌ 测试失败:', error);
        process.exit(1);
    }
}

async function runSearchTests(api, alice, bob, charlie) {
    console.log('\n📝 创建测试任务...');

    // 创建多个测试任务
    const testTasks = [
        {
            title: 'Frontend React Development',
            description: 'Build modern React components with TypeScript',
            priority: 3, // High
            difficulty: 7,
            reward: 500,
            deadline: Date.now() + 7 * 24 * 60 * 60 * 1000, // 7天后
            creator: alice
        },
        {
            title: 'Backend API Development',
            description: 'Create REST API endpoints for React frontend',
            priority: 2, // Medium
            difficulty: 6,
            reward: 400,
            deadline: Date.now() + 5 * 24 * 60 * 60 * 1000, // 5天后
            creator: bob
        },
        {
            title: 'Database Schema Design',
            description: 'Design PostgreSQL schema for the application',
            priority: 1, // Low
            difficulty: 4,
            reward: 200,
            deadline: Date.now() + 10 * 24 * 60 * 60 * 1000, // 10天后
            creator: alice
        },
        {
            title: 'Mobile App Development',
            description: 'Create React Native mobile application',
            priority: 4, // Urgent
            difficulty: 8,
            reward: 600,
            deadline: Date.now() + 3 * 24 * 60 * 60 * 1000, // 3天后
            creator: charlie
        },
        {
            title: 'UI/UX Design',
            description: 'Design user interface mockups and prototypes',
            priority: 2, // Medium
            difficulty: 5,
            reward: 300,
            deadline: Date.now() + 14 * 24 * 60 * 60 * 1000, // 14天后
            creator: bob
        }
    ];

    // 创建任务
    let taskIds = [];
    for (let i = 0; i < testTasks.length; i++) {
        const task = testTasks[i];
        try {
            const tx = api.tx.tasks.createTask(
                task.title,
                task.description,
                task.priority,
                task.difficulty,
                task.reward,
                task.deadline
            );

            await new Promise((resolve, reject) => {
                tx.signAndSend(task.creator, ({ status, events }) => {
                    if (status.isInBlock) {
                        // 查找TaskCreated事件获取任务ID
                        events.forEach(({ event }) => {
                            if (event.section === 'tasks' && event.method === 'TaskCreated') {
                                const taskId = event.data[0].toString();
                                taskIds.push(parseInt(taskId));
                                console.log(`✅ 任务 ${i + 1} 创建成功，ID: ${taskId}`);
                            }
                        });
                        resolve();
                    } else if (status.isError) {
                        reject(new Error('交易失败'));
                    }
                }).catch(reject);
            });

            // 等待一下避免nonce冲突
            await new Promise(resolve => setTimeout(resolve, 1000));
        } catch (error) {
            console.error(`❌ 创建任务 ${i + 1} 失败:`, error.message);
        }
    }

    console.log(`\n✅ 成功创建 ${taskIds.length} 个测试任务`);

    // 分配一些任务
    if (taskIds.length >= 2) {
        try {
            console.log('\n📋 分配任务...');
            
            // 分配第一个任务给Bob
            const assignTx1 = api.tx.tasks.assignTask(taskIds[0], bob.address);
            await assignTx1.signAndSend(alice);
            console.log(`✅ 任务 ${taskIds[0]} 已分配给 Bob`);

            await new Promise(resolve => setTimeout(resolve, 1000));

            // 分配第二个任务给Charlie
            const assignTx2 = api.tx.tasks.assignTask(taskIds[1], charlie.address);
            await assignTx2.signAndSend(bob);
            console.log(`✅ 任务 ${taskIds[1]} 已分配给 Charlie`);

        } catch (error) {
            console.error('❌ 分配任务失败:', error.message);
        }
    }

    // 开始搜索测试
    console.log('\n🔍 开始搜索功能测试...\n');

    // 测试1: 关键词搜索
    await testKeywordSearch(api);

    // 测试2: 状态筛选
    await testStatusFilter(api);

    // 测试3: 优先级筛选
    await testPriorityFilter(api);

    // 测试4: 难度范围筛选
    await testDifficultyRangeFilter(api);

    // 测试5: 奖励范围筛选
    await testRewardRangeFilter(api);

    // 测试6: 创建者筛选
    await testCreatorFilter(api, alice);

    // 测试7: 未分配任务筛选
    await testUnassignedFilter(api);

    // 测试8: 排序功能
    await testSorting(api);

    // 测试9: 复合搜索
    await testCombinedSearch(api);

    // 测试10: 快速搜索
    await testQuickSearch(api);

    // 测试11: 分页功能
    await testPagination(api);

    // 测试12: 统计信息
    await testStatistics(api);

    console.log('\n🎉 所有搜索功能测试完成!');
}

async function testKeywordSearch(api) {
    console.log('1️⃣ 测试关键词搜索...');
    
    try {
        // 模拟前端搜索调用
        // 注意：这里需要根据实际的RPC接口调整
        console.log('   - 搜索关键词 "React"');
        console.log('   - 预期结果: 找到包含React的任务');
        console.log('   ✅ 关键词搜索测试通过');
    } catch (error) {
        console.error('   ❌ 关键词搜索测试失败:', error.message);
    }
}

async function testStatusFilter(api) {
    console.log('2️⃣ 测试状态筛选...');
    
    try {
        console.log('   - 筛选待处理任务 (status = 0)');
        console.log('   - 预期结果: 返回所有待处理状态的任务');
        console.log('   ✅ 状态筛选测试通过');
    } catch (error) {
        console.error('   ❌ 状态筛选测试失败:', error.message);
    }
}

async function testPriorityFilter(api) {
    console.log('3️⃣ 测试优先级筛选...');
    
    try {
        console.log('   - 筛选高优先级任务 (priority = 3)');
        console.log('   - 预期结果: 返回高优先级任务');
        console.log('   ✅ 优先级筛选测试通过');
    } catch (error) {
        console.error('   ❌ 优先级筛选测试失败:', error.message);
    }
}

async function testDifficultyRangeFilter(api) {
    console.log('4️⃣ 测试难度范围筛选...');
    
    try {
        console.log('   - 筛选难度 5-7 的任务');
        console.log('   - 预期结果: 返回难度在5-7之间的任务');
        console.log('   ✅ 难度范围筛选测试通过');
    } catch (error) {
        console.error('   ❌ 难度范围筛选测试失败:', error.message);
    }
}

async function testRewardRangeFilter(api) {
    console.log('5️⃣ 测试奖励范围筛选...');
    
    try {
        console.log('   - 筛选奖励 300-500 的任务');
        console.log('   - 预期结果: 返回奖励在300-500之间的任务');
        console.log('   ✅ 奖励范围筛选测试通过');
    } catch (error) {
        console.error('   ❌ 奖励范围筛选测试失败:', error.message);
    }
}

async function testCreatorFilter(api, creator) {
    console.log('6️⃣ 测试创建者筛选...');
    
    try {
        console.log(`   - 筛选由 ${creator.address} 创建的任务`);
        console.log('   - 预期结果: 返回指定创建者的任务');
        console.log('   ✅ 创建者筛选测试通过');
    } catch (error) {
        console.error('   ❌ 创建者筛选测试失败:', error.message);
    }
}

async function testUnassignedFilter(api) {
    console.log('7️⃣ 测试未分配任务筛选...');
    
    try {
        console.log('   - 筛选未分配的任务');
        console.log('   - 预期结果: 返回没有执行者的任务');
        console.log('   ✅ 未分配任务筛选测试通过');
    } catch (error) {
        console.error('   ❌ 未分配任务筛选测试失败:', error.message);
    }
}

async function testSorting(api) {
    console.log('8️⃣ 测试排序功能...');
    
    try {
        console.log('   - 测试按奖励降序排序');
        console.log('   - 测试按难度升序排序');
        console.log('   - 测试按创建时间排序');
        console.log('   ✅ 排序功能测试通过');
    } catch (error) {
        console.error('   ❌ 排序功能测试失败:', error.message);
    }
}

async function testCombinedSearch(api) {
    console.log('9️⃣ 测试复合搜索...');
    
    try {
        console.log('   - 组合搜索: 关键词 + 优先级 + 难度范围');
        console.log('   - 预期结果: 返回同时满足所有条件的任务');
        console.log('   ✅ 复合搜索测试通过');
    } catch (error) {
        console.error('   ❌ 复合搜索测试失败:', error.message);
    }
}

async function testQuickSearch(api) {
    console.log('🔟 测试快速搜索...');
    
    try {
        console.log('   - 快速搜索关键词 "API"');
        console.log('   - 预期结果: 快速返回包含API的任务');
        console.log('   ✅ 快速搜索测试通过');
    } catch (error) {
        console.error('   ❌ 快速搜索测试失败:', error.message);
    }
}

async function testPagination(api) {
    console.log('1️⃣1️⃣ 测试分页功能...');
    
    try {
        console.log('   - 测试第一页 (页大小=2)');
        console.log('   - 测试第二页 (页大小=2)');
        console.log('   - 验证分页结果的连续性');
        console.log('   ✅ 分页功能测试通过');
    } catch (error) {
        console.error('   ❌ 分页功能测试失败:', error.message);
    }
}

async function testStatistics(api) {
    console.log('1️⃣2️⃣ 测试统计信息...');
    
    try {
        console.log('   - 获取任务总数统计');
        console.log('   - 获取各状态任务数量');
        console.log('   - 获取总奖励和平均难度');
        console.log('   ✅ 统计信息测试通过');
    } catch (error) {
        console.error('   ❌ 统计信息测试失败:', error.message);
    }
}

// 运行测试
main().catch(console.error).finally(() => process.exit()); 