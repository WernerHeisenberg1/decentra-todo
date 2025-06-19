import React, { useState, useEffect, useCallback } from 'react';
import { Modal, Button, Card, Progress, Tag, Typography, Spin, message } from 'antd';
import { useApi } from '../hooks/useApi';
import { Task, TaskStatus } from '../types';

const { Title, Text } = Typography;

interface VerificationStatus {
  endBlock: number;
  approveVotes: number;
  rejectVotes: number;
}

interface CommunityVerificationModalProps {
  visible: boolean;
  onClose: () => void;
  taskId: number;
}

const CommunityVerificationModal: React.FC<CommunityVerificationModalProps> = ({
  visible,
  onClose,
  taskId,
}) => {
  const { api, selectedAccount } = useApi();
  const [loading, setLoading] = useState(false);
  const [verificationStatus, setVerificationStatus] = useState<VerificationStatus | null>(null);
  const [currentBlock, setCurrentBlock] = useState<number>(0);
  const [hasVoted, setHasVoted] = useState(false);

  const loadVerificationStatus = useCallback(async () => {
    if (!api || !taskId) return;
    
    try {
      setLoading(true);
      const status = await api.query.tasks.verificationStatus(taskId);
      if (status.isSome) {
        const [endBlock, approveVotes, rejectVotes] = status.unwrap();
        setVerificationStatus({
          endBlock: endBlock.toNumber(),
          approveVotes: approveVotes.toNumber(),
          rejectVotes: rejectVotes.toNumber()
        });
      }
      
      if (selectedAccount) {
        const voted = await api.query.tasks.verificationVotes(taskId, selectedAccount);
        setHasVoted(voted.isSome);
      }
    } catch (error) {
      console.error('Error loading verification status:', error);
    } finally {
      setLoading(false);
    }
  }, [api, taskId, selectedAccount]);

  const loadCurrentBlock = useCallback(async () => {
    if (!api) return;
    
    try {
      const header = await api.rpc.chain.getHeader();
      setCurrentBlock(header.number.toNumber());
    } catch (error) {
      console.error('Error loading current block:', error);
    }
  }, [api]);

  useEffect(() => {
    if (visible) {
      loadVerificationStatus();
      loadCurrentBlock();
    }
  }, [visible, loadVerificationStatus, loadCurrentBlock]);

  const handleVote = async (approve: boolean) => {
    if (!api || !selectedAccount) {
      message.error('请先连接钱包');
      return;
    }

    try {
      setLoading(true);
      const tx = api.tx.tasks.submitVerificationVote(taskId, approve);
      await tx.signAndSend(selectedAccount);
      message.success(`投票${approve ? '赞成' : '反对'}提交成功`);
      await loadVerificationStatus();
    } catch (error) {
      console.error('Error submitting vote:', error);
      message.error('投票提交失败');
    } finally {
      setLoading(false);
    }
  };

  const calculateProgress = () => {
    if (!verificationStatus) return 0;
    const { approveVotes, rejectVotes } = verificationStatus;
    const total = (approveVotes || 0) + (rejectVotes || 0);
    return total > 0 ? ((approveVotes || 0) / total) * 100 : 0;
  };

  const calculateTimeLeft = () => {
    if (!verificationStatus || !currentBlock) return 0;
    const { endBlock } = verificationStatus;
    return Math.max(0, endBlock - currentBlock);
  };

  return (
    <Modal
      title="社区验证"
      open={visible}
      onCancel={onClose}
      footer={null}
      width={600}
    >
      <Spin spinning={loading}>
        {verificationStatus ? (
          <Card>
            <Title level={4}>验证进度</Title>
            <Progress
              percent={calculateProgress()}
              strokeColor="#52c41a"
              trailColor="#ff4d4f"
              format={(percent) => `赞成: ${percent?.toFixed(1)}%`}
            />
            
            <div style={{ margin: '16px 0' }}>
              <Tag color="green">赞成票: {verificationStatus.approveVotes || 0}</Tag>
              <Tag color="red">反对票: {verificationStatus.rejectVotes || 0}</Tag>
            </div>

            <Text>剩余区块: {calculateTimeLeft()}</Text>

            {!hasVoted && selectedAccount && (
              <div style={{ marginTop: 16 }}>
                <Button
                  type="primary"
                  onClick={() => handleVote(true)}
                  style={{ marginRight: 8 }}
                  loading={loading}
                >
                  赞成
                </Button>
                <Button
                  danger
                  onClick={() => handleVote(false)}
                  loading={loading}
                >
                  反对
                </Button>
              </div>
            )}

            {hasVoted && (
              <Text type="secondary">您已经投过票了</Text>
            )}
          </Card>
        ) : (
          <Text>正在加载验证信息...</Text>
        )}
      </Spin>
    </Modal>
  );
};

export default CommunityVerificationModal; 