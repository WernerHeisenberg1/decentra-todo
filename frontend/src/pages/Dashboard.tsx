import React, { useState, useEffect, useCallback } from 'react';
import {
  Row,
  Col,
  Card,
  Statistic,
  Typography,
  Table,
  Progress,
  Tag,
  List,
  Avatar,
  Space,
  Spin,
  DatePicker,
  Select,
  Button,
  message,
  Timeline,
} from 'antd';
import {
  DashboardOutlined,
  UserOutlined,
  CheckCircleOutlined,
  ClockCircleOutlined,
  TrophyOutlined,
  FireOutlined,
  RiseOutlined,
  TeamOutlined,
  ProjectOutlined,
  GiftOutlined,
  StarOutlined,
} from '@ant-design/icons';
import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
import {
  PlatformStatistics,
  TrendData,
  Achievement,
  DashboardData,
} from '../types';
import { useApi } from '../hooks/useApi';
import StatisticsCharts from '../components/StatisticsCharts';

dayjs.extend(relativeTime);

const { Title, Text } = Typography;
const { RangePicker } = DatePicker;
const { Option } = Select;

const Dashboard: React.FC = () => {
  const { api, selectedAccount, isConnected } = useApi();
  
  const [dashboardData, setDashboardData] = useState<DashboardData | null>(null);
  const [loading, setLoading] = useState(false);
  const [dateRange, setDateRange] = useState<[dayjs.Dayjs, dayjs.Dayjs]>([
    dayjs().subtract(30, 'day'),
    dayjs(),
  ]);
  const [selectedPeriod, setSelectedPeriod] = useState<string>('30d');

  // 加载统计数据
  const loadDashboardData = useCallback(async () => {
    if (!api && !isConnected) {
      // 即使未连接API也加载模拟数据用于演示
    }
    
    try {
      setLoading(true);
      
      // 模拟API调用 - 在实际项目中应该调用后端API
      const mockPlatformStats: PlatformStatistics = {
        totalUsers: 1247,
        totalTasks: 3856,
        completedTasks: 2891,
        pendingTasks: 423,
        inProgressTasks: 398,
        cancelledTasks: 144,
        totalRewards: 125680,
        averageTaskDifficulty: 5.8,
        completionRate: 75.0,
        activeUsers: 894,
        topPerformers: [
          {
            user: 'alice',
            completedTasks: 89,
            totalRewards: 12500,
            averageRating: 4.8,
            reputation: 2850,
            achievements: 15,
          },
          {
            user: 'bob',
            completedTasks: 76,
            totalRewards: 9800,
            averageRating: 4.6,
            reputation: 2340,
            achievements: 12,
          },
          {
            user: 'charlie',
            completedTasks: 65,
            totalRewards: 8200,
            averageRating: 4.5,
            reputation: 1980,
            achievements: 10,
          },
          {
            user: 'dave',
            completedTasks: 58,
            totalRewards: 7600,
            averageRating: 4.4,
            reputation: 1750,
            achievements: 8,
          },
          {
            user: 'eve',
            completedTasks: 52,
            totalRewards: 6900,
            averageRating: 4.3,
            reputation: 1620,
            achievements: 7,
          },
        ],
        taskStatusDistribution: [
          { status: 'Pending' as any, count: 423, percentage: 11.0 },
          { status: 'InProgress' as any, count: 398, percentage: 10.3 },
          { status: 'Completed' as any, count: 2891, percentage: 75.0 },
          { status: 'Cancelled' as any, count: 144, percentage: 3.7 },
        ],
        priorityDistribution: [
          { priority: 'Low' as any, count: 985, percentage: 25.5 },
          { priority: 'Medium' as any, count: 1654, percentage: 42.9 },
          { priority: 'High' as any, count: 894, percentage: 23.2 },
          { priority: 'Urgent' as any, count: 323, percentage: 8.4 },
        ],
        difficultyDistribution: [
          { difficulty: 1, count: 156, percentage: 4.0 },
          { difficulty: 2, count: 298, percentage: 7.7 },
          { difficulty: 3, count: 485, percentage: 12.6 },
          { difficulty: 4, count: 623, percentage: 16.2 },
          { difficulty: 5, count: 734, percentage: 19.0 },
          { difficulty: 6, count: 598, percentage: 15.5 },
          { difficulty: 7, count: 465, percentage: 12.1 },
          { difficulty: 8, count: 324, percentage: 8.4 },
          { difficulty: 9, count: 123, percentage: 3.2 },
          { difficulty: 10, count: 50, percentage: 1.3 },
        ],
        rewardDistribution: [
          { range: '0-100', count: 1234, totalRewards: 65800, percentage: 32.0 },
          { range: '101-500', count: 1567, totalRewards: 45600, percentage: 40.6 },
          { range: '501-1000', count: 789, totalRewards: 12400, percentage: 20.5 },
          { range: '1000+', count: 266, totalRewards: 1880, percentage: 6.9 },
        ],
        lastUpdated: Date.now(),
      };

      // 模拟趋势数据
      const mockTrendData: TrendData[] = Array.from({ length: 7 }, (_, i) => {
        const date = dayjs().subtract(6 - i, 'day');
        return {
          date: date.format('MM-DD'),
          tasksCreated: Math.floor(Math.random() * 20) + 10,
          tasksCompleted: Math.floor(Math.random() * 25) + 15,
          newUsers: Math.floor(Math.random() * 8) + 2,
          totalRewards: Math.floor(Math.random() * 2000) + 1000,
        };
      });

      // 模拟最近成就
      const mockRecentAchievements: Achievement[] = [
        {
          id: 1,
          name: '任务达人',
          description: '完成100个任务',
          type: 'TaskCompletion',
          rarity: 'Epic',
          earnedAt: Date.now() - 3600000,
          user: 'alice',
        },
        {
          id: 2,
          name: '声誉专家',
          description: '声誉达到2000分',
          type: 'Reputation',
          rarity: 'Rare',
          earnedAt: Date.now() - 7200000,
          user: 'bob',
        },
        {
          id: 3,
          name: '社区守护者',
          description: '参与50次社区验证',
          type: 'Community',
          rarity: 'Rare',
          earnedAt: Date.now() - 10800000,
          user: 'charlie',
        },
      ];

      const dashboardData: DashboardData = {
        platformStats: mockPlatformStats,
        trendData: mockTrendData,
        userStats: {
          totalScore: 1250,
          level: 'Skilled' as any,
          completedTasks: 15,
          cancelledTasks: 2,
          totalRatings: 12,
          averageRating: 4.2,
          completionRate: 88.2,
          lastUpdated: Date.now(),
        },
        recentAchievements: mockRecentAchievements,
      };

      setDashboardData(dashboardData);
    } catch (error) {
      console.error('加载统计数据失败:', error);
      message.error('加载统计数据失败');
    } finally {
      setLoading(false);
    }
  }, [api, isConnected]);

  useEffect(() => {
    loadDashboardData();
  }, [loadDashboardData]);

  // 处理时间范围变化
  const handleDateRangeChange = (dates: any) => {
    if (dates) {
      setDateRange(dates);
    }
  };

  // 处理预设时间段变化
  const handlePeriodChange = (period: string) => {
    setSelectedPeriod(period);
    const now = dayjs();
    let startDate = now;
    
    switch (period) {
      case '7d':
        startDate = now.subtract(7, 'day');
        break;
      case '30d':
        startDate = now.subtract(30, 'day');
        break;
      case '90d':
        startDate = now.subtract(90, 'day');
        break;
      case '1y':
        startDate = now.subtract(1, 'year');
        break;
      default:
        startDate = now.subtract(30, 'day');
    }
    
    setDateRange([startDate, now]);
  };

  // 获取稀有度标签颜色
  const getRarityColor = (rarity: string) => {
    switch (rarity) {
      case 'Common':
        return 'gray';
      case 'Rare':
        return 'blue';
      case 'Epic':
        return 'purple';
      case 'Legendary':
        return 'gold';
      default:
        return 'gray';
    }
  };

  // 表现者表格列配置
  const performerColumns = [
    {
      title: '排名',
      dataIndex: 'index',
      key: 'index',
      width: 60,
      render: (_: any, __: any, index: number) => (
        <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
          {index === 0 && <TrophyOutlined style={{ color: '#faad14' }} />}
          {index === 1 && <TrophyOutlined style={{ color: '#bfbfbf' }} />}
          {index === 2 && <TrophyOutlined style={{ color: '#d4971c' }} />}
          <span>{index + 1}</span>
        </div>
      ),
    },
    {
      title: '用户',
      dataIndex: 'user',
      key: 'user',
      render: (user: string) => (
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <Avatar size="small" icon={<UserOutlined />} />
          <span>{user}</span>
        </div>
      ),
    },
    {
      title: '完成任务',
      dataIndex: 'completedTasks',
      key: 'completedTasks',
    },
    {
      title: '总奖励',
      dataIndex: 'totalRewards',
      key: 'totalRewards',
      render: (value: number) => `${value.toLocaleString()}`,
    },
    {
      title: '平均评分',
      dataIndex: 'averageRating',
      key: 'averageRating',
      render: (value: number) => (
        <div style={{ display: 'flex', alignItems: 'center', gap: 4 }}>
          <StarOutlined style={{ color: '#faad14' }} />
          <span>{value.toFixed(1)}</span>
        </div>
      ),
    },
    {
      title: '声誉',
      dataIndex: 'reputation',
      key: 'reputation',
    },
    {
      title: '成就',
      dataIndex: 'achievements',
      key: 'achievements',
    },
  ];

  if (loading) {
    return (
      <div style={{ padding: 24, textAlign: 'center' }}>
        <Spin size="large" />
        <div style={{ marginTop: 16 }}>
          <Text>正在加载统计数据...</Text>
        </div>
      </div>
    );
  }

  if (!dashboardData) {
    return (
      <div style={{ padding: 24, textAlign: 'center' }}>
        <Text>暂无统计数据</Text>
      </div>
    );
  }

  const { platformStats, trendData, userStats, recentAchievements } = dashboardData;

  return (
    <div style={{ padding: 24, background: '#f5f5f5', minHeight: '100vh' }}>
      {/* 页面标题和控制器 */}
      <div style={{ marginBottom: 24 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 16 }}>
          <div>
            <Title level={2} style={{ margin: 0, display: 'flex', alignItems: 'center', gap: 8 }}>
              <DashboardOutlined />
              数据统计看板
            </Title>
            <Text type="secondary">
              最后更新时间：{dayjs(platformStats.lastUpdated).format('YYYY-MM-DD HH:mm:ss')}
            </Text>
          </div>
          <Space>
            <Select
              value={selectedPeriod}
              onChange={handlePeriodChange}
              style={{ width: 120 }}
            >
              <Option value="7d">最近7天</Option>
              <Option value="30d">最近30天</Option>
              <Option value="90d">最近90天</Option>
              <Option value="1y">最近1年</Option>
            </Select>
            <RangePicker
              value={dateRange}
              onChange={handleDateRangeChange}
              format="YYYY-MM-DD"
            />
            <Button type="primary" onClick={loadDashboardData}>
              刷新数据
            </Button>
          </Space>
        </div>
      </div>

      {/* 核心指标卡片 */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="总用户数"
              value={platformStats.totalUsers}
              prefix={<UserOutlined />}
              suffix="人"
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="总任务数"
              value={platformStats.totalTasks}
              prefix={<ProjectOutlined />}
              suffix="个"
              valueStyle={{ color: '#13c2c2' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="完成率"
              value={platformStats.completionRate}
              prefix={<CheckCircleOutlined />}
              suffix="%"
              precision={1}
              valueStyle={{ color: '#52c41a' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={12} md={6}>
          <Card>
            <Statistic
              title="总奖励"
              value={platformStats.totalRewards}
              prefix={<GiftOutlined />}
              suffix="代币"
              valueStyle={{ color: '#faad14' }}
              formatter={(value) => value?.toLocaleString()}
            />
          </Card>
        </Col>
      </Row>

      {/* 次要指标卡片 */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="活跃用户"
              value={platformStats.activeUsers}
              prefix={<TeamOutlined />}
              suffix="人"
              valueStyle={{ color: '#722ed1' }}
            />
            <div style={{ marginTop: 8 }}>
              <Text type="secondary">占比：{((platformStats.activeUsers / platformStats.totalUsers) * 100).toFixed(1)}%</Text>
            </div>
          </Card>
        </Col>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="平均难度"
              value={platformStats.averageTaskDifficulty}
              prefix={<RiseOutlined />}
              suffix="分"
              precision={1}
              valueStyle={{ color: '#fa8c16' }}
            />
          </Card>
        </Col>
        <Col xs={24} sm={8}>
          <Card>
            <Statistic
              title="进行中任务"
              value={platformStats.inProgressTasks}
              prefix={<ClockCircleOutlined />}
              suffix="个"
              valueStyle={{ color: '#1890ff' }}
            />
          </Card>
        </Col>
      </Row>

      {/* 统计图表 */}
      <div style={{ marginBottom: 24 }}>
        <StatisticsCharts
          taskStatusDistribution={platformStats.taskStatusDistribution}
          priorityDistribution={platformStats.priorityDistribution}
          difficultyDistribution={platformStats.difficultyDistribution}
        />
      </div>

      {/* 趋势数据 */}
      <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
        <Col xs={24} lg={16}>
          <Card title="最近7天趋势" extra={<RiseOutlined />}>
            <Timeline mode="left">
              {trendData.slice(-7).map((data, index) => (
                <Timeline.Item 
                  key={data.date} 
                  color={index === trendData.length - 1 ? 'green' : 'blue'}
                >
                  <div>
                    <Text strong>{data.date}</Text>
                    <div style={{ marginTop: 4 }}>
                      <div>创建任务：{data.tasksCreated} 个</div>
                      <div>完成任务：{data.tasksCompleted} 个</div>
                      <div>新用户：{data.newUsers} 人</div>
                      <div>总奖励：{data.totalRewards.toLocaleString()} 代币</div>
                    </div>
                  </div>
                </Timeline.Item>
              ))}
            </Timeline>
          </Card>
        </Col>
        <Col xs={24} lg={8}>
          <Card title="奖励分布" extra={<GiftOutlined />}>
            <div style={{ marginBottom: 16 }}>
              {platformStats.rewardDistribution.map((range, index) => (
                <div key={range.range} style={{ marginBottom: 12 }}>
                  <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4 }}>
                    <Text>{range.range} 代币</Text>
                    <Text>{range.count} 个</Text>
                  </div>
                  <Progress
                    percent={range.percentage}
                    strokeColor="#1890ff"
                    showInfo={false}
                    size="small"
                  />
                  <div style={{ marginTop: 2 }}>
                    <Text type="secondary" style={{ fontSize: 12 }}>
                      总奖励：{range.totalRewards.toLocaleString()} 代币 ({range.percentage.toFixed(1)}%)
                    </Text>
                  </div>
                </div>
              ))}
            </div>
          </Card>
        </Col>
      </Row>

      {/* 排行榜和最近成就 */}
      <Row gutter={[16, 16]}>
        <Col xs={24} lg={16}>
          <Card title="顶级表现者排行榜" extra={<TrophyOutlined style={{ color: '#faad14' }} />}>
            <Table
              dataSource={platformStats.topPerformers}
              columns={performerColumns}
              pagination={false}
              size="small"
              rowKey="user"
            />
          </Card>
        </Col>
        <Col xs={24} lg={8}>
          <Card title="最近获得的成就" extra={<FireOutlined style={{ color: '#ff4d4f' }} />}>
            <List
              dataSource={recentAchievements}
              renderItem={(achievement) => (
                <List.Item>
                  <List.Item.Meta
                    avatar={<Avatar icon={<TrophyOutlined />} style={{ backgroundColor: '#722ed1' }} />}
                    title={
                      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                        <span>{achievement.name}</span>
                        <Tag color={getRarityColor(achievement.rarity)}>
                          {achievement.rarity}
                        </Tag>
                      </div>
                    }
                    description={
                      <div>
                        <div>{achievement.description}</div>
                        <div style={{ marginTop: 4 }}>
                          <Text type="secondary" style={{ fontSize: 12 }}>
                            {achievement.user} · {dayjs(achievement.earnedAt).fromNow()}
                          </Text>
                        </div>
                      </div>
                    }
                  />
                </List.Item>
              )}
            />
          </Card>
        </Col>
      </Row>
    </div>
  );
};

export default Dashboard; 