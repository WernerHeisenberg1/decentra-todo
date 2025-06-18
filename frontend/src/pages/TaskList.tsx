import React, { useState, useEffect } from 'react';
import {
  Table,
  Button,
  Space,
  Tag,
  Modal,
  message,
  Card,
  Row,
  Col,
  Select,
  Input,
  Tooltip,
} from 'antd';
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  PlayCircleOutlined,
  CheckCircleOutlined,
  CloseCircleOutlined,
  StarOutlined,
  TrophyOutlined,
} from '@ant-design/icons';
import { useApi } from '../hooks/useApi';
import { Task, TaskStatus, TaskPriority, UserReputation, EvaluationFormData } from '../types';
import ReputationCard from '../components/ReputationCard';
import TaskEvaluationModal from '../components/TaskEvaluationModal';

const { Search } = Input;
const { Option } = Select;

const TaskList: React.FC = () => {
  const { api, selectedAccount } = useApi();
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [userReputation, setUserReputation] = useState<UserReputation | null>(null);
  const [reputationLoading, setReputationLoading] = useState(false);
  const [evaluationModalVisible, setEvaluationModalVisible] = useState(false);
  const [selectedTaskForEvaluation, setSelectedTaskForEvaluation] = useState<Task | null>(null);
  const [searchText, setSearchText] = useState('');
  const [statusFilter, setStatusFilter] = useState<TaskStatus | undefined>();
  const [priorityFilter, setPriorityFilter] = useState<TaskPriority | undefined>();

  // 加载任务列表
  const loadTasks = async () => {
    if (!api || !selectedAccount) return;

    setLoading(true);
    try {
      // 这里需要调用实际的API来获取任务
      // 暂时使用模拟数据
      const mockTasks: Task[] = [
        {
          id: 1,
          title: "实现用户认证功能",
          description: "为应用添加用户注册、登录和权限管理功能",
          creator: "Alice",
          assignee: "Bob",
          status: TaskStatus.Completed,
          priority: TaskPriority.High,
          difficulty: 7,
          reward: 500,
          deadline: Date.now() + 86400000,
          created_at: Date.now() - 172800000,
          updated_at: Date.now() - 86400000,
        },
        {
          id: 2,
          title: "优化数据库查询性能",
          description: "分析并优化慢查询，提高应用响应速度",
          creator: "Charlie",
          assignee: null,
          status: TaskStatus.Pending,
          priority: TaskPriority.Medium,
          difficulty: 5,
          reward: 300,
          deadline: Date.now() + 259200000,
          created_at: Date.now() - 86400000,
          updated_at: Date.now() - 86400000,
        },
      ];
      setTasks(mockTasks);
    } catch (error) {
      message.error('加载任务列表失败');
    } finally {
      setLoading(false);
    }
  };

  // 加载用户声誉
  const loadUserReputation = async () => {
    if (!api || !selectedAccount) return;

    setReputationLoading(true);
    try {
      // 这里需要调用实际的API来获取用户声誉
      // 暂时使用模拟数据
      const mockReputation: UserReputation = {
        totalScore: 350,
        level: 'Skilled' as any,
        completedTasks: 12,
        cancelledTasks: 2,
        totalRatings: 10,
        averageRating: 4.2,
        completionRate: 0.86,
        lastUpdated: Date.now(),
      };
      setUserReputation(mockReputation);
    } catch (error) {
      message.error('加载声誉信息失败');
    } finally {
      setReputationLoading(false);
    }
  };

  // 处理任务评价
  const handleTaskEvaluation = async (taskId: number, data: EvaluationFormData) => {
    if (!api || !selectedAccount) return;

    try {
      // 这里应该调用区块链API提交评价
      console.log('提交任务评价:', { taskId, ...data });
      
      // 模拟API调用
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      message.success('评价提交成功！执行者的声誉已更新。');
      
      // 重新加载数据
      loadTasks();
      loadUserReputation();
    } catch (error) {
      throw error;
    }
  };

  // 获取状态标签
  const getStatusTag = (status: TaskStatus) => {
    const statusMap = {
      [TaskStatus.Pending]: { color: 'default', text: '待处理' },
      [TaskStatus.InProgress]: { color: 'processing', text: '进行中' },
      [TaskStatus.Completed]: { color: 'success', text: '已完成' },
      [TaskStatus.Cancelled]: { color: 'error', text: '已取消' },
      [TaskStatus.PendingVerification]: { color: 'warning', text: '待验证' },
    };
    return <Tag color={statusMap[status].color}>{statusMap[status].text}</Tag>;
  };

  // 获取优先级标签
  const getPriorityTag = (priority: TaskPriority) => {
    const priorityMap = {
      [TaskPriority.Low]: { color: 'green', text: '低' },
      [TaskPriority.Medium]: { color: 'blue', text: '中' },
      [TaskPriority.High]: { color: 'orange', text: '高' },
      [TaskPriority.Urgent]: { color: 'red', text: '紧急' },
    };
    return <Tag color={priorityMap[priority].color}>{priorityMap[priority].text}</Tag>;
  };

  // 过滤任务
  const filteredTasks = tasks.filter(task => {
    const matchesSearch = task.title.toLowerCase().includes(searchText.toLowerCase()) ||
                         task.description.toLowerCase().includes(searchText.toLowerCase());
    const matchesStatus = !statusFilter || task.status === statusFilter;
    const matchesPriority = !priorityFilter || task.priority === priorityFilter;
    
    return matchesSearch && matchesStatus && matchesPriority;
  });

  // 表格列定义
  const columns = [
    {
      title: '任务标题',
      dataIndex: 'title',
      key: 'title',
      render: (text: string, record: Task) => (
        <div>
          <div style={{ fontWeight: 'bold' }}>{text}</div>
          <div style={{ fontSize: '12px', color: '#666' }}>
            难度: {record.difficulty}/10 | 奖励: {record.reward}
          </div>
        </div>
      ),
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: TaskStatus) => getStatusTag(status),
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      render: (priority: TaskPriority) => getPriorityTag(priority),
    },
    {
      title: '创建者',
      dataIndex: 'creator',
      key: 'creator',
    },
    {
      title: '执行者',
      dataIndex: 'assignee',
      key: 'assignee',
      render: (assignee: string | null) => assignee || '未分配',
    },
    {
      title: '操作',
      key: 'action',
      render: (_, record: Task) => (
        <Space>
          <Tooltip title="编辑任务">
            <Button 
              type="link" 
              icon={<EditOutlined />}
              size="small"
              disabled={record.creator !== selectedAccount}
            />
          </Tooltip>
          
          {record.status === TaskStatus.Completed && record.creator === selectedAccount && record.assignee && (
            <Tooltip title="评价任务">
              <Button
                type="link"
                icon={<StarOutlined />}
                size="small"
                onClick={() => {
                  setSelectedTaskForEvaluation(record);
                  setEvaluationModalVisible(true);
                }}
              />
            </Tooltip>
          )}
          
          <Tooltip title="删除任务">
            <Button
              type="link"
              danger
              icon={<DeleteOutlined />}
              size="small"
              disabled={record.creator !== selectedAccount}
            />
          </Tooltip>
        </Space>
      ),
    },
  ];

  useEffect(() => {
    if (selectedAccount) {
      loadTasks();
      loadUserReputation();
    }
  }, [api, selectedAccount]);

  return (
    <div style={{ padding: '24px' }}>
      <Row gutter={24}>
        {/* 左侧：任务列表 */}
        <Col span={16}>
          <Card 
            title="任务列表" 
            extra={
              <Button type="primary" icon={<PlusOutlined />}>
                创建任务
              </Button>
            }
          >
            {/* 搜索和过滤器 */}
            <div style={{ marginBottom: 16 }}>
              <Row gutter={16}>
                <Col span={8}>
                  <Search
                    placeholder="搜索任务标题或描述"
                    value={searchText}
                    onChange={(e) => setSearchText(e.target.value)}
                    allowClear
                  />
                </Col>
                <Col span={4}>
                  <Select
                    placeholder="状态"
                    style={{ width: '100%' }}
                    value={statusFilter}
                    onChange={setStatusFilter}
                    allowClear
                  >
                    <Option value={TaskStatus.Pending}>待处理</Option>
                    <Option value={TaskStatus.InProgress}>进行中</Option>
                    <Option value={TaskStatus.Completed}>已完成</Option>
                    <Option value={TaskStatus.Cancelled}>已取消</Option>
                    <Option value={TaskStatus.PendingVerification}>待验证</Option>
                  </Select>
                </Col>
                <Col span={4}>
                  <Select
                    placeholder="优先级"
                    style={{ width: '100%' }}
                    value={priorityFilter}
                    onChange={setPriorityFilter}
                    allowClear
                  >
                    <Option value={TaskPriority.Low}>低</Option>
                    <Option value={TaskPriority.Medium}>中</Option>
                    <Option value={TaskPriority.High}>高</Option>
                    <Option value={TaskPriority.Urgent}>紧急</Option>
                  </Select>
                </Col>
              </Row>
            </div>

            <Table
              columns={columns}
              dataSource={filteredTasks}
              rowKey="id"
              loading={loading}
              pagination={{
                total: filteredTasks.length,
                pageSize: 10,
                showSizeChanger: true,
                showQuickJumper: true,
                showTotal: (total) => `共 ${total} 个任务`,
              }}
            />
          </Card>
        </Col>

        {/* 右侧：声誉信息 */}
        <Col span={8}>
          <ReputationCard 
            reputation={userReputation || undefined}
            loading={reputationLoading}
          />
        </Col>
      </Row>

      {/* 任务评价弹窗 */}
      <TaskEvaluationModal
        visible={evaluationModalVisible}
        task={selectedTaskForEvaluation}
        onCancel={() => {
          setEvaluationModalVisible(false);
          setSelectedTaskForEvaluation(null);
        }}
        onSubmit={handleTaskEvaluation}
      />
    </div>
  );
};

export default TaskList; 