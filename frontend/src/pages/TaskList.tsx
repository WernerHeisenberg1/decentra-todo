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
  Tabs,
} from 'antd';
import { 
  PlusOutlined,
  SearchOutlined,
} from '@ant-design/icons';

import { Task, TaskStatus, UserReputation, ReputationLevel } from '../types';
import { useApi } from '../hooks/useApi';
import ReputationCard from '../components/ReputationCard';
import TaskSearchForm, { SearchFilters } from '../components/TaskSearchForm';
import SearchResults from '../components/SearchResults';

const { Title, Text } = Typography;
const { TabPane } = Tabs;

const TaskList: React.FC = () => {
  const navigate = useNavigate();
  const { api, selectedAccount, isConnected } = useApi();
  
  const [tasks, setTasks] = useState<Task[]>([]);
  const [searchResults, setSearchResults] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchLoading, setSearchLoading] = useState(false);
  const [activeTab, setActiveTab] = useState('list');
  const [searchFilters, setSearchFilters] = useState<SearchFilters>({});
  const [searchTotal, setSearchTotal] = useState(0);
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize, setPageSize] = useState(10);
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
      
      // 模拟获取任务列表 - 扩展了更多样本数据
      const mockTasks: Task[] = [
        {
          id: 1,
          title: '开发新功能',
          description: '实现用户认证系统，包括登录、注册、密码重置等功能',
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
          description: '解决登录页面的显示问题，确保在不同设备上都能正常显示',
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
          description: '为新API编写详细文档，包括接口说明和使用示例',
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
        {
          id: 4,
          title: '性能优化',
          description: '优化数据库查询性能，提升系统响应速度',
          creator: 'eve',
          assignee: null,
          status: TaskStatus.Pending,
          priority: 'Urgent' as any,
          difficulty: 8,
          reward: 800,
          deadline: Date.now() + 86400000 * 2,
          created_at: Date.now() - 86400000 * 3,
          updated_at: Date.now(),
        },
        {
          id: 5,
          title: '设计UI界面',
          description: '设计移动端用户界面，符合现代化设计标准',
          creator: 'frank',
          assignee: 'george',
          status: TaskStatus.PendingVerification,
          priority: 'Medium' as any,
          difficulty: 6,
          reward: 400,
          deadline: Date.now() + 86400000 * 5,
          created_at: Date.now() - 86400000 * 4,
          updated_at: Date.now(),
        },
        {
          id: 6,
          title: '代码重构',
          description: '重构核心模块代码，提高代码可维护性',
          creator: 'helen',
          assignee: null,
          status: TaskStatus.Cancelled,
          priority: 'Low' as any,
          difficulty: 9,
          reward: 600,
          deadline: Date.now() - 86400000,
          created_at: Date.now() - 86400000 * 10,
          updated_at: Date.now() - 86400000 * 3,
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

  const handleSearch = useCallback(async (filters: SearchFilters) => {
    try {
      setSearchLoading(true);
      setSearchFilters(filters);
      setCurrentPage(1);

      // 模拟搜索API调用
      // 在实际项目中，这里应该调用 api.query.tasks.search_tasks
      let filteredTasks = [...tasks];

      // 关键词搜索
      if (filters.keyword) {
        filteredTasks = filteredTasks.filter(task =>
          task.title.toLowerCase().includes(filters.keyword!.toLowerCase()) ||
          task.description.toLowerCase().includes(filters.keyword!.toLowerCase())
        );
      }

      // 状态筛选
      if (filters.status) {
        filteredTasks = filteredTasks.filter(task => task.status === filters.status);
      }

      // 优先级筛选
      if (filters.priority) {
        filteredTasks = filteredTasks.filter(task => task.priority === filters.priority);
      }

      // 难度范围筛选
      if (filters.difficultyRange) {
        const [min, max] = filters.difficultyRange;
        filteredTasks = filteredTasks.filter(task => 
          task.difficulty >= min && task.difficulty <= max
        );
      }

      // 奖励范围筛选
      if (filters.rewardRange) {
        const [min, max] = filters.rewardRange;
        filteredTasks = filteredTasks.filter(task => 
          task.reward >= min && task.reward <= max
        );
      }

      // 创建者筛选
      if (filters.creator) {
        filteredTasks = filteredTasks.filter(task => 
          task.creator.toLowerCase().includes(filters.creator!.toLowerCase())
        );
      }

      // 执行者筛选
      if (filters.assignee) {
        filteredTasks = filteredTasks.filter(task => 
          task.assignee?.toLowerCase().includes(filters.assignee!.toLowerCase())
        );
      }

      // 未分配筛选
      if (filters.unassignedOnly) {
        filteredTasks = filteredTasks.filter(task => !task.assignee);
      }

      // 有截止日期筛选
      if (filters.hasDeadline) {
        filteredTasks = filteredTasks.filter(task => task.deadline);
      }

      // 排序
      filteredTasks.sort((a, b) => {
        let aValue: any, bValue: any;
        
        switch (filters.sortBy) {
          case 'created_at':
            aValue = a.created_at;
            bValue = b.created_at;
            break;
          case 'updated_at':
            aValue = a.updated_at;
            bValue = b.updated_at;
            break;
          case 'deadline':
            aValue = a.deadline || 0;
            bValue = b.deadline || 0;
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
            const priorityOrder = { 'Low': 1, 'Medium': 2, 'High': 3, 'Urgent': 4 };
            aValue = priorityOrder[a.priority as keyof typeof priorityOrder] || 0;
            bValue = priorityOrder[b.priority as keyof typeof priorityOrder] || 0;
            break;
          default:
            aValue = a.created_at;
            bValue = b.created_at;
        }

        if (filters.sortDesc) {
          return bValue > aValue ? 1 : -1;
        } else {
          return aValue > bValue ? 1 : -1;
        }
      });

      const total = filteredTasks.length;
      const startIndex = (currentPage - 1) * pageSize;
      const endIndex = startIndex + pageSize;
      const paginatedTasks = filteredTasks.slice(startIndex, endIndex);

      setSearchResults(paginatedTasks);
      setSearchTotal(total);
      setActiveTab('search');
      
    } catch (error) {
      console.error('搜索失败:', error);
      message.error('搜索失败');
    } finally {
      setSearchLoading(false);
    }
  }, [tasks, currentPage, pageSize]);

  const handleSearchReset = useCallback(() => {
    setSearchResults([]);
    setSearchFilters({});
    setSearchTotal(0);
    setCurrentPage(1);
    setActiveTab('list');
  }, []);

  const handlePageChange = useCallback((page: number, size?: number) => {
    setCurrentPage(page);
    if (size) {
      setPageSize(size);
    }
    // 重新执行搜索
    handleSearch(searchFilters);
  }, [searchFilters, handleSearch]);

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
                <Title level={2}>任务管理</Title>
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
          {/* 搜索表单 */}
          <TaskSearchForm
            onSearch={handleSearch}
            onReset={handleSearchReset}
            loading={searchLoading}
          />
        </Col>

        <Col span={24}>
          <Tabs 
            activeKey={activeTab} 
            onChange={setActiveTab}
            items={[
              {
                key: 'list',
                label: (
                  <span>
                    <Space>
                      任务列表
                      <Tag>{tasks.length}</Tag>
                    </Space>
                  </span>
                ),
                children: (
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
                )
              },
              {
                key: 'search',
                label: (
                  <span>
                    <Space>
                      <SearchOutlined />
                      搜索结果
                      {searchTotal > 0 && <Tag>{searchTotal}</Tag>}
                    </Space>
                  </span>
                ),
                children: (
                  <SearchResults
                    tasks={searchResults}
                    loading={searchLoading}
                    total={searchTotal}
                    currentPage={currentPage}
                    pageSize={pageSize}
                    onPageChange={handlePageChange}
                    onTaskClick={handleTaskClick}
                  />
                )
              }
            ]}
          />
        </Col>
      </Row>
    </div>
  );
};

export default TaskList; 