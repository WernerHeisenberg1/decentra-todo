const { ApiPromise, WsProvider } = require('@polkadot/api');

async function testCommunityVerificationAPI() {
    console.log('🔗 正在连接到 Substrate 节点...');
    
    try {
        // 连接到节点
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        
        console.log('✅ 已连接到区块链节点');
        console.log(`📦 链名: ${await api.rpc.system.chain()}`);
        console.log(`🔗 节点版本: ${await api.rpc.system.version()}`);
        
        // 检查区块高度
        const header = await api.rpc.chain.getHeader();
        console.log(`📊 当前区块高度: #${header.number}`);
        
        // 检查是否有 tasks pallet
        const metadata = await api.rpc.state.getMetadata();
        const pallets = metadata.asLatest.pallets;
        const tasksPallet = pallets.find(p => p.name.toString() === 'Tasks');
        
        if (tasksPallet) {
            console.log('✅ 找到 Tasks pallet');
            
            // 检查社区验证相关的存储项
            const verificationMethods = [
                'verificationVotes',
                'verificationStatus', 
                'verificationVoters'
            ];
            
            console.log('🔍 检查社区验证存储项:');
            verificationMethods.forEach(method => {
                try {
                    if (api.query.tasks[method]) {
                        console.log(`  ✅ ${method} - 可用`);
                    } else {
                        console.log(`  ❌ ${method} - 不可用`);
                    }
                } catch (e) {
                    console.log(`  ❌ ${method} - 错误: ${e.message}`);
                }
            });
            
            // 检查社区验证相关的交易方法
            const verificationTxs = [
                'submitVerificationVote',
                'completeVerification'
            ];
            
            console.log('🔍 检查社区验证交易方法:');
            verificationTxs.forEach(method => {
                try {
                    if (api.tx.tasks[method]) {
                        console.log(`  ✅ ${method} - 可用`);
                    } else {
                        console.log(`  ❌ ${method} - 不可用`);
                    }
                } catch (e) {
                    console.log(`  ❌ ${method} - 错误: ${e.message}`);
                }
            });
            
            // 检查事件类型
            console.log('🔍 检查社区验证事件:');
            const eventTypes = api.events.tasks;
            const verificationEvents = [
                'CommunityVerificationStarted',
                'VerificationVoteSubmitted', 
                'CommunityVerificationCompleted',
                'CommunityVerificationExpired'
            ];
            
            verificationEvents.forEach(eventName => {
                if (eventTypes[eventName]) {
                    console.log(`  ✅ ${eventName} - 可用`);
                } else {
                    console.log(`  ❌ ${eventName} - 不可用`);
                }
            });
            
        } else {
            console.log('❌ 未找到 Tasks pallet');
        }
        
        // 检查是否有 reputation pallet
        const reputationPallet = pallets.find(p => p.name.toString() === 'Reputation');
        if (reputationPallet) {
            console.log('✅ 找到 Reputation pallet');
        } else {
            console.log('❌ 未找到 Reputation pallet');
        }
        
        console.log('🎉 功能检查完成！');
        
    } catch (error) {
        console.error('❌ 测试失败:', error.message);
    }
}

// 运行测试
testCommunityVerificationAPI().then(() => {
    console.log('测试完成，正在退出...');
    process.exit(0);
}).catch(err => {
    console.error('测试出现异常:', err);
    process.exit(1);
}); 