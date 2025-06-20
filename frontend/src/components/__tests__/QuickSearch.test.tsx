import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import QuickSearch from '../QuickSearch';
import { Task, TaskStatus } from '../../types';

const mockTasks: Task[] = [
  {
    id: 1,
    title: 'React Development Task',
    description: 'Build React components for the frontend',
    creator: 'alice',
    assignee: null,
    status: TaskStatus.Pending,
    priority: 'High' as any,
    difficulty: 7,
    reward: 500,
    deadline: Date.now() + 86400000,
    created_at: Date.now() - 86400000,
    updated_at: Date.now(),
  },
  {
    id: 2,
    title: 'API Integration',
    description: 'Integrate React app with backend API',
    creator: 'bob',
    assignee: 'charlie',
    status: TaskStatus.InProgress,
    priority: 'Medium' as any,
    difficulty: 5,
    reward: 300,
    deadline: Date.now() + 86400000 * 2,
    created_at: Date.now() - 86400000 * 2,
    updated_at: Date.now(),
  },
];

describe('QuickSearch', () => {
  const mockOnSearch = jest.fn();
  const mockOnTaskClick = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  const renderComponent = (props = {}) => {
    const defaultProps = {
      onSearch: mockOnSearch,
      onTaskClick: mockOnTaskClick,
      placeholder: '快速搜索任务...',
      ...props,
    };

    return render(<QuickSearch {...defaultProps} />);
  };

  describe('基础渲染', () => {
    test('应该正确渲染搜索框', () => {
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      expect(searchInput).toBeInTheDocument();
    });

    test('应该使用自定义placeholder', () => {
      renderComponent({ placeholder: '搜索我的任务...' });
      
      const searchInput = screen.getByPlaceholderText('搜索我的任务...');
      expect(searchInput).toBeInTheDocument();
    });
  });

  describe('搜索功能', () => {
    test('应该在输入关键词后调用搜索函数', async () => {
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await userEvent.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await userEvent.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith('React');
    });

    test('应该显示搜索结果', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(screen.getByText('搜索结果')).toBeInTheDocument();
        expect(screen.getByText('React Development Task')).toBeInTheDocument();
        expect(screen.getByText('API Integration')).toBeInTheDocument();
      });
    });

    test('应该高亮显示搜索关键词', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        const highlightedText = screen.getAllByText('React');
        expect(highlightedText.length).toBeGreaterThan(0);
      });
    });

    test('应该显示任务详细信息', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(screen.getByText('奖励: 500')).toBeInTheDocument();
        expect(screen.getByText('难度: 7/10')).toBeInTheDocument();
        expect(screen.getByText('创建者: alice')).toBeInTheDocument();
        expect(screen.getByText('待处理')).toBeInTheDocument();
      });
    });
  });

  describe('任务点击', () => {
    test('应该在点击任务时调用onTaskClick', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        const taskItem = screen.getByText('React Development Task');
        expect(taskItem).toBeInTheDocument();
      });
      
      const taskItem = screen.getByText('React Development Task').closest('.ant-list-item');
      if (taskItem) {
        await user.click(taskItem);
        expect(mockOnTaskClick).toHaveBeenCalledWith(1);
      }
    });
  });

  describe('空结果处理', () => {
    test('应该显示无结果提示', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue([]);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'NotFound');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(screen.getByText(/没有找到包含.*NotFound.*的任务/)).toBeInTheDocument();
      });
    });
  });

  describe('加载状态', () => {
    test('应该显示加载状态', async () => {
      const user = userEvent.setup();
      
      // 模拟延迟的搜索
      mockOnSearch.mockImplementation(() => 
        new Promise(resolve => setTimeout(() => resolve(mockTasks), 100))
      );
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      // 检查加载状态
      expect(searchButton).toHaveClass('ant-btn-loading');
      
      // 等待加载完成
      await waitFor(() => {
        expect(screen.getByText('React Development Task')).toBeInTheDocument();
      });
    });
  });

  describe('关闭功能', () => {
    test('应该能够关闭搜索结果', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(screen.getByText('搜索结果')).toBeInTheDocument();
      });
      
      // 查找并点击关闭按钮
      const closeButton = screen.getByRole('button', { name: /close/i });
      await user.click(closeButton);
      
      expect(screen.queryByText('搜索结果')).not.toBeInTheDocument();
    });
  });

  describe('清空搜索', () => {
    test('应该在清空输入时隐藏结果', async () => {
      const user = userEvent.setup();
      mockOnSearch.mockResolvedValue(mockTasks);
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(screen.getByText('搜索结果')).toBeInTheDocument();
      });
      
      // 清空输入
      await user.clear(searchInput);
      
      // 再次搜索空字符串
      await user.click(searchButton);
      
      expect(screen.queryByText('搜索结果')).not.toBeInTheDocument();
    });
  });

  describe('错误处理', () => {
    test('应该处理搜索错误', async () => {
      const user = userEvent.setup();
      const consoleErrorSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
      
      mockOnSearch.mockRejectedValue(new Error('搜索失败'));
      
      renderComponent();
      
      const searchInput = screen.getByPlaceholderText('快速搜索任务...');
      await user.type(searchInput, 'React');
      
      const searchButton = screen.getByRole('button');
      await user.click(searchButton);
      
      await waitFor(() => {
        expect(consoleErrorSpy).toHaveBeenCalledWith('快速搜索失败:', expect.any(Error));
      });
      
      consoleErrorSpy.mockRestore();
    });
  });
}); 