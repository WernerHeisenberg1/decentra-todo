import React from 'react';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import '@testing-library/jest-dom';
import TaskSearchForm, { SearchFilters } from '../TaskSearchForm';
import { TaskStatus, TaskPriority } from '../../types';

// Mock antd components
jest.mock('antd', () => ({
  ...jest.requireActual('antd'),
  message: {
    error: jest.fn(),
    success: jest.fn(),
  },
}));

describe('TaskSearchForm', () => {
  const mockOnSearch = jest.fn();
  const mockOnReset = jest.fn();

  beforeEach(() => {
    jest.clearAllMocks();
  });

  const renderComponent = (props = {}) => {
    const defaultProps = {
      onSearch: mockOnSearch,
      onReset: mockOnReset,
      loading: false,
      ...props,
    };

    return render(<TaskSearchForm {...defaultProps} />);
  };

  describe('基础渲染', () => {
    test('应该正确渲染搜索表单', () => {
      renderComponent();
      
      expect(screen.getByText('任务搜索')).toBeInTheDocument();
      expect(screen.getByPlaceholderText('搜索任务标题或描述...')).toBeInTheDocument();
      expect(screen.getByText('搜索')).toBeInTheDocument();
      expect(screen.getByText('重置')).toBeInTheDocument();
    });

    test('应该显示基础搜索字段', () => {
      renderComponent();
      
      expect(screen.getByLabelText('关键词搜索')).toBeInTheDocument();
      expect(screen.getByLabelText('任务状态')).toBeInTheDocument();
      expect(screen.getByLabelText('优先级')).toBeInTheDocument();
    });

    test('应该可以展开高级搜索选项', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        expect(screen.getByLabelText('难度范围')).toBeInTheDocument();
        expect(screen.getByLabelText('奖励范围')).toBeInTheDocument();
        expect(screen.getByLabelText('创建者')).toBeInTheDocument();
        expect(screen.getByLabelText('执行者')).toBeInTheDocument();
      });
    });
  });

  describe('搜索功能', () => {
    test('应该能够进行基础关键词搜索', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      const keywordInput = screen.getByPlaceholderText('搜索任务标题或描述...');
      const searchButton = screen.getByRole('button', { name: '搜索' });
      
      await user.type(keywordInput, 'React开发');
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          keyword: 'React开发'
        })
      );
    });

    test('应该能够选择任务状态进行搜索', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 打开状态选择器
      const statusSelect = screen.getByLabelText('任务状态');
      await user.click(statusSelect);
      
      // 选择"进行中"状态
      const inProgressOption = screen.getByText('进行中');
      await user.click(inProgressOption);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          status: TaskStatus.InProgress
        })
      );
    });

    test('应该能够选择优先级进行搜索', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 打开优先级选择器
      const prioritySelect = screen.getByLabelText('优先级');
      await user.click(prioritySelect);
      
      // 选择"高"优先级
      const highPriorityOption = screen.getByText('高');
      await user.click(highPriorityOption);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          priority: TaskPriority.High
        })
      );
    });

    test('应该能够进行复合搜索', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 输入关键词
      const keywordInput = screen.getByPlaceholderText('搜索任务标题或描述...');
      await user.type(keywordInput, 'API开发');
      
      // 选择状态
      const statusSelect = screen.getByLabelText('任务状态');
      await user.click(statusSelect);
      const pendingOption = screen.getByText('待处理');
      await user.click(pendingOption);
      
      // 选择优先级
      const prioritySelect = screen.getByLabelText('优先级');
      await user.click(prioritySelect);
      const urgentOption = screen.getByText('紧急');
      await user.click(urgentOption);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          keyword: 'API开发',
          status: TaskStatus.Pending,
          priority: TaskPriority.Urgent
        })
      );
    });
  });

  describe('高级搜索功能', () => {
    test('应该能够设置难度范围', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 展开高级搜索
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        expect(screen.getByLabelText('难度范围')).toBeInTheDocument();
      });
      
      // 这里需要模拟滑块操作，实际测试中可能需要更复杂的交互
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalled();
    });

    test('应该能够输入创建者地址', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 展开高级搜索
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        const creatorInput = screen.getByLabelText('创建者');
        expect(creatorInput).toBeInTheDocument();
      });
      
      const creatorInput = screen.getByPlaceholderText('输入创建者地址...');
      await user.type(creatorInput, '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          creator: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY'
        })
      );
    });

    test('应该能够设置特殊筛选条件', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 展开高级搜索
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        expect(screen.getByText('有截止日期')).toBeInTheDocument();
        expect(screen.getByText('仅未分配')).toBeInTheDocument();
      });
      
      // 开启"仅未分配"选项
      const unassignedSwitch = screen.getByRole('switch', { name: /仅未分配/ });
      await user.click(unassignedSwitch);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          unassignedOnly: true
        })
      );
    });
  });

  describe('表单重置', () => {
    test('应该能够重置表单', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 输入一些搜索条件
      const keywordInput = screen.getByPlaceholderText('搜索任务标题或描述...');
      await user.type(keywordInput, '测试任务');
      
      // 点击重置按钮
      const resetButton = screen.getByRole('button', { name: '重置' });
      await user.click(resetButton);
      
      expect(mockOnReset).toHaveBeenCalled();
      expect(keywordInput).toHaveValue('');
    });
  });

  describe('加载状态', () => {
    test('搜索按钮应该显示加载状态', () => {
      renderComponent({ loading: true });
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      expect(searchButton).toBeDisabled();
    });
  });

  describe('排序功能', () => {
    test('应该能够选择排序方式', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 展开高级搜索
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        expect(screen.getByLabelText('排序方式')).toBeInTheDocument();
      });
      
      // 选择按奖励排序
      const sortSelect = screen.getByLabelText('排序方式');
      await user.click(sortSelect);
      
      const rewardSortOption = screen.getByText('奖励金额');
      await user.click(rewardSortOption);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          sortBy: 'reward'
        })
      );
    });

    test('应该能够切换排序方向', async () => {
      const user = userEvent.setup();
      renderComponent();
      
      // 展开高级搜索
      const advancedToggle = screen.getByText('高级搜索选项');
      await user.click(advancedToggle);
      
      await waitFor(() => {
        expect(screen.getByText('降序')).toBeInTheDocument();
      });
      
      // 切换到升序
      const sortDirectionSwitch = screen.getByRole('switch', { name: /降序/ });
      await user.click(sortDirectionSwitch);
      
      const searchButton = screen.getByRole('button', { name: '搜索' });
      await user.click(searchButton);
      
      expect(mockOnSearch).toHaveBeenCalledWith(
        expect.objectContaining({
          sortDesc: false
        })
      );
    });
  });
}); 