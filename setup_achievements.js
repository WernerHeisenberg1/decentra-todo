const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { u8aToHex } = require('@polkadot/util');

// 成就类型映射: 0=TaskCompletion, 1=Reputation, 2=Community, 3=Special
// 稀有度映射: 0=Common, 1=Rare, 2=Epic, 3=Legendary
// 条件类型映射: 0=CompleteTasksCount, 1=ReachReputationScore, 2=ConsecutiveTaskCompletion, 
//             3=AverageRating, 4=CommunityVerificationCount, 5=CreateTasksCount, 6=CompleteTaskInTime

// 预定义成就数据
const achievements = [
    {
        name: "初出茅庐",
        description: "完成第一个任务",
        achievementType: 0, // TaskCompletion
        rarity: 0, // Common
        conditionType: 0, // CompleteTasksCount
        conditionValue: 1,
        metadataUri: "https://example.com/achievements/first-task.json"
    },
    {
        name: "勤奋工作者",
        description: "完成10个任务",
        achievementType: 0, // TaskCompletion
        rarity: 1, // Rare
        conditionType: 0, // CompleteTasksCount
        conditionValue: 10,
        metadataUri: "https://example.com/achievements/10-tasks.json"
    },
    {
        name: "任务大师",
        description: "完成100个任务",
        achievementType: 0, // TaskCompletion
        rarity: 2, // Epic
        conditionType: 0, // CompleteTasksCount
        conditionValue: 100,
        metadataUri: "https://example.com/achievements/100-tasks.json"
    },
    {
        name: "传奇工作者",
        description: "完成1000个任务",
        achievementType: 0, // TaskCompletion
        rarity: 3, // Legendary
        conditionType: 0, // CompleteTasksCount
        conditionValue: 1000,
        metadataUri: "https://example.com/achievements/1000-tasks.json"
    },
    {
        name: "连击新手",
        description: "连续完成3个任务",
        achievementType: 0, // TaskCompletion
        rarity: 0, // Common
        conditionType: 2, // ConsecutiveTaskCompletion
        conditionValue: 3,
        metadataUri: "https://example.com/achievements/3-streak.json"
    },
    {
        name: "连击高手",
        description: "连续完成10个任务",
        achievementType: 0, // TaskCompletion
        rarity: 1, // Rare
        conditionType: 2, // ConsecutiveTaskCompletion
        conditionValue: 10,
        metadataUri: "https://example.com/achievements/10-streak.json"
    },
    {
        name: "任务发布者",
        description: "创建第一个任务",
        achievementType: 2, // Community
        rarity: 0, // Common
        conditionType: 5, // CreateTasksCount
        conditionValue: 1,
        metadataUri: "https://example.com/achievements/first-created.json"
    },
    {
        name: "活跃发布者",
        description: "创建50个任务",
        achievementType: 2, // Community
        rarity: 2, // Epic
        conditionType: 5, // CreateTasksCount
        conditionValue: 50,
        metadataUri: "https://example.com/achievements/50-created.json"
    },
    {
        name: "社区守护者",
        description: "参与10次社区验证",
        achievementType: 2, // Community
        rarity: 1, // Rare
        conditionType: 4, // CommunityVerificationCount
        conditionValue: 10,
        metadataUri: "https://example.com/achievements/10-verifications.json"
    },
    {
        name: "质量专家",
        description: "获得平均4.5分以上评价",
        achievementType: 1, // Reputation
        rarity: 2, // Epic
        conditionType: 3, // AverageRating
        conditionValue: 45, // 4.5分 * 10
        metadataUri: "https://example.com/achievements/high-rating.json"
    },
    {
        name: "闪电侠",
        description: "在1小时内完成任务",
        achievementType: 3, // Special
        rarity: 1, // Rare
        conditionType: 6, // CompleteTaskInTime
        conditionValue: 1,
        metadataUri: "https://example.com/achievements/fast-completion.json"
    }
];

// 稀有度名称映射
const rarityNames = ['普通', '稀有', '史诗', '传说'];

async function setupAchievements() {
    try {
        console.log('🚀 开始设置NFT成就系统...');

        // 连接到本地节点
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });

        // 创建keyring并添加Alice账户
        const keyring = new Keyring({ type: 'sr25519' });
        const alice = keyring.addFromUri('//Alice');

        console.log('✅ 已连接到区块链网络');
        console.log(`📋 准备创建 ${achievements.length} 个成就定义...`);

        // 批量创建成就
        for (let i = 0; i < achievements.length; i++) {
            const achievement = achievements[i];
            
            console.log(`\n🎯 创建成就 ${i + 1}/${achievements.length}: ${achievement.name} (${rarityNames[achievement.rarity]})`);
            
            try {
                // 创建成就定义 - 使用新的API格式
                const tx = api.tx.achievements.createAchievement(
                    achievement.name,
                    achievement.description,
                    achievement.achievementType,
                    achievement.rarity,
                    achievement.conditionType,
                    achievement.conditionValue,
                    achievement.metadataUri
                );

                // 签名并发送交易
                const hash = await tx.signAndSend(alice);
                console.log(`   ✅ 交易已提交: ${hash.toHex()}`);

                // 等待一小段时间避免nonce冲突
                await new Promise(resolve => setTimeout(resolve, 3000));
                
            } catch (error) {
                console.error(`   ❌ 创建成就失败: ${error.message}`);
            }
        }

        console.log('\n🎉 成就系统设置完成！');
        console.log('\n📊 成就系统统计:');
        console.log(`   - 普通成就: ${achievements.filter(a => a.rarity === 0).length} 个`);
        console.log(`   - 稀有成就: ${achievements.filter(a => a.rarity === 1).length} 个`);
        console.log(`   - 史诗成就: ${achievements.filter(a => a.rarity === 2).length} 个`);
        console.log(`   - 传说成就: ${achievements.filter(a => a.rarity === 3).length} 个`);

        console.log('\n💡 现在用户完成任务时会自动检测并解锁相应成就！');
        console.log('💡 每个解锁的成就都会自动铸造为NFT发放给用户！');

        // 展示一些使用示例
        console.log('\n📖 使用示例:');
        console.log('   1. 用户完成第一个任务 → 自动解锁"初出茅庐"成就');
        console.log('   2. 用户连续完成3个任务 → 自动解锁"连击新手"成就');
        console.log('   3. 用户创建第一个任务 → 自动解锁"任务发布者"成就');
        console.log('   4. 所有成就都会自动铸造为NFT，用户可以在钱包中查看');

        console.log('\n🔧 集成说明:');
        console.log('   - 在任务完成时调用: pallet_achievements::Pallet::<T>::on_task_completed()');
        console.log('   - 在任务创建时调用: pallet_achievements::Pallet::<T>::on_task_created()');
        console.log('   - 在社区验证时调用: pallet_achievements::Pallet::<T>::on_community_verification()');
        console.log('   - 在任务评价时调用: pallet_achievements::Pallet::<T>::on_task_rated()');

    } catch (error) {
        console.error('❌ 设置失败:', error);
    } finally {
        process.exit(0);
    }
}

// 运行设置
setupAchievements(); 