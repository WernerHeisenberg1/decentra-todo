import React, { useState, useCallback } from 'react';
import { Input, Card, List, Typography, Tag, Space, Button, Empty } from 'antd';
import { SearchOutlined, CloseOutlined } from '@ant-design/icons';
import { Task, TaskStatus } from '../types';

const { Search } = Input;
const { Text, Title } = Typography;

interface QuickSearchProps {
  onSearch: (keyword: string) => Promise<Task[]>;
  onTaskClick?: (taskId: number) => void;
  placeholder?: string;
}

const QuickSearch: React.FC<QuickSearchProps> = ({
  onSearch,
  onTaskClick,
  placeholder = "快速搜索任务...",
}) => {
  const [loading, setLoading] = useState(false);
  const [results, setResults] = useState<Task[]>([]);
  const [showResults, setShowResults] = useState(false);
  const [searchKeyword, setSearchKeyword] = useState('');

  const handleSearch = useCallback(async (value: string) => {
    if (!value.trim()) {
      setResults([]);
      setShowResults(false);
      setSearchKeyword('');
      return;
    }

    try {
      setLoading(true);
      setSearchKeyword(value);
      const searchResults = await onSearch(value);
      setResults(searchResults);
      setShowResults(true);
    } catch (error) {
      console.error('快速搜索失败:', error);
      setResults([]);
    } finally {
      setLoading(false);
    }
  }, [onSearch]);

  const handleClear = () => {
    setResults([]);
    setShowResults(false);
    setSearchKeyword('');
  };

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

  const highlightKeyword = (text: string, keyword: string) => {
    if (!keyword) return text;
    
    const parts = text.split(new RegExp(`(${keyword})`, 'gi'));
    return parts.map((part, index) => 
      part.toLowerCase() === keyword.toLowerCase() ? 
        <mark key={index} style={{ backgroundColor: '#fff566', padding: 0 }}>{part}</mark> : 
        part
    );
  };

  const renderTaskItem = (task: Task) => (
    <List.Item
      key={task.id}
      style={{ cursor: 'pointer', padding: '12px 16px' }}
      onClick={() => onTaskClick?.(task.id)}
    >
      <List.Item.Meta
        title={
          <Space>
            {highlightKeyword(task.title, searchKeyword)}
            <Tag color={getStatusColor(task.status)}>
              {getStatusText(task.status)}
            </Tag>
          </Space>
        }
        description={
          <div>
            <Text type="secondary" style={{ fontSize: '12px' }}>
              {highlightKeyword(
                task.description.length > 80 
                  ? `${task.description.substring(0, 80)}...` 
                  : task.description,
                searchKeyword
              )}
            </Text>
            <div style={{ marginTop: '4px' }}>
              <Space size="small">
                <Text style={{ fontSize: '12px' }}>奖励: {task.reward}</Text>
                <Text style={{ fontSize: '12px' }}>难度: {task.difficulty}/10</Text>
                <Text style={{ fontSize: '12px' }}>创建者: {task.creator}</Text>
              </Space>
            </div>
          </div>
        }
      />
    </List.Item>
  );

  return (
    <div style={{ position: 'relative' }}>
      <Search
        placeholder={placeholder}
        allowClear
        enterButton={<SearchOutlined />}
        size="middle"
        loading={loading}
        onSearch={handleSearch}
        style={{ marginBottom: showResults ? 8 : 0 }}
      />

      {showResults && (
        <Card
          size="small"
          style={{
            position: 'absolute',
            top: '100%',
            left: 0,
            right: 0,
            zIndex: 1000,
            maxHeight: '400px',
            overflow: 'auto',
            boxShadow: '0 4px 12px rgba(0,0,0,0.1)',
          }}
          title={
            <Space>
              <Text strong>搜索结果</Text>
              <Text type="secondary">({results.length})</Text>
              <Button 
                type="text" 
                size="small" 
                icon={<CloseOutlined />}
                onClick={handleClear}
                style={{ marginLeft: 'auto' }}
              />
            </Space>
          }
          headStyle={{ padding: '8px 16px' }}
          bodyStyle={{ padding: 0 }}
        >
          {results.length > 0 ? (
            <List
              dataSource={results}
              renderItem={renderTaskItem}
              split={true}
              style={{ maxHeight: '300px', overflow: 'auto' }}
            />
          ) : (
            <Empty
              image={Empty.PRESENTED_IMAGE_SIMPLE}
              description={`没有找到包含 "${searchKeyword}" 的任务`}
              style={{ margin: '20px 0' }}
            />
          )}
        </Card>
      )}
    </div>
  );
};

export default QuickSearch; 