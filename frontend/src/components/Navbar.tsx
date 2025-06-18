import React from 'react';
import { Layout, Menu, Button } from 'antd';
import { Link, useLocation } from 'react-router-dom';
import { useApi } from '../hooks/useApi';

const { Header } = Layout;

const Navbar: React.FC = () => {
  const location = useLocation();
  const { selectedAccount, connect } = useApi();

  const menuItems = [
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
    <Header style={{ display: 'flex', alignItems: 'center' }}>
      <div style={{ color: 'white', marginRight: '24px' }}>
        DecentraTodo
      </div>
      <Menu
        theme="dark"
        mode="horizontal"
        selectedKeys={[location.pathname]}
        style={{ flex: 1 }}
        items={menuItems}
      />
      <div>
        {selectedAccount ? (
          <span style={{ color: 'white' }}>
            {selectedAccount.toString().slice(0, 6)}...
            {selectedAccount.toString().slice(-4)}
          </span>
        ) : (
          <Button type="primary" onClick={connect}>
            连接钱包
          </Button>
        )}
      </div>
    </Header>
  );
};

export default Navbar; 