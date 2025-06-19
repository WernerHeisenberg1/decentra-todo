import React, { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { Card, Button, Tag, Typography, Spin, message } from 'antd';
import { Task, TaskStatus } from '../types';
import { useApi } from '../hooks/useApi';
import TaskEvaluationModal from '../components/TaskEvaluationModal';
import CommunityVerificationModal from '../components/CommunityVerificationModal';

const { Title, Text } = Typography;

const TaskDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { api, selectedAccount } = useApi();
  
  const [task, setTask] = useState<Task | null>(null);
  const [loading, setLoading] = useState(true);
  const [showEvaluationModal, setShowEvaluationModal] = useState(false);
  const [showVerificationModal, setShowVerificationModal] = useState(false);

  const fetchTask = useCallback(async () => {
    if (!api || !id) return;
    
    try {
      setLoading(true);
      // 模拟从API获取任务数据
      const mockTask: Task = {
        id: parseInt(id),
        title: '示例任务',
        description: '这是一个示例任务描述',
        creator: 'creator123',
        assignee: 'assignee456',
        status: TaskStatus.Completed,
        priority: 'Medium' as any,
        difficulty: 5,
        reward: 100,
        deadline: Date.now() + 86400000,
        created_at: Date.now() - 86400000,
        updated_at: Date.now(),
      };
      setTask(mockTask);
    } catch (error) {
      console.error('获取任务详情失败:', error);
      message.error('获取任务详情失败');
    } finally {
      setLoading(false);
    }
  }, [api, id]);

  useEffect(() => {
    fetchTask();
  }, [fetchTask]);

  const handleAssignTask = async () => {
    if (!api || !selectedAccount || !task) {
      message.error('请先连接钱包');
      return;
    }

    try {
      const tx = api.tx.tasks.assignTask(task.id, selectedAccount);
      await tx.signAndSend(selectedAccount);
      message.success('任务分配成功');
      await fetchTask();
    } catch (error) {
      console.error('分配任务失败:', error);
      message.error('分配任务失败');
    }
  };

  const handleChangeStatus = async (newStatus: TaskStatus) => {
    if (!api || !selectedAccount || !task) {
      message.error('请先连接钱包');
      return;
    }

    try {
      const statusMap = {
        [TaskStatus.Pending]: 0,
        [TaskStatus.InProgress]: 1,
        [TaskStatus.Completed]: 2,
        [TaskStatus.Cancelled]: 3,
        [TaskStatus.PendingVerification]: 4,
      };

      const tx = api.tx.tasks.changeTaskStatus(task.id, statusMap[newStatus]);
      await tx.signAndSend(selectedAccount);
      message.success('任务状态更新成功');
      await fetchTask();
    } catch (error) {
      console.error('更新任务状态失败:', error);
      message.error('更新任务状态失败');
    }
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

  if (loading) {
    return <Spin size="large" style={{ display: 'block', margin: '50px auto' }} />;
  }

  if (!task) {
    return <div>任务不存在</div>;
  }

  return (
    <div style={{ padding: '24px', maxWidth: '800px', margin: '0 auto' }}>
      <Card>
        <Title level={2}>{task.title}</Title>
        
        <div style={{ marginBottom: '24px' }}>
          <Tag color={getStatusColor(task.status)}>{task.status}</Tag>
          <Tag color="blue">难度: {task.difficulty}/10</Tag>
          <Tag color="green">奖励: {task.reward} tokens</Tag>
        </div>

        <div style={{ marginBottom: '24px' }}>
          <Text strong>描述:</Text>
          <div style={{ marginTop: '8px' }}>{task.description}</div>
        </div>

        <div style={{ marginBottom: '24px' }}>
          <Text strong>创建者:</Text> {task.creator}<br />
          <Text strong>执行者:</Text> {task.assignee || '未分配'}<br />
          <Text strong>截止时间:</Text> {new Date(task.deadline).toLocaleString()}
        </div>

        <div style={{ gap: '8px', display: 'flex', flexWrap: 'wrap' }}>
          {!task.assignee && (
            <Button type="primary" onClick={handleAssignTask}>
              接受任务
            </Button>
          )}
          
          {task.status === TaskStatus.InProgress && (
            <>
              <Button onClick={() => handleChangeStatus(TaskStatus.PendingVerification)}>
                请求社区验证
              </Button>
              <Button onClick={() => handleChangeStatus(TaskStatus.Completed)}>
                直接标记完成
              </Button>
            </>
          )}
          
          {task.status === TaskStatus.Completed && (
            <Button onClick={() => setShowEvaluationModal(true)}>
              评价任务
            </Button>
          )}
          
          {task.status === TaskStatus.PendingVerification && (
            <Button onClick={() => setShowVerificationModal(true)}>
              参与验证
            </Button>
          )}
          
          <Button onClick={() => navigate('/tasks')}>
            返回列表
          </Button>
        </div>
      </Card>

      {showEvaluationModal && (
        <TaskEvaluationModal
          visible={showEvaluationModal}
          onClose={() => setShowEvaluationModal(false)}
          taskId={task.id}
        />
      )}

      {showVerificationModal && (
        <CommunityVerificationModal
          visible={showVerificationModal}
          onClose={() => setShowVerificationModal(false)}
          taskId={task.id}
        />
      )}
    </div>
  );
};

export default TaskDetail; 