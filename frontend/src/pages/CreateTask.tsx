import React, { useState } from 'react';
import { Form, Input, Button, InputNumber, Select, DatePicker, message } from 'antd';
import { useNavigate } from 'react-router-dom';
import { useApi } from '../hooks/useApi';
import { TaskPriority, TaskFormData } from '../types';
import dayjs from 'dayjs';

const { Option } = Select;
const { TextArea } = Input;

const CreateTask: React.FC = () => {
  const navigate = useNavigate();
  const { api, selectedAccount } = useApi();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: TaskFormData & { deadline: dayjs.Dayjs }) => {
    if (!api || !selectedAccount) return;

    setLoading(true);
    try {
      const deadline = values.deadline.valueOf();
      await api.tx.tasks
        .createTask(
          values.title,
          values.description,
          values.priority,
          values.difficulty,
          values.reward,
          deadline
        )
        .signAndSend(selectedAccount);
      message.success('任务创建成功');
      navigate('/tasks');
    } catch (error) {
      console.error('Failed to create task:', error);
      message.error('任务创建失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: '24px' }}>
      <h1>创建任务</h1>
      <Form
        layout="vertical"
        onFinish={onFinish}
        style={{ maxWidth: 600 }}
      >
        <Form.Item
          label="标题"
          name="title"
          rules={[{ required: true, message: '请输入任务标题' }]}
        >
          <Input />
        </Form.Item>

        <Form.Item
          label="描述"
          name="description"
          rules={[{ required: true, message: '请输入任务描述' }]}
        >
          <TextArea rows={4} />
        </Form.Item>

        <Form.Item
          label="优先级"
          name="priority"
          rules={[{ required: true, message: '请选择优先级' }]}
        >
          <Select>
            {Object.values(TaskPriority).map((priority) => (
              <Option key={priority} value={priority}>
                {priority}
              </Option>
            ))}
          </Select>
        </Form.Item>

        <Form.Item
          label="难度"
          name="difficulty"
          rules={[{ required: true, message: '请输入难度等级' }]}
        >
          <InputNumber min={1} max={10} />
        </Form.Item>

        <Form.Item
          label="奖励"
          name="reward"
          rules={[{ required: true, message: '请输入奖励金额' }]}
        >
          <InputNumber min={0} />
        </Form.Item>

        <Form.Item
          label="截止日期"
          name="deadline"
          rules={[{ required: true, message: '请选择截止日期' }]}
        >
          <DatePicker showTime />
        </Form.Item>

        <Form.Item>
          <Button type="primary" htmlType="submit" loading={loading}>
            创建任务
          </Button>
          <Button
            style={{ marginLeft: 8 }}
            onClick={() => navigate('/tasks')}
          >
            取消
          </Button>
        </Form.Item>
      </Form>
    </div>
  );
};

export default CreateTask; 