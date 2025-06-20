import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { Layout } from 'antd';
import { ApiProvider } from './hooks/useApi';
import Navbar from './components/Navbar';
import TaskList from './pages/TaskList';
import TaskDetail from './pages/TaskDetail';
import CreateTask from './pages/CreateTask';
import Dashboard from './pages/Dashboard';

const { Content } = Layout;

const App: React.FC = () => {
  return (
    <ApiProvider>
      <Router>
        <Layout style={{ minHeight: '100vh' }}>
          <Navbar />
          <Content>
            <Routes>
              <Route path="/dashboard" element={<Dashboard />} />
              <Route path="/tasks" element={<TaskList />} />
              <Route path="/tasks/:id" element={<TaskDetail />} />
              <Route path="/tasks/create" element={<CreateTask />} />
              <Route path="/" element={<Dashboard />} />
            </Routes>
          </Content>
        </Layout>
      </Router>
    </ApiProvider>
  );
};

export default App; 