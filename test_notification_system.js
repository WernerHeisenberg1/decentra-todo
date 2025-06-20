const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { cryptoWaitReady } = require('@polkadot/util-crypto');

async function main() {
    await cryptoWaitReady();

    // 连接到本地节点
    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({ provider });

    console.log('🔗 已连接到 Substrate 节点');
    console.log(`⛓️  链信息: ${(await api.rpc.system.chain()).toString()}`);

    // 创建账户
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    console.log('\n👥 测试账户:');
    console.log(`   Alice: ${alice.address}`);
    console.log(`   Bob: ${bob.address}`);

    try {
        // 1. 测试通知偏好设置
        console.log('\n🔧 测试 1: 设置通知偏好');
        const preferences = {
            taskNotifications: true,
            communityNotifications: true,
            reputationNotifications: true,
            achievementNotifications: true,
            rewardNotifications: true,
            systemAnnouncements: true,
            emailNotifications: false,
            pushNotifications: true,
        };

        const tx1 = api.tx.notifications.updateNotificationPreferences(preferences);
        await tx1.signAndSend(alice, ({ status, events }) => {
            if (status.isInBlock) {
                console.log('✅ 通知偏好设置成功');
                events.forEach(({ event }) => {
                    if (event.section === 'notifications' && event.method === 'NotificationPreferencesUpdated') {
                        console.log(`📝 偏好已更新: ${JSON.stringify(event.data[1].toJSON())}`);
                    }
                });
            }
        });

        // 等待交易完成
        await new Promise(resolve => setTimeout(resolve, 6000));

        // 2. 测试任务创建通知
        console.log('\n📋 测试 2: 任务创建通知');
        const taskTx = api.tx.tasks.createTask(
            '测试通知任务',
            '这是一个测试通知系统的任务',
            2, // priority: Medium
            5, // difficulty
            1000000000000, // reward: 1 token
            null // deadline
        );

        let taskId = null;
        await taskTx.signAndSend(alice, ({ status, events }) => {
            if (status.isInBlock) {
                console.log('✅ 任务创建成功');
                events.forEach(({ event }) => {
                    if (event.section === 'tasks' && event.method === 'TaskCreated') {
                        taskId = event.data[0].toNumber();
                        console.log(`📋 任务ID: ${taskId}`);
                    }
                    if (event.section === 'notifications' && event.method === 'NotificationCreated') {
                        console.log('🔔 通知已创建:', {
                            user: event.data[0].toString(),
                            notificationId: event.data[1].toNumber(),
                            type: event.data[2].toString(),
                            title: new TextDecoder().decode(event.data[3])
                        });
                    }
                });
            }
        });

        // 等待交易完成
        await new Promise(resolve => setTimeout(resolve, 6000));

        // 3. 测试任务分配通知
        if (taskId !== null) {
            console.log('\n👤 测试 3: 任务分配通知');
            const assignTx = api.tx.tasks.assignTask(taskId, bob.address);
            await assignTx.signAndSend(alice, ({ status, events }) => {
                if (status.isInBlock) {
                    console.log('✅ 任务分配成功');
                    events.forEach(({ event }) => {
                        if (event.section === 'notifications' && event.method === 'NotificationCreated') {
                            console.log('🔔 分配通知已创建:', {
                                user: event.data[0].toString(),
                                notificationId: event.data[1].toNumber(),
                                type: event.data[2].toString(),
                                title: new TextDecoder().decode(event.data[3])
                            });
                        }
                    });
                }
            });

            // 等待交易完成
            await new Promise(resolve => setTimeout(resolve, 6000));

            // 4. 测试任务状态变更通知
            console.log('\n🔄 测试 4: 任务状态变更通知');
            const statusTx = api.tx.tasks.changeTaskStatus(taskId, 1); // InProgress
            await statusTx.signAndSend(bob, ({ status, events }) => {
                if (status.isInBlock) {
                    console.log('✅ 任务状态更新成功');
                    events.forEach(({ event }) => {
                        if (event.section === 'notifications' && event.method === 'NotificationCreated') {
                            console.log('🔔 状态变更通知已创建:', {
                                user: event.data[0].toString(),
                                notificationId: event.data[1].toNumber(),
                                type: event.data[2].toString(),
                                title: new TextDecoder().decode(event.data[3])
                            });
                        }
                    });
                }
            });

            // 等待交易完成
            await new Promise(resolve => setTimeout(resolve, 6000));
        }

        // 5. 查询用户通知
        console.log('\n📱 测试 5: 查询用户通知');
        
        // 查询 Alice 的通知
        const aliceNotifications = await api.query.notifications.userNotificationList(alice.address);
        console.log(`👩 Alice 的通知数量: ${aliceNotifications.length}`);

        for (const notificationId of aliceNotifications.slice(-3)) { // 显示最新3个
            const notification = await api.query.notifications.userNotifications(alice.address, notificationId);
            if (notification.isSome) {
                const notificationData = notification.unwrap();
                console.log(`📋 通知 ${notificationId}:`, {
                    type: notificationData.notificationType.toString(),
                    title: new TextDecoder().decode(notificationData.title),
                    content: new TextDecoder().decode(notificationData.content),
                    priority: notificationData.priority.toString(),
                    isRead: notificationData.isRead.toPrimitive(),
                    createdAt: notificationData.createdAt.toNumber(),
                });
            }
        }

        // 查询 Bob 的通知
        const bobNotifications = await api.query.notifications.userNotificationList(bob.address);
        console.log(`\n👨 Bob 的通知数量: ${bobNotifications.length}`);

        for (const notificationId of bobNotifications.slice(-3)) { // 显示最新3个
            const notification = await api.query.notifications.userNotifications(bob.address, notificationId);
            if (notification.isSome) {
                const notificationData = notification.unwrap();
                console.log(`📋 通知 ${notificationId}:`, {
                    type: notificationData.notificationType.toString(),
                    title: new TextDecoder().decode(notificationData.title),
                    content: new TextDecoder().decode(notificationData.content),
                    priority: notificationData.priority.toString(),
                    isRead: notificationData.isRead.toPrimitive(),
                    createdAt: notificationData.createdAt.toNumber(),
                });
            }
        }

        // 6. 测试标记通知为已读
        if (bobNotifications.length > 0) {
            console.log('\n✅ 测试 6: 标记通知为已读');
            const latestNotificationId = bobNotifications[bobNotifications.length - 1];
            const readTx = api.tx.notifications.markNotificationRead(latestNotificationId);
            await readTx.signAndSend(bob, ({ status, events }) => {
                if (status.isInBlock) {
                    console.log('✅ 通知已标记为已读');
                    events.forEach(({ event }) => {
                        if (event.section === 'notifications' && event.method === 'NotificationRead') {
                            console.log('📖 已读事件:', {
                                user: event.data[0].toString(),
                                notificationId: event.data[1].toNumber(),
                            });
                        }
                    });
                }
            });

            // 等待交易完成
            await new Promise(resolve => setTimeout(resolve, 6000));
        }

        // 7. 查询通知统计
        console.log('\n📊 测试 7: 查询通知统计');
        const aliceStats = await api.query.notifications.notificationStats(alice.address);
        console.log('👩 Alice 的通知统计:', {
            totalReceived: aliceStats.totalReceived.toNumber(),
            totalRead: aliceStats.totalRead.toNumber(),
            unreadCount: aliceStats.unreadCount.toNumber(),
            lastNotificationAt: aliceStats.lastNotificationAt.toNumber(),
        });

        const bobStats = await api.query.notifications.notificationStats(bob.address);
        console.log('👨 Bob 的通知统计:', {
            totalReceived: bobStats.totalReceived.toNumber(),
            totalRead: bobStats.totalRead.toNumber(),
            unreadCount: bobStats.unreadCount.toNumber(),
            lastNotificationAt: bobStats.lastNotificationAt.toNumber(),
        });

        console.log('\n🎉 通知系统测试完成！');

    } catch (error) {
        console.error('❌ 测试过程中出现错误:', error);
    } finally {
        await api.disconnect();
        console.log('🔌 已断开连接');
    }
}

main().catch(console.error); 