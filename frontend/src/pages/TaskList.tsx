import React, { useState, useEffect, useCallback } from 'react';
import { useNavigate } from 'react-router-dom';
import {
  Card,
  Button,
  List,
  Tag,
  Space,
  Typography,
  Row,
  Col,
  Statistic,
  Spin,
  message,
} from 'antd';
import { 
  PlusOutlined,
  UserOutlined,
} from '@ant-design/icons';

import { Task, TaskStatus, UserReputation, ReputationLevel } from '../types';
import { useApi } from '../hooks/useApi';
import ReputationCard from '../components/ReputationCard';

const { Title, Text } = Typography;

const TaskList: React.FC = () => {
  const navigate = useNavigate();
  const { api, selectedAccount, isConnected } = useApi();
  
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [userReputation, setUserReputation] = useState<UserReputation>({
    totalScore: 0,
    level: ReputationLevel.Newcomer,
    completedTasks: 0,
    cancelledTasks: 0,
    totalRatings: 0,
    averageRating: 0,
    completionRate: 0,
    lastUpdated: 0,
  });

  const loadTasks = useCallback(async () => {
    if (!api) return;
    
    try {
      setLoading(true);
      
      // 模拟获取任务列表
      const mockTasks: Task[] = [
        {
          id: 1,
          title: '开发新功能',
          description: '实现用户认证系统',
          creator: 'alice',
          assignee: null,
          status: TaskStatus.Pending,
          priority: 'High' as any,
          difficulty: 7,
          reward: 500,
          deadline: Date.now() + 86400000 * 7,
          created_at: Date.now() - 86400000,
          updated_at: Date.now(),
        },
        {
          id: 2,
          title: '修复Bug',
          description: '解决登录页面的显示问题',
          creator: 'bob',
          assignee: 'charlie',
          status: TaskStatus.InProgress,
          priority: 'Medium' as any,
          difficulty: 4,
          reward: 200,
          deadline: Date.now() + 86400000 * 3,
          created_at: Date.now() - 86400000 * 2,
          updated_at: Date.now(),
        },
        {
          id: 3,
          title: '撰写文档',
          description: '为新API编写详细文档',
          creator: 'alice',
          assignee: 'dave',
          status: TaskStatus.Completed,
          priority: 'Low' as any,
          difficulty: 3,
          reward: 150,
          deadline: Date.now() + 86400000,
          created_at: Date.now() - 86400000 * 5,
          updated_at: Date.now(),
        },
      ];
      
      setTasks(mockTasks);
    } catch (error) {
      console.error('加载任务失败:', error);
      message.error('加载任务失败');
    } finally {
      setLoading(false);
    }
  }, [api]);

  const loadUserReputation = useCallback(async () => {
    if (!api || !selectedAccount) return;
    
    try {
      // 模拟获取用户声誉
      const mockReputation: UserReputation = {
        totalScore: 1250,
        level: ReputationLevel.Skilled,
        completedTasks: 15,
        cancelledTasks: 2,
        totalRatings: 12,
        averageRating: 4.2,
        completionRate: 88.2,
        lastUpdated: Date.now(),
      };
      
      setUserReputation(mockReputation);
    } catch (error) {
      console.error('加载声誉信息失败:', error);
    }
  }, [api, selectedAccount]);

  useEffect(() => {
    if (isConnected) {
      loadTasks();
      loadUserReputation();
    }
  }, [isConnected, loadTasks, loadUserReputation]);

  const getStatusColor = (status: TaskStatus) => {
    switch (status) {
      case TaskStatus.Pending: return 'default';
      case TaskStatus.InProgress: return 'processing';
      case TaskStatus.Completed: return 'success';
      case TaskStatus.Cancelled: return 'error';
      case TaskStatus.PendingVerification: return 'warning';
      default: return 'default';
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'Low': return 'green';
      case 'Medium': return 'orange';
      case 'High': return 'red';
      case 'Urgent': return 'magenta';
      default: return 'default';
    }
  };

  const formatDeadline = (deadline: number) => {
    const now = Date.now();
    const timeLeft = deadline - now;
    
    if (timeLeft < 0) return '已过期';
    
    const days = Math.floor(timeLeft / (1000 * 60 * 60 * 24));
    const hours = Math.floor((timeLeft % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    
    if (days > 0) return `${days}天${hours}小时`;
    return `${hours}小时`;
  };

  const handleTaskClick = (taskId: number) => {
    navigate(`/tasks/${taskId}`);
  };

  const renderTaskItem = (task: Task) => (
    <List.Item
      key={task.id}
      actions={[
        <Button 
          type="link" 
          onClick={() => handleTaskClick(task.id)}
        >
          查看详情
        </Button>
      ]}
      style={{ cursor: 'pointer' }}
      onClick={() => handleTaskClick(task.id)}
    >
      <List.Item.Meta
        title={
          <Space>
            <span>{task.title}</span>
            <Tag color={getStatusColor(task.status)}>{task.status}</Tag>
            <Tag color={getPriorityColor(task.priority)}>{task.priority}</Tag>
          </Space>
        }
        description={
          <div>
            <Text type="secondary">{task.description}</Text>
            <div style={{ marginTop: '8px' }}>
              <Space>
                <Text>难度: {task.difficulty}/10</Text>
                <Text>奖励: {task.reward} tokens</Text>
                <Text>截止: {formatDeadline(task.deadline)}</Text>
                {task.assignee && <Text>执行者: {task.assignee}</Text>}
              </Space>
            </div>
          </div>
        }
      />
    </List.Item>
  );

  return (
    <div style={{ padding: '24px' }}>
      <Row gutter={[24, 24]}>
        <Col span={24}>
          <Card>
            <Row justify="space-between" align="middle">
              <Col>
                <Title level={2}>任务列表</Title>
              </Col>
              <Col>
                <Button
                  type="primary"
                  icon={<PlusOutlined />}
                  onClick={() => navigate('/tasks/create')}
                >
                  创建任务
                </Button>
              </Col>
            </Row>
          </Card>
        </Col>

        {selectedAccount && (
          <Col span={24}>
            <ReputationCard reputation={userReputation} />
          </Col>
        )}

        <Col span={24}>
          <Card>
            <Row gutter={[16, 16]} style={{ marginBottom: '24px' }}>
              <Col span={6}>
                <Statistic title="总任务数" value={tasks.length} />
              </Col>
              <Col span={6}>
                <Statistic 
                  title="待处理" 
                  value={tasks.filter(t => t.status === TaskStatus.Pending).length} 
                />
              </Col>
              <Col span={6}>
                <Statistic 
                  title="进行中" 
                  value={tasks.filter(t => t.status === TaskStatus.InProgress).length} 
                />
              </Col>
              <Col span={6}>
                <Statistic 
                  title="已完成" 
                  value={tasks.filter(t => t.status === TaskStatus.Completed).length} 
                />
              </Col>
            </Row>

            <Spin spinning={loading}>
              <List
                dataSource={tasks}
                renderItem={renderTaskItem}
                pagination={{
                  total: tasks.length,
                  pageSize: 10,
                  showSizeChanger: true,
                  showQuickJumper: true,
                  showTotal: (total, range) => 
                    `第 ${range[0]}-${range[1]} 条，共 ${total} 条`,
                }}
              />
            </Spin>
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default TaskList; 