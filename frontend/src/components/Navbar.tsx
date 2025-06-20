import React from 'react';
import { Layout, Menu, Button, Space, Dropdown, Typography } from 'antd';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import { UserOutlined, DownOutlined } from '@ant-design/icons';
import { useApi } from '../hooks/useApi';
import QuickSearch from './QuickSearch';
import NotificationCenter from './NotificationCenter';
import { Task } from '../types';

const { Header } = Layout;
const { Text } = Typography;

const Navbar: React.FC = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const { selectedAccount, accounts, selectAccount, connect, isConnected } = useApi();

  // 模拟快速搜索API
  const handleQuickSearch = async (keyword: string): Promise<Task[]> => {
    // 模拟网络延迟
    await new Promise(resolve => setTimeout(resolve, 300));
    
    // 模拟搜索结果
    const mockTasks: Task[] = [
      {
        id: 1,
        title: '开发新功能',
        description: '实现用户认证系统，包括登录、注册、密码重置等功能',
        creator: 'alice',
        assignee: null,
        status: 'Pending' as any,
        priority: 'High' as any,
        difficulty: 7,
        reward: 500,
        deadline: Date.now() + 86400000 * 7,
        created_at: Date.now() - 86400000,
        updated_at: Date.now(),
      },
      {
        id: 2,
        title: '修复Bug',
        description: '解决登录页面的显示问题，确保在不同设备上都能正常显示',
        creator: 'bob',
        assignee: 'charlie',
        status: 'InProgress' as any,
        priority: 'Medium' as any,
        difficulty: 4,
        reward: 200,
        deadline: Date.now() + 86400000 * 3,
        created_at: Date.now() - 86400000 * 2,
        updated_at: Date.now(),
      },
    ];

    // 简单的关键词匹配
    return mockTasks.filter(task => 
      task.title.toLowerCase().includes(keyword.toLowerCase()) ||
      task.description.toLowerCase().includes(keyword.toLowerCase())
    );
  };

  const handleTaskClick = (taskId: number) => {
    navigate(`/tasks/${taskId}`);
  };

  const accountMenuItems = accounts.map((account) => ({
    key: account.toString(),
    label: account.toString(),
    onClick: () => selectAccount(account),
  }));

  const menuItems = [
    {
      key: '/dashboard',
      label: <Link to="/dashboard">统计看板</Link>,
    },
    {
      key: '/tasks',
      label: <Link to="/tasks">任务列表</Link>,
    },
    {
      key: '/tasks/create',
      label: <Link to="/tasks/create">创建任务</Link>,
    },
  ];

  return (
    <Header style={{ 
      display: 'flex', 
      alignItems: 'center', 
      justifyContent: 'space-between',
      background: '#fff',
      boxShadow: '0 2px 8px rgba(0,0,0,0.1)',
    }}>
      <div style={{ display: 'flex', alignItems: 'center', flex: 1 }}>
        <div style={{ color: '#1890ff', fontSize: '20px', fontWeight: 'bold', marginRight: '40px' }}>
          任务管理平台
        </div>
        <Menu
          mode="horizontal"
          selectedKeys={[location.pathname]}
          items={menuItems}
          style={{ border: 'none', flex: 1, maxWidth: '300px' }}
        />
      </div>

      {/* 快速搜索 */}
      <div style={{ flex: 1, maxWidth: '400px', margin: '0 20px' }}>
        <QuickSearch 
          onSearch={handleQuickSearch}
          onTaskClick={handleTaskClick}
          placeholder="快速搜索任务..."
        />
      </div>

      <Space>
        {/* 通知中心 */}
        {isConnected && selectedAccount && (
          <NotificationCenter />
        )}
        
        {isConnected && selectedAccount ? (
          <Dropdown 
            menu={{ items: accountMenuItems }}
            trigger={['click']}
          >
            <Button icon={<UserOutlined />}>
              <Space>
                <Text ellipsis style={{ maxWidth: '120px' }}>
                  {selectedAccount.toString()}
                </Text>
                <DownOutlined />
              </Space>
            </Button>
          </Dropdown>
        ) : (
          <Button type="primary" onClick={connect}>
            连接钱包
          </Button>
        )}
      </Space>
    </Header>
  );
};

export default Navbar; 