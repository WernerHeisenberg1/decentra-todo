import React, { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, Button, Space, message, Descriptions, Select } from 'antd';
import { useApi } from '../hooks/useApi';
import { Task, TaskStatus, TaskPriority } from '../types';

const { Option } = Select;

const TaskDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { api, selectedAccount } = useApi();
  const [task, setTask] = useState<Task | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchTask = async () => {
    if (!api || !selectedAccount || !id) return;
    
    setLoading(true);
    try {
      const result = await api.query.tasks.tasks(id);
      if (result.isSome) {
        const taskData = result.unwrap().toJSON() as any;
        setTask({
          id: parseInt(id),
          ...taskData,
        });
      }
    } catch (error) {
      console.error('Failed to fetch task:', error);
      message.error('获取任务详情失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTask();
  }, [api, selectedAccount, id]);

  const handleStatusChange = async (newStatus: TaskStatus) => {
    if (!api || !selectedAccount || !id) return;

    try {
      await api.tx.tasks
        .updateTaskStatus(parseInt(id), newStatus)
        .signAndSend(selectedAccount);
      message.success('状态更新成功');
      fetchTask();
    } catch (error) {
      console.error('Failed to update status:', error);
      message.error('状态更新失败');
    }
  };

  const handleAssign = async () => {
    if (!api || !selectedAccount || !id) return;

    try {
      await api.tx.tasks
        .assignTask(parseInt(id), selectedAccount.toString())
        .signAndSend(selectedAccount);
      message.success('任务分配成功');
      fetchTask();
    } catch (error) {
      console.error('Failed to assign task:', error);
      message.error('任务分配失败');
    }
  };

  if (!task) {
    return <div>加载中...</div>;
  }

  return (
    <div style={{ padding: '24px' }}>
      <Card
        title={`任务 #${task.id}`}
        extra={
          <Space>
            <Button onClick={() => navigate('/tasks')}>返回列表</Button>
          </Space>
        }
      >
        <Descriptions bordered>
          <Descriptions.Item label="标题">{task.title}</Descriptions.Item>
          <Descriptions.Item label="描述">{task.description}</Descriptions.Item>
          <Descriptions.Item label="状态">
            <Select
              value={task.status}
              onChange={handleStatusChange}
              style={{ width: 120 }}
            >
              {Object.values(TaskStatus).map((status) => (
                <Option key={status} value={status}>
                  {status}
                </Option>
              ))}
            </Select>
          </Descriptions.Item>
          <Descriptions.Item label="优先级">{task.priority}</Descriptions.Item>
          <Descriptions.Item label="难度">{task.difficulty}</Descriptions.Item>
          <Descriptions.Item label="奖励">{task.reward}</Descriptions.Item>
          <Descriptions.Item label="创建者">{task.creator}</Descriptions.Item>
          <Descriptions.Item label="执行者">
            {task.assignee || '未分配'}
          </Descriptions.Item>
          <Descriptions.Item label="截止日期">
            {new Date(task.deadline).toLocaleString()}
          </Descriptions.Item>
        </Descriptions>

        {!task.assignee && (
          <div style={{ marginTop: '16px' }}>
            <Button type="primary" onClick={handleAssign}>
              接受任务
            </Button>
          </div>
        )}
      </Card>
    </div>
  );
};

export default TaskDetail; 