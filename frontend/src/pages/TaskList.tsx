import React, { useEffect, useState } from 'react';
import { Table, Button, Space, message } from 'antd';
import { useNavigate } from 'react-router-dom';
import { useApi } from '../hooks/useApi';
import { Task, TaskStatus, TaskPriority } from '../types';

const TaskList: React.FC = () => {
  const { api, selectedAccount } = useApi();
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const navigate = useNavigate();

  const fetchTasks = async () => {
    if (!api || !selectedAccount) return;
    
    setLoading(true);
    try {
      const result = await api.query.tasks.tasks.entries();
      const taskList = result.map(([key, value]) => {
        const task = value.toJSON() as any;
        return {
          id: key.args[0].toNumber(),
          ...task,
        };
      });
      setTasks(taskList);
    } catch (error) {
      console.error('Failed to fetch tasks:', error);
      message.error('获取任务列表失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
  }, [api, selectedAccount]);

  const columns = [
    {
      title: 'ID',
      dataIndex: 'id',
      key: 'id',
    },
    {
      title: '标题',
      dataIndex: 'title',
      key: 'title',
    },
    {
      title: '状态',
      dataIndex: 'status',
      key: 'status',
      render: (status: TaskStatus) => status,
    },
    {
      title: '优先级',
      dataIndex: 'priority',
      key: 'priority',
      render: (priority: TaskPriority) => priority,
    },
    {
      title: '操作',
      key: 'action',
      render: (_: unknown, record: Task) => (
        <Space size="middle">
          <Button type="link" onClick={() => navigate(`/tasks/${record.id}`)}>
            查看详情
          </Button>
        </Space>
      ),
    },
  ];

  return (
    <div style={{ padding: '24px' }}>
      <div style={{ marginBottom: '16px' }}>
        <Button type="primary" onClick={() => navigate('/tasks/create')}>
          创建任务
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={tasks}
        rowKey="id"
        loading={loading}
      />
    </div>
  );
};

export default TaskList; 