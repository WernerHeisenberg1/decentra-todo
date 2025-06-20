import React from 'react';
import { Card, Progress, Row, Col, Typography } from 'antd';
import { TaskStatusCount, TaskPriorityCount, TaskDifficultyCount } from '../types';

const { Text } = Typography;

interface StatisticsChartsProps {
  taskStatusDistribution: TaskStatusCount[];
  priorityDistribution: TaskPriorityCount[];
  difficultyDistribution: TaskDifficultyCount[];
}

const StatisticsCharts: React.FC<StatisticsChartsProps> = ({
  taskStatusDistribution,
  priorityDistribution,
  difficultyDistribution,
}) => {
  // 获取状态颜色
  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Pending':
        return '#faad14'; // 橙色
      case 'InProgress':
        return '#1890ff'; // 蓝色
      case 'Completed':
        return '#52c41a'; // 绿色
      case 'Cancelled':
        return '#ff4d4f'; // 红色
      case 'PendingVerification':
        return '#722ed1'; // 紫色
      default:
        return '#8c8c8c'; // 灰色
    }
  };

  // 获取优先级颜色
  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'Low':
        return '#52c41a'; // 绿色
      case 'Medium':
        return '#faad14'; // 橙色
      case 'High':
        return '#ff7a45'; // 橙红色
      case 'Urgent':
        return '#ff4d4f'; // 红色
      default:
        return '#8c8c8c'; // 灰色
    }
  };

  // 获取状态中文名
  const getStatusName = (status: string) => {
    switch (status) {
      case 'Pending':
        return '待处理';
      case 'InProgress':
        return '进行中';
      case 'Completed':
        return '已完成';
      case 'Cancelled':
        return '已取消';
      case 'PendingVerification':
        return '待验证';
      default:
        return status;
    }
  };

  // 获取优先级中文名
  const getPriorityName = (priority: string) => {
    switch (priority) {
      case 'Low':
        return '低优先级';
      case 'Medium':
        return '中优先级';
      case 'High':
        return '高优先级';
      case 'Urgent':
        return '紧急';
      default:
        return priority;
    }
  };

  return (
    <Row gutter={[16, 16]}>
      {/* 任务状态分布 */}
      <Col xs={24} lg={8}>
        <Card title="任务状态分布" size="small">
          {taskStatusDistribution.map((item) => (
            <div key={item.status} style={{ marginBottom: 16 }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4 }}>
                <Text>{getStatusName(item.status)}</Text>
                <Text>{item.count} 个 ({item.percentage.toFixed(1)}%)</Text>
              </div>
              <Progress
                percent={item.percentage}
                strokeColor={getStatusColor(item.status)}
                showInfo={false}
                size="small"
              />
            </div>
          ))}
        </Card>
      </Col>

      {/* 优先级分布 */}
      <Col xs={24} lg={8}>
        <Card title="优先级分布" size="small">
          {priorityDistribution.map((item) => (
            <div key={item.priority} style={{ marginBottom: 16 }}>
              <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4 }}>
                <Text>{getPriorityName(item.priority)}</Text>
                <Text>{item.count} 个 ({item.percentage.toFixed(1)}%)</Text>
              </div>
              <Progress
                percent={item.percentage}
                strokeColor={getPriorityColor(item.priority)}
                showInfo={false}
                size="small"
              />
            </div>
          ))}
        </Card>
      </Col>

      {/* 难度分布 */}
      <Col xs={24} lg={8}>
        <Card title="难度分布" size="small">
          <div style={{ maxHeight: 280, overflowY: 'auto' }}>
            {difficultyDistribution.map((item) => (
              <div key={item.difficulty} style={{ marginBottom: 12 }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 2 }}>
                  <Text>难度 {item.difficulty}</Text>
                  <Text>{item.count} 个 ({item.percentage.toFixed(1)}%)</Text>
                </div>
                <Progress
                  percent={item.percentage}
                  strokeColor="#722ed1"
                  showInfo={false}
                  size="small"
                />
              </div>
            ))}
          </div>
        </Card>
      </Col>
    </Row>
  );
};

export default StatisticsCharts; 