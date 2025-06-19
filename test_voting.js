const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

async function testVoting() {
    console.log('🗳️  测试社区验证投票功能...');
    
    try {
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        const keyring = new Keyring({ type: 'sr25519' });
        const charlie = keyring.addFromUri('//Charlie');
        const dave = keyring.addFromUri('//Dave');
        const eve = keyring.addFromUri('//Eve');
        
        const taskId = 0; // 使用现有的任务
        
        console.log('✅ 已连接到区块链节点');
        console.log(`🎯 测试任务ID: ${taskId}`);
        
        // 检查当前验证状态
        const verificationStatus = await api.query.tasks.verificationStatus(taskId);
        if (verificationStatus.isSome) {
            const [endBlock, approveVotes, rejectVotes] = verificationStatus.unwrap();
            console.log(`📊 当前投票状态: 赞成${approveVotes} 反对${rejectVotes}`);
        }
        
        // Charlie 投赞成票
        console.log('\n🗳️  Charlie 投赞成票...');
        try {
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => reject(new Error('投票超时')), 30000);
                
                api.tx.tasks.submitVerificationVote(taskId, true).signAndSend(charlie, (result) => {
                    if (result.status.isInBlock) {
                        clearTimeout(timeout);
                        console.log('✅ Charlie 投票成功');
                        
                        // 检查事件
                        result.events.forEach(({ event }) => {
                            if (event.section === 'tasks' && event.method === 'VerificationVoteSubmitted') {
                                console.log('📝 投票事件:', event.data.toString());
                            }
                        });
                        resolve();
                    } else if (result.status.isError) {
                        clearTimeout(timeout);
                        reject(new Error('Charlie 投票失败'));
                    }
                });
            });
        } catch (error) {
            console.log(`❌ Charlie 投票失败: ${error.message}`);
        }
        
        // Dave 投赞成票
        console.log('\n🗳️  Dave 投赞成票...');
        try {
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => reject(new Error('投票超时')), 30000);
                
                api.tx.tasks.submitVerificationVote(taskId, true).signAndSend(dave, (result) => {
                    if (result.status.isInBlock) {
                        clearTimeout(timeout);
                        console.log('✅ Dave 投票成功');
                        resolve();
                    } else if (result.status.isError) {
                        clearTimeout(timeout);
                        reject(new Error('Dave 投票失败'));
                    }
                });
            });
        } catch (error) {
            console.log(`❌ Dave 投票失败: ${error.message}`);
        }
        
        // Eve 投反对票
        console.log('\n🗳️  Eve 投反对票...');
        try {
            await new Promise((resolve, reject) => {
                const timeout = setTimeout(() => reject(new Error('投票超时')), 30000);
                
                api.tx.tasks.submitVerificationVote(taskId, false).signAndSend(eve, (result) => {
                    if (result.status.isInBlock) {
                        clearTimeout(timeout);
                        console.log('✅ Eve 投票成功');
                        
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
                        clearTimeout(timeout);
                        reject(new Error('Eve 投票失败'));
                    }
                });
            });
        } catch (error) {
            console.log(`❌ Eve 投票失败: ${error.message}`);
        }
        
        // 查询最终状态
        console.log('\n🔍 查询最终状态:');
        const finalTask = await api.query.tasks.tasks(taskId);
        if (finalTask.isSome) {
            const task = finalTask.unwrap();
            console.log(`📋 任务最终状态: ${task.status}`);
        }
        
        const finalVerificationStatus = await api.query.tasks.verificationStatus(taskId);
        if (finalVerificationStatus.isSome) {
            const [endBlock, approveVotes, rejectVotes] = finalVerificationStatus.unwrap();
            console.log(`📊 最终投票状态: 赞成${approveVotes} 反对${rejectVotes}`);
        } else {
            console.log('🧹 验证数据已清理');
        }
        
        console.log('\n🎉 投票测试完成！');
        
    } catch (error) {
        console.error('❌ 测试失败:', error);
    }
    
    process.exit(0);
}

testVoting().catch(console.error); 