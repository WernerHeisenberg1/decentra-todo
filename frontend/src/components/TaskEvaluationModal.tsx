import React, { useState } from 'react';
import { Modal, Form, Rate, Input, Button, message } from 'antd';
import { useApi } from '../hooks/useApi';

const { TextArea } = Input;

interface TaskEvaluationModalProps {
  visible: boolean;
  onClose: () => void;
  taskId: number;
}

const TaskEvaluationModal: React.FC<TaskEvaluationModalProps> = ({
  visible,
  onClose,
  taskId,
}) => {
  const { api, selectedAccount } = useApi();
  const [form] = Form.useForm();
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (values: { rating: number; comment: string }) => {
    if (!api || !selectedAccount) {
      message.error('请先连接钱包');
      return;
    }

    try {
      setLoading(true);
      const tx = api.tx.reputation.evaluateTask(
        taskId,
        values.rating,
        values.comment
      );
      await tx.signAndSend(selectedAccount);
      message.success('任务评价提交成功');
      form.resetFields();
      onClose();
    } catch (error) {
      console.error('提交评价失败:', error);
      message.error('提交评价失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <Modal
      title="评价任务"
      open={visible}
      onCancel={onClose}
      footer={null}
      width={500}
    >
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSubmit}
        initialValues={{ rating: 5 }}
      >
        <Form.Item
          label="评分"
          name="rating"
          rules={[{ required: true, message: '请给出评分' }]}
        >
          <Rate />
        </Form.Item>

        <Form.Item
          label="评价内容"
          name="comment"
          rules={[
            { required: true, message: '请输入评价内容' },
            { max: 500, message: '评价内容不能超过500字符' }
          ]}
        >
          <TextArea
            placeholder="请描述任务完成质量..."
            rows={4}
            maxLength={500}
            showCount
          />
        </Form.Item>

        <Form.Item>
          <div style={{ display: 'flex', justifyContent: 'flex-end', gap: '8px' }}>
            <Button onClick={onClose}>
              取消
            </Button>
            <Button type="primary" htmlType="submit" loading={loading}>
              提交评价
            </Button>
          </div>
        </Form.Item>
      </Form>
    </Modal>
  );
};

export default TaskEvaluationModal; 