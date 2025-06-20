import React, { useState } from 'react';
import {
  Card,
  Form,
  Input,
  Select,
  Slider,
  DatePicker,
  Switch,
  Button,
  Row,
  Col,
  Space,
  Collapse,
} from 'antd';
import { SearchOutlined, ClearOutlined } from '@ant-design/icons';
import { TaskStatus, TaskPriority } from '../types';
import dayjs from 'dayjs';

const { Option } = Select;
const { RangePicker } = DatePicker;
const { Panel } = Collapse;

export interface SearchFilters {
  keyword?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  difficultyRange?: [number, number];
  rewardRange?: [number, number];
  creator?: string;
  assignee?: string;
  deadlineRange?: [dayjs.Dayjs, dayjs.Dayjs] | null;
  createdTimeRange?: [dayjs.Dayjs, dayjs.Dayjs] | null;
  hasDeadline?: boolean;
  unassignedOnly?: boolean;
  sortBy?: string;
  sortDesc?: boolean;
}

interface TaskSearchFormProps {
  onSearch: (filters: SearchFilters) => void;
  onReset: () => void;
  loading?: boolean;
}

const TaskSearchForm: React.FC<TaskSearchFormProps> = ({
  onSearch,
  onReset,
  loading = false,
}) => {
  const [form] = Form.useForm();
  const [showAdvanced, setShowAdvanced] = useState(false);

  const handleSearch = (values: any) => {
    // 处理表单数据
    const filters: SearchFilters = {
      keyword: values.keyword,
      status: values.status,
      priority: values.priority,
      difficultyRange: values.difficultyRange,
      rewardRange: values.rewardRange,
      creator: values.creator,
      assignee: values.assignee,
      deadlineRange: values.deadlineRange,
      createdTimeRange: values.createdTimeRange,
      hasDeadline: values.hasDeadline,
      unassignedOnly: values.unassignedOnly,
      sortBy: values.sortBy || 'created_at',
      sortDesc: values.sortDesc !== false,
    };

    onSearch(filters);
  };

  const handleReset = () => {
    form.resetFields();
    onReset();
  };

  const difficultyMarks = {
    1: '1',
    3: '3',
    5: '5',
    7: '7',
    10: '10',
  };

  return (
    <Card title="任务搜索" style={{ marginBottom: 24 }}>
      <Form
        form={form}
        layout="vertical"
        onFinish={handleSearch}
        initialValues={{
          difficultyRange: [1, 10],
          rewardRange: [0, 10000],
          sortBy: 'created_at',
          sortDesc: true,
        }}
      >
        {/* 基础搜索 */}
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item label="关键词搜索" name="keyword">
              <Input
                placeholder="搜索任务标题或描述..."
                prefix={<SearchOutlined />}
                allowClear
              />
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="任务状态" name="status">
              <Select placeholder="选择状态" allowClear>
                <Option value={TaskStatus.Pending}>待处理</Option>
                <Option value={TaskStatus.InProgress}>进行中</Option>
                <Option value={TaskStatus.Completed}>已完成</Option>
                <Option value={TaskStatus.Cancelled}>已取消</Option>
                <Option value={TaskStatus.PendingVerification}>待验证</Option>
              </Select>
            </Form.Item>
          </Col>
          <Col span={6}>
            <Form.Item label="优先级" name="priority">
              <Select placeholder="选择优先级" allowClear>
                <Option value={TaskPriority.Low}>低</Option>
                <Option value={TaskPriority.Medium}>中</Option>
                <Option value={TaskPriority.High}>高</Option>
                <Option value={TaskPriority.Urgent}>紧急</Option>
              </Select>
            </Form.Item>
          </Col>
        </Row>

        {/* 高级搜索 */}
        <Collapse
          ghost
          onChange={(keys) => setShowAdvanced(keys.length > 0)}
        >
          <Panel header="高级搜索选项" key="advanced">
            <Row gutter={16}>
              <Col span={12}>
                <Form.Item label="难度范围" name="difficultyRange">
                  <Slider
                    range
                    min={1}
                    max={10}
                    marks={difficultyMarks}
                    tipFormatter={(value) => `难度 ${value}`}
                  />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item label="奖励范围" name="rewardRange">
                  <Slider
                    range
                    min={0}
                    max={10000}
                    step={100}
                    tipFormatter={(value) => `${value} tokens`}
                  />
                </Form.Item>
              </Col>
            </Row>

            <Row gutter={16}>
              <Col span={8}>
                <Form.Item label="创建者" name="creator">
                  <Input placeholder="输入创建者地址..." />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item label="执行者" name="assignee">
                  <Input placeholder="输入执行者地址..." />
                </Form.Item>
              </Col>
              <Col span={8}>
                <Form.Item label="排序方式" name="sortBy">
                  <Select defaultValue="created_at">
                    <Option value="created_at">创建时间</Option>
                    <Option value="updated_at">更新时间</Option>
                    <Option value="deadline">截止时间</Option>
                    <Option value="reward">奖励金额</Option>
                    <Option value="difficulty">难度</Option>
                    <Option value="priority">优先级</Option>
                  </Select>
                </Form.Item>
              </Col>
            </Row>

            <Row gutter={16}>
              <Col span={12}>
                <Form.Item label="截止日期范围" name="deadlineRange">
                  <RangePicker
                    showTime
                    format="YYYY-MM-DD HH:mm:ss"
                    placeholder={['开始时间', '结束时间']}
                    style={{ width: '100%' }}
                  />
                </Form.Item>
              </Col>
              <Col span={12}>
                <Form.Item label="创建时间范围" name="createdTimeRange">
                  <RangePicker
                    showTime
                    format="YYYY-MM-DD HH:mm:ss"
                    placeholder={['开始时间', '结束时间']}
                    style={{ width: '100%' }}
                  />
                </Form.Item>
              </Col>
            </Row>

            <Row gutter={16}>
              <Col span={6}>
                <Form.Item name="hasDeadline" valuePropName="checked">
                  <Switch checkedChildren="有截止日期" unCheckedChildren="全部" />
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item name="unassignedOnly" valuePropName="checked">
                  <Switch checkedChildren="仅未分配" unCheckedChildren="全部" />
                </Form.Item>
              </Col>
              <Col span={6}>
                <Form.Item name="sortDesc" valuePropName="checked">
                  <Switch checkedChildren="降序" unCheckedChildren="升序" />
                </Form.Item>
              </Col>
            </Row>
          </Panel>
        </Collapse>

        {/* 操作按钮 */}
        <Row justify="end" style={{ marginTop: 16 }}>
          <Space>
            <Button onClick={handleReset} icon={<ClearOutlined />}>
              重置
            </Button>
            <Button
              type="primary"
              htmlType="submit"
              loading={loading}
              icon={<SearchOutlined />}
            >
              搜索
            </Button>
          </Space>
        </Row>
      </Form>
    </Card>
  );
};

export default TaskSearchForm; 