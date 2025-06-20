import React from 'react';
import { List, Card, Tag, Space, Typography, Button, Empty, Pagination } from 'antd';
import { 
  ClockCircleOutlined, 
  UserOutlined, 
  StarOutlined,
  CalendarOutlined,
} from '@ant-design/icons';
import { Task, TaskStatus } from '../types';

const { Text, Title } = Typography;

interface SearchResultsProps {
  tasks: Task[];
  loading?: boolean;
  total?: number;
  currentPage?: number;
  pageSize?: number;
  onPageChange?: (page: number, size?: number) => void;
  onTaskClick?: (taskId: number) => void;
}

const SearchResults: React.FC<SearchResultsProps> = ({
  tasks,
  loading = false,
  total = 0,
  currentPage = 1,
  pageSize = 10,
  onPageChange,
  onTaskClick,
}) => {
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

  const getStatusText = (status: TaskStatus) => {
    switch (status) {
      case TaskStatus.Pending: return '待处理';
      case TaskStatus.InProgress: return '进行中';
      case TaskStatus.Completed: return '已完成';
      case TaskStatus.Cancelled: return '已取消';
      case TaskStatus.PendingVerification: return '待验证';
      default: return '未知';
    }
  };

  const getPriorityText = (priority: string) => {
    switch (priority) {
      case 'Low': return '低';
      case 'Medium': return '中';
      case 'High': return '高';
      case 'Urgent': return '紧急';
      default: return priority;
    }
  };

  const formatDeadline = (deadline: number) => {
    const now = Date.now();
    const timeLeft = deadline - now;
    
    if (timeLeft < 0) return { text: '已过期', color: 'red' };
    
    const days = Math.floor(timeLeft / (1000 * 60 * 60 * 24));
    const hours = Math.floor((timeLeft % (1000 * 60 * 60 * 24)) / (1000 * 60 * 60));
    
    if (days > 7) return { text: `${days}天后`, color: 'green' };
    if (days > 3) return { text: `${days}天${hours}小时`, color: 'orange' };
    if (days > 0) return { text: `${days}天${hours}小时`, color: 'red' };
    return { text: `${hours}小时`, color: 'red' };
  };

  const renderTaskItem = (task: Task) => {
    const deadline = task.deadline ? formatDeadline(task.deadline) : null;

    return (
      <List.Item
        key={task.id}
        style={{ cursor: 'pointer' }}
        onClick={() => onTaskClick?.(task.id)}
      >
        <Card
          hoverable
          style={{ width: '100%' }}
          bodyStyle={{ padding: '16px' }}
        >
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-start' }}>
            <div style={{ flex: 1 }}>
              {/* 任务标题和标签 */}
              <div style={{ marginBottom: 8 }}>
                <Title level={4} style={{ margin: 0, marginBottom: 4 }}>
                  {task.title}
                </Title>
                <Space>
                  <Tag color={getStatusColor(task.status)}>
                    {getStatusText(task.status)}
                  </Tag>
                  <Tag color={getPriorityColor(task.priority as string)}>
                    {getPriorityText(task.priority as string)}
                  </Tag>
                  <Tag color="blue">难度 {task.difficulty}/10</Tag>
                </Space>
              </div>

              {/* 任务描述 */}
              <Text type="secondary" style={{ display: 'block', marginBottom: 12 }}>
                {task.description.length > 150 
                  ? `${task.description.substring(0, 150)}...` 
                  : task.description
                }
              </Text>

              {/* 任务详情 */}
              <Space size="large" style={{ marginBottom: 8 }}>
                <Space>
                  <StarOutlined style={{ color: '#faad14' }} />
                  <Text>奖励: {task.reward} tokens</Text>
                </Space>
                <Space>
                  <UserOutlined />
                  <Text>创建者: {task.creator}</Text>
                </Space>
                {task.assignee && (
                  <Space>
                    <UserOutlined style={{ color: '#52c41a' }} />
                    <Text>执行者: {task.assignee}</Text>
                  </Space>
                )}
              </Space>

              {/* 时间信息 */}
              <Space size="large">
                <Space>
                  <CalendarOutlined />
                  <Text type="secondary">
                    创建于: {new Date(task.created_at).toLocaleDateString()}
                  </Text>
                </Space>
                {deadline && (
                  <Space>
                    <ClockCircleOutlined style={{ color: deadline.color }} />
                    <Text style={{ color: deadline.color }}>
                      截止: {deadline.text}
                    </Text>
                  </Space>
                )}
              </Space>
            </div>

            {/* 操作按钮 */}
            <div style={{ marginLeft: 16 }}>
              <Button type="link">查看详情</Button>
            </div>
          </div>
        </Card>
      </List.Item>
    );
  };

  if (tasks.length === 0 && !loading) {
    return (
      <Card>
        <Empty
          description="没有找到符合条件的任务"
          image={Empty.PRESENTED_IMAGE_SIMPLE}
        />
      </Card>
    );
  }

  return (
    <div>
      {/* 搜索结果统计 */}
      <Card size="small" style={{ marginBottom: 16 }}>
        <Space>
          <Text strong>搜索结果:</Text>
          <Text>共找到 {total} 个任务</Text>
        </Space>
      </Card>

      {/* 任务列表 */}
      <List
        loading={loading}
        dataSource={tasks}
        renderItem={renderTaskItem}
        pagination={false}
        style={{ marginBottom: 16 }}
      />

      {/* 分页 */}
      {total > pageSize && (
        <div style={{ textAlign: 'center' }}>
          <Pagination
            current={currentPage}
            total={total}
            pageSize={pageSize}
            showSizeChanger
            showQuickJumper
            showTotal={(total, range) => 
              `第 ${range[0]}-${range[1]} 条，共 ${total} 条`
            }
            onChange={onPageChange}
            onShowSizeChange={onPageChange}
            pageSizeOptions={['10', '20', '50', '100']}
          />
        </div>
      )}
    </div>
  );
};

export default SearchResults; 