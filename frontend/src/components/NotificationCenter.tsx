import React, { useState, useEffect, useCallback } from 'react';
import { 
  Badge, 
  Button, 
  Popover, 
  List, 
  Avatar, 
  Typography, 
  Spin, 
  Empty, 
  Divider,
  message 
} from 'antd';
import { 
  BellOutlined, 
  DeleteOutlined, 
  CheckOutlined,
  ClockCircleOutlined 
} from '@ant-design/icons';
import { useApi } from '../hooks/useApi';

const { Text, Title } = Typography;

interface Notification {
  id: number;
  type: string;
  title: string;
  content: string;
  priority: 'low' | 'medium' | 'high' | 'urgent';
  isRead: boolean;
  createdAt: number;
  readAt?: number;
  relatedData?: any;
}

interface NotificationStats {
  totalReceived: number;
  totalRead: number;
  unreadCount: number;
  lastNotificationAt: number;
}

const NotificationCenter: React.FC = () => {
  const { api, selectedAccount } = useApi();
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [unreadCount, setUnreadCount] = useState(0);
  const [stats, setStats] = useState<NotificationStats | null>(null);
  const [loading, setLoading] = useState(false);
  const [visible, setVisible] = useState(false);

  // 获取通知列表
  const fetchNotifications = useCallback(async () => {
    if (!api || !selectedAccount) return;

    try {
      setLoading(true);
      
      // 获取用户通知列表
      const notificationIds = await api.query.notifications.userNotificationList(selectedAccount);
      const notificationList: Notification[] = [];

      // 获取每个通知的详细信息
      for (const id of notificationIds.slice(-20)) { // 最新20个通知
        const notification = await api.query.notifications.userNotifications(selectedAccount, id);
        if (notification.isSome) {
          const notificationData = notification.unwrap();
          notificationList.push({
            id: notificationData.id.toNumber(),
            type: notificationData.notificationType.toString(),
            title: new TextDecoder().decode(notificationData.title),
            content: new TextDecoder().decode(notificationData.content),
            priority: notificationData.priority.toString().toLowerCase() as any,
            isRead: notificationData.isRead.toPrimitive(),
            createdAt: notificationData.createdAt.toNumber(),
            readAt: notificationData.readAt.isSome ? notificationData.readAt.unwrap().toNumber() : undefined,
            relatedData: notificationData.relatedData.isSome ? 
              JSON.parse(new TextDecoder().decode(notificationData.relatedData.unwrap())) : null,
          });
        }
      }

      // 按时间倒序排列
      notificationList.sort((a, b) => b.createdAt - a.createdAt);
      setNotifications(notificationList);

      // 获取统计信息
      const statsData = await api.query.notifications.notificationStats(selectedAccount);
      setStats({
        totalReceived: statsData.totalReceived.toNumber(),
        totalRead: statsData.totalRead.toNumber(),
        unreadCount: statsData.unreadCount.toNumber(),
        lastNotificationAt: statsData.lastNotificationAt.toNumber(),
      });

      setUnreadCount(statsData.unreadCount.toNumber());
    } catch (error) {
      console.error('获取通知失败:', error);
      message.error('获取通知失败');
    } finally {
      setLoading(false);
    }
  }, [api, selectedAccount]);

  // 标记通知为已读
  const markAsRead = async (notificationId: number) => {
    if (!api || !selectedAccount) return;

    try {
      const tx = api.tx.notifications.markNotificationRead(notificationId);
      await tx.signAndSend(selectedAccount);
      
      // 更新本地状态
      setNotifications(prev => 
        prev.map(n => 
          n.id === notificationId 
            ? { ...n, isRead: true, readAt: Date.now() }
            : n
        )
      );
      setUnreadCount(prev => Math.max(0, prev - 1));
      message.success('已标记为已读');
    } catch (error) {
      console.error('标记通知已读失败:', error);
      message.error('标记已读失败');
    }
  };

  // 删除通知
  const deleteNotification = async (notificationId: number) => {
    if (!api || !selectedAccount) return;

    try {
      const tx = api.tx.notifications.deleteNotification(notificationId);
      await tx.signAndSend(selectedAccount);
      
      // 更新本地状态
      setNotifications(prev => prev.filter(n => n.id !== notificationId));
      message.success('通知已删除');
    } catch (error) {
      console.error('删除通知失败:', error);
      message.error('删除通知失败');
    }
  };

  // 获取通知图标
  const getNotificationIcon = (type: string) => {
    switch (type) {
      case 'TaskCreated':
        return '📝';
      case 'TaskAssigned':
        return '👤';
      case 'TaskStatusChanged':
        return '🔄';
      case 'CommunityVerification':
        return '🗳️';
      case 'ReputationLevelUp':
        return '⭐';
      case 'AchievementUnlocked':
        return '🏆';
      case 'RewardReceived':
        return '💰';
      default:
        return '🔔';
    }
  };

  // 获取优先级颜色
  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'urgent':
        return '#ff4d4f';
      case 'high':
        return '#fa8c16';
      case 'medium':
        return '#1890ff';
      case 'low':
        return '#8c8c8c';
      default:
        return '#8c8c8c';
    }
  };

  // 格式化时间
  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp * 1000);
    const now = new Date();
    const diffInHours = (now.getTime() - date.getTime()) / (1000 * 60 * 60);

    if (diffInHours < 1) {
      return '刚刚';
    } else if (diffInHours < 24) {
      return `${Math.floor(diffInHours)}小时前`;
    } else {
      return date.toLocaleDateString('zh-CN', {
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    }
  };

  // 监听实时通知事件
  useEffect(() => {
    if (!api || !selectedAccount) return;

    let unsubscribe: (() => void) | null = null;

    const subscribeToEvents = async () => {
      unsubscribe = await api.query.system.events((events) => {
        events.forEach((record) => {
          const { event } = record;
          if (event.section === 'notifications') {
            if (event.method === 'NotificationCreated') {
              const [user] = event.data;
              if (user.toString() === selectedAccount) {
                // 有新通知，重新获取通知列表
                fetchNotifications();
              }
            } else if (event.method === 'RealtimeNotificationPushed') {
              const [user, notificationType] = event.data;
              if (user.toString() === selectedAccount) {
                // 显示实时通知（可以使用浏览器通知API）
                if ('Notification' in window && Notification.permission === 'granted') {
                  new Notification('新通知', {
                    body: `您有新的${notificationType}通知`,
                    icon: '/favicon.ico',
                  });
                }
              }
            }
          }
        });
      });
    };

    subscribeToEvents();

    return () => {
      if (unsubscribe) {
        unsubscribe();
      }
    };
  }, [api, selectedAccount, fetchNotifications]);

  // 请求浏览器通知权限
  useEffect(() => {
    if ('Notification' in window && Notification.permission === 'default') {
      Notification.requestPermission();
    }
  }, []);

  // 初始加载
  useEffect(() => {
    fetchNotifications();
  }, [fetchNotifications]);

  // 通知列表内容
  const notificationContent = (
    <div style={{ width: 400, maxHeight: 500, overflow: 'hidden' }}>
      <div style={{ padding: '16px 16px 8px', borderBottom: '1px solid #f0f0f0' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Title level={4} style={{ margin: 0 }}>通知中心</Title>
          {stats && (
            <Text type="secondary">{unreadCount} 条未读</Text>
          )}
        </div>
      </div>

      <div style={{ maxHeight: 400, overflow: 'auto' }}>
        {loading ? (
          <div style={{ textAlign: 'center', padding: '50px' }}>
            <Spin size="large" />
          </div>
        ) : notifications.length === 0 ? (
          <Empty 
            image={Empty.PRESENTED_IMAGE_SIMPLE}
            description="暂无通知"
            style={{ padding: '50px' }}
          />
        ) : (
          <List
            dataSource={notifications}
            renderItem={(notification) => (
              <List.Item
                style={{
                  padding: '12px 16px',
                  backgroundColor: !notification.isRead ? '#f6ffed' : undefined,
                  borderLeft: !notification.isRead ? '4px solid #52c41a' : undefined,
                  cursor: 'pointer'
                }}
                actions={[
                  !notification.isRead && (
                    <Button
                      type="text"
                      size="small"
                      icon={<CheckOutlined />}
                      onClick={(e) => {
                        e.stopPropagation();
                        markAsRead(notification.id);
                      }}
                      title="标记为已读"
                    />
                  ),
                  <Button
                    type="text"
                    size="small"
                    icon={<DeleteOutlined />}
                    onClick={(e) => {
                      e.stopPropagation();
                      deleteNotification(notification.id);
                    }}
                    title="删除通知"
                    danger
                  />
                ].filter(Boolean)}
                onClick={() => {
                  if (!notification.isRead) {
                    markAsRead(notification.id);
                  }
                }}
              >
                <List.Item.Meta
                  avatar={
                    <Avatar style={{ backgroundColor: 'transparent' }}>
                      {getNotificationIcon(notification.type)}
                    </Avatar>
                  }
                  title={
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                      <Text strong={!notification.isRead}>{notification.title}</Text>
                      <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
                        <Badge
                          color={getPriorityColor(notification.priority)}
                          text={
                            notification.priority === 'urgent' ? '紧急' :
                            notification.priority === 'high' ? '高' :
                            notification.priority === 'medium' ? '中' : '低'
                          }
                        />
                      </div>
                    </div>
                  }
                  description={
                    <div>
                      <Text type="secondary" style={{ fontSize: '12px' }}>
                        {notification.content}
                      </Text>
                      <div style={{ marginTop: 4, display: 'flex', alignItems: 'center', gap: 4 }}>
                        <ClockCircleOutlined style={{ fontSize: '12px', color: '#999' }} />
                        <Text type="secondary" style={{ fontSize: '12px' }}>
                          {formatTime(notification.createdAt)}
                        </Text>
                      </div>
                    </div>
                  }
                />
              </List.Item>
            )}
          />
        )}
      </div>

      {stats && (
        <>
          <Divider style={{ margin: 0 }} />
          <div style={{ padding: 16, backgroundColor: '#fafafa' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '12px', color: '#666' }}>
              <span>总通知数：{stats.totalReceived}</span>
              <span>已读：{stats.totalRead}</span>
              <span>未读：{stats.unreadCount}</span>
            </div>
          </div>
        </>
      )}
    </div>
  );

  return (
    <Popover
      content={notificationContent}
      title={null}
      trigger="click"
      placement="bottomRight"
      visible={visible}
      onVisibleChange={setVisible}
    >
      <Badge count={unreadCount} size="small">
        <Button 
          type="text" 
          icon={<BellOutlined />} 
          style={{ border: 'none' }}
        />
      </Badge>
    </Popover>
  );
};

export default NotificationCenter; 