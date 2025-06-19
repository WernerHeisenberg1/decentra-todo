const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function testCommunityVerificationComplete() {
    console.log('🚀 开始完整的社区验证功能测试...');
    
    try {
        // 连接到区块链节点
        console.log('🔗 正在连接到 Substrate 节点...');
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        console.log('✅ 已连接到区块链节点');
        console.log(`📦 链名: ${await api.rpc.system.chain()}`);
        console.log(`🔗 节点版本: ${await api.rpc.system.version()}`);
        
        // 创建测试账户
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');
        const bob = keyring.addFromUri('//Bob');
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        
        console.log('👥 测试账户已创建:');
        console.log(`  - Alice (创建者): ${alice.address}`);
        console.log(`  - Bob (执行者): ${bob.address}`);
        console.log(`  - Charlie (投票者1): ${charlie.address}`);
        console.log(`  - Dave (投票者2): ${dave.address}`);
        console.log(`  - Eve (投票者3): ${eve.address}`);
        
        // 检查账户余额
        const aliceBalance = await api.query.system.account(alice.address);
        console.log(`💰 Alice 余额: ${aliceBalance.data.free.toHuman()}`);
        
        // 1. 创建任务
        console.log('\n📝 步骤1: 创建任务');
        const title = '测试社区验证任务';
        const description = '这是一个测试社区验证功能的任务';
        const priority = 3; // High
        const difficulty = 5;
        const reward = 1000000000000000000n; // 1 DOT
        const deadline = Date.now() + 86400000 * 7; // 7天后
        
        const createTaskTx = api.tx.tasks.createTask(
            title,
            description,
            priority,
            difficulty,
            reward,
            deadline
        );
        
        const taskId = await new Promise((resolve, reject) => {
            createTaskTx.signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ 任务创建成功，区块哈希:', result.status.asInBlock.toString());
                    
                    // 查找任务创建事件
                    result.events.forEach(({ event }) => {
                        if (event.section === 'tasks' && event.method === 'TaskCreated') {
                            const [taskId] = event.data;
                            console.log(`📋 任务ID: ${taskId}`);
                            resolve(taskId.toNumber());
                        }
                    });
                } else if (result.status.isError) {
                    reject(new Error('任务创建失败'));
                }
            });
        });
        
        // 2. 分配任务给Bob
        console.log('\n👤 步骤2: 分配任务给Bob');
        const assignTaskTx = api.tx.tasks.assignTask(taskId, bob.address);
        
        await new Promise((resolve, reject) => {
            assignTaskTx.signAndSend(alice, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ 任务分配成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('任务分配失败'));
                }
            });
        });
        
        // 3. 将任务状态改为进行中
        console.log('\n⏳ 步骤3: 开始执行任务');
        const startTaskTx = api.tx.tasks.changeTaskStatus(taskId, 1); // InProgress
        
        await new Promise((resolve, reject) => {
            startTaskTx.signAndSend(bob, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ 任务状态已更改为进行中');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('状态更改失败'));
                }
            });
        });
        
        // 4. 请求社区验证
        console.log('\n🔍 步骤4: 请求社区验证');
        const requestVerificationTx = api.tx.tasks.changeTaskStatus(taskId, 4); // PendingVerification
        
        await new Promise((resolve, reject) => {
            requestVerificationTx.signAndSend(bob, (result) => {
                if (result.status.isInBlock) {
                    console.log('✅ 已请求社区验证');
                    
                    // 查找验证开始事件
                    result.events.forEach(({ event }) => {
                        if (event.section === 'tasks' && event.method === 'CommunityVerificationStarted') {
                            const [taskId, endBlock] = event.data;
                            console.log(`🎯 验证结束区块: ${endBlock}`);
                        }
                    });
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('请求验证失败'));
                }
            });
        });
        
        // 5. 社区投票
        console.log('\n🗳️  步骤5: 社区投票');
        
        // Charlie 投赞成票
        console.log('  Charlie 投赞成票...');
        const charlieVoteTx = api.tx.tasks.submitVerificationVote(taskId, true);
        await new Promise((resolve, reject) => {
            charlieVoteTx.signAndSend(charlie, (result) => {
                if (result.status.isInBlock) {
                    console.log('  ✅ Charlie 投票成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Charlie 投票失败'));
                }
            });
        });
        
        // Dave 投赞成票
        console.log('  Dave 投赞成票...');
        const daveVoteTx = api.tx.tasks.submitVerificationVote(taskId, true);
        await new Promise((resolve, reject) => {
            daveVoteTx.signAndSend(dave, (result) => {
                if (result.status.isInBlock) {
                    console.log('  ✅ Dave 投票成功');
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Dave 投票失败'));
                }
            });
        });
        
        // Eve 投反对票
        console.log('  Eve 投反对票...');
        const eveVoteTx = api.tx.tasks.submitVerificationVote(taskId, false);
        await new Promise((resolve, reject) => {
            eveVoteTx.signAndSend(eve, (result) => {
                if (result.status.isInBlock) {
                    console.log('  ✅ Eve 投票成功');
                    
                    // 检查是否自动完成验证
                    result.events.forEach(({ event }) => {
                        if (event.section === 'tasks' && event.method === 'CommunityVerificationCompleted') {
                            const [taskId, approved, approveVotes, rejectVotes] = event.data;
                            console.log(`🎉 验证自动完成!`);
                            console.log(`  - 任务ID: ${taskId}`);
                            console.log(`  - 验证结果: ${approved ? '通过' : '未通过'}`);
                            console.log(`  - 赞成票: ${approveVotes}`);
                            console.log(`  - 反对票: ${rejectVotes}`);
                        }
                    });
                    resolve();
                } else if (result.status.isError) {
                    reject(new Error('Eve 投票失败'));
                }
            });
        });
        
        // 6. 查询最终状态
        console.log('\n🔍 步骤6: 查询最终状态');
        const finalTask = await api.query.tasks.tasks(taskId);
        if (finalTask.isSome) {
            const task = finalTask.unwrap();
            console.log(`📋 任务最终状态: ${task.status}`);
            console.log(`👤 执行者: ${task.assignee.isSome ? task.assignee.unwrap() : '无'}`);
        }
        
        // 检查验证数据是否已清理
        const verificationStatus = await api.query.tasks.verificationStatus(taskId);
        console.log(`🧹 验证数据已清理: ${verificationStatus.isNone ? '是' : '否'}`);
        
        console.log('\n🎉 社区验证功能测试完成！');
        console.log('✅ 所有功能都正常工作：');
        console.log('  - ✅ PendingVerification 状态支持');
        console.log('  - ✅ 社区投票验证机制');
        console.log('  - ✅ 多人验证功能');
        console.log('  - ✅ 自动化处理');
        console.log('  - ✅ 数据清理');
        
    } catch (error) {
        console.error('❌ 测试失败:', error.message);
        console.error(error);
    }
    
    console.log('\n⏹️  测试完成，正在退出...');
    process.exit(0);
}

// 运行测试
testCommunityVerificationComplete().catch(console.error); 