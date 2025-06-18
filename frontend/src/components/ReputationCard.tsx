import React from 'react';
import { Card, Progress, Badge, Statistic, Row, Col } from 'antd';
import { StarOutlined, TrophyOutlined, CheckCircleOutlined, CloseCircleOutlined } from '@ant-design/icons';
import { UserReputation, ReputationLevel } from '../types';

interface ReputationCardProps {
  reputation?: UserReputation;
  loading?: boolean;
}

const ReputationCard: React.FC<ReputationCardProps> = ({ reputation, loading = false }) => {
  const getLevelColor = (level: ReputationLevel) => {
    switch (level) {
      case ReputationLevel.Newcomer:
        return '#d9d9d9';
      case ReputationLevel.Apprentice:
        return '#52c41a';
      case ReputationLevel.Skilled:
        return '#1890ff';
      case ReputationLevel.Expert:
        return '#722ed1';
      case ReputationLevel.Master:
        return '#fa8c16';
      case ReputationLevel.Legendary:
        return '#f5222d';
      default:
        return '#d9d9d9';
    }
  };

  const getLevelName = (level: ReputationLevel) => {
    switch (level) {
      case ReputationLevel.Newcomer:
        return '新手';
      case ReputationLevel.Apprentice:
        return '学徒';
      case ReputationLevel.Skilled:
        return '熟练';
      case ReputationLevel.Expert:
        return '专家';
      case ReputationLevel.Master:
        return '大师';
      case ReputationLevel.Legendary:
        return '传奇';
      default:
        return '未知';
    }
  };

  const getNextLevelThreshold = (level: ReputationLevel) => {
    switch (level) {
      case ReputationLevel.Newcomer:
        return 100;
      case ReputationLevel.Apprentice:
        return 300;
      case ReputationLevel.Skilled:
        return 600;
      case ReputationLevel.Expert:
        return 1000;
      case ReputationLevel.Master:
        return 1500;
      default:
        return null; // 已达到最高等级
    }
  };

  const getProgressPercent = () => {
    if (!reputation) return 0;
    
    const nextThreshold = getNextLevelThreshold(reputation.level);
    if (!nextThreshold) return 100; // 已达到最高等级
    
    const currentLevelMin = getCurrentLevelMin(reputation.level);
    return ((reputation.totalScore - currentLevelMin) / (nextThreshold - currentLevelMin)) * 100;
  };

  const getCurrentLevelMin = (level: ReputationLevel) => {
    switch (level) {
      case ReputationLevel.Newcomer:
        return 0;
      case ReputationLevel.Apprentice:
        return 101;
      case ReputationLevel.Skilled:
        return 301;
      case ReputationLevel.Expert:
        return 601;
      case ReputationLevel.Master:
        return 1001;
      case ReputationLevel.Legendary:
        return 1501;
      default:
        return 0;
    }
  };

  if (!reputation) {
    return (
      <Card title="声誉信息" loading={loading}>
        <p>暂无声誉数据</p>
      </Card>
    );
  }

  return (
    <Card
      title={
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <TrophyOutlined />
          声誉信息
        </div>
      }
      loading={loading}
    >
      <div style={{ marginBottom: 16 }}>
        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 8 }}>
          <span>声誉等级</span>
          <Badge
            color={getLevelColor(reputation.level)}
            text={getLevelName(reputation.level)}
            style={{ fontSize: 14, fontWeight: 'bold' }}
          />
        </div>
        
        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          <span>声誉分数:</span>
          <span style={{ fontWeight: 'bold', color: '#1890ff' }}>{reputation.totalScore}</span>
        </div>
        
        <Progress
          percent={getProgressPercent()}
          strokeColor={getLevelColor(reputation.level)}
          format={() => {
            const nextThreshold = getNextLevelThreshold(reputation.level);
            return nextThreshold ? `${reputation.totalScore}/${nextThreshold}` : '满级';
          }}
          style={{ marginTop: 8 }}
        />
      </div>

      <Row gutter={16}>
        <Col span={12}>
          <Statistic
            title="完成任务"
            value={reputation.completedTasks}
            prefix={<CheckCircleOutlined style={{ color: '#52c41a' }} />}
            suffix="个"
          />
        </Col>
        <Col span={12}>
          <Statistic
            title="取消任务"
            value={reputation.cancelledTasks}
            prefix={<CloseCircleOutlined style={{ color: '#ff4d4f' }} />}
            suffix="个"
          />
        </Col>
      </Row>

      <Row gutter={16} style={{ marginTop: 16 }}>
        <Col span={12}>
          <Statistic
            title="平均评分"
            value={reputation.averageRating}
            precision={1}
            prefix={<StarOutlined style={{ color: '#faad14' }} />}
            suffix="分"
          />
        </Col>
        <Col span={12}>
          <Statistic
            title="完成率"
            value={reputation.completionRate * 100}
            precision={1}
            suffix="%"
            valueStyle={{ color: reputation.completionRate >= 0.8 ? '#52c41a' : '#ff4d4f' }}
          />
        </Col>
      </Row>
    </Card>
  );
};

export default ReputationCard; 