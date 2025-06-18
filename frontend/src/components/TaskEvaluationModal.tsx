import React, { useState } from 'react';
import { Modal, Form, Rate, Input, Button, message } from 'antd';
import { StarOutlined } from '@ant-design/icons';
import { Task, TaskRating, EvaluationFormData } from '../types';

interface TaskEvaluationModalProps {
  visible: boolean;
  task: Task | null;
  onCancel: () => void;
  onSubmit: (taskId: number, data: EvaluationFormData) => Promise<void>;
}

const TaskEvaluationModal: React.FC<TaskEvaluationModalProps> = ({
  visible,
  task,
  onCancel,
  onSubmit,
}) => {
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const ratingDesc = ['差评', '一般', '良好', '优秀', '完美'];

  const handleSubmit = async () => {
    if (!task) return;

    try {
      const values = await form.validateFields();
      setLoading(true);
      
      await onSubmit(task.id, {
        rating: values.rating as TaskRating,
        comment: values.comment || '',
      });
      
      message.success('评价提交成功！');
      handleCancel();
    } catch (error) {
      console.error('提交评价失败:', error);
      message.error('提交评价失败，请重试');
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    form.resetFields();
    onCancel();
  };

  return (
    <Modal
      title={
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <StarOutlined />
          评价任务
        </div>
      }
      open={visible}
      onCancel={handleCancel}
      footer={[
        <Button key="cancel" onClick={handleCancel}>
          取消
        </Button>,
        <Button
          key="submit"
          type="primary"
          loading={loading}
          onClick={handleSubmit}
        >
          提交评价
        </Button>,
      ]}
      width={600}
    >
      {task && (
        <div>
          <div style={{ marginBottom: 20, padding: 16, backgroundColor: '#f5f5f5', borderRadius: 6 }}>
            <h4 style={{ margin: 0, marginBottom: 8 }}>{task.title}</h4>
            <p style={{ margin: 0, color: '#666' }}>执行者: {task.assignee}</p>
          </div>

          <Form
            form={form}
            layout="vertical"
            initialValues={{
              rating: 3, // 默认良好评分
              comment: '',
            }}
          >
            <Form.Item
              label="任务完成质量评分"
              name="rating"
              rules={[
                { required: true, message: '请选择评分' },
                { type: 'number', min: 1, max: 5, message: '评分必须在1-5之间' },
              ]}
            >
              <Rate
                tooltips={ratingDesc}
                style={{ fontSize: 32 }}
              />
            </Form.Item>

            <Form.Item
              label="评价备注"
              name="comment"
              rules={[
                { max: 500, message: '备注不能超过500个字符' },
              ]}
            >
              <Input.TextArea
                placeholder="请描述任务完成情况、质量等（可选）"
                rows={4}
                showCount
                maxLength={500}
              />
            </Form.Item>
          </Form>

          <div style={{ 
            marginTop: 16, 
            padding: 12, 
            backgroundColor: '#e6f7ff', 
            borderRadius: 6,
            border: '1px solid #91d5ff'
          }}>
            <h5 style={{ margin: 0, marginBottom: 8, color: '#1890ff' }}>评分说明：</h5>
            <ul style={{ margin: 0, paddingLeft: 20, color: '#666' }}>
              <li>1分（差评）：任务完成质量很差，有明显问题</li>
              <li>2分（一般）：任务基本完成，但存在一些问题</li>
              <li>3分（良好）：任务完成质量符合要求</li>
              <li>4分（优秀）：任务完成质量很好，超出预期</li>
              <li>5分（完美）：任务完成质量完美，无可挑剔</li>
            </ul>
          </div>
        </div>
      )}
    </Modal>
  );
};

export default TaskEvaluationModal; 