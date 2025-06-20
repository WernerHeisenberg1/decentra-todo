import { AccountId } from '@polkadot/types/interfaces';

export interface Task {
  id: number;
  title: string;
  description: string;
  creator: string;
  assignee: string | null;
  status: TaskStatus;
  priority: TaskPriority;
  difficulty: number;
  reward: number;
  deadline: number;
  created_at: number;
  updated_at: number;
}

export enum TaskStatus {
  Pending = 'Pending',
  InProgress = 'InProgress',
  Completed = 'Completed',
  Cancelled = 'Cancelled',
  PendingVerification = 'PendingVerification'
}

export enum TaskPriority {
  Low = 'Low',
  Medium = 'Medium',
  High = 'High',
  Urgent = 'Urgent'
}

export interface TaskFormData {
  title: string;
  description: string;
  priority: TaskPriority;
  difficulty: number;
  reward: number;
  deadline: number;
}

// 声誉相关类型
export enum ReputationLevel {
  Newcomer = 'Newcomer',
  Apprentice = 'Apprentice', 
  Skilled = 'Skilled',
  Expert = 'Expert',
  Master = 'Master',
  Legendary = 'Legendary'
}

export enum TaskRating {
  Poor = 1,
  Fair = 2,
  Good = 3,
  Excellent = 4,
  Perfect = 5
}

export interface UserReputation {
  totalScore: number;
  level: ReputationLevel;
  completedTasks: number;
  cancelledTasks: number;
  totalRatings: number;
  averageRating: number;
  completionRate: number;
  lastUpdated: number;
}

export interface TaskEvaluation {
  taskId: number;
  assignee: string;
  evaluator: string;
  rating: TaskRating;
  evaluatedAt: number;
  comment: string;
}

export interface EvaluationFormData {
  rating: TaskRating;
  comment: string;
}

export interface ApiContextType {
  api: any;
  accounts: AccountId[];
  selectedAccount: AccountId | null;
  isConnected: boolean;
  connect: () => Promise<void>;
  selectAccount: (account: AccountId) => void;
}

// 新增数据统计相关类型
export interface PlatformStatistics {
  totalUsers: number;
  totalTasks: number;
  completedTasks: number;
  pendingTasks: number;
  inProgressTasks: number;
  cancelledTasks: number;
  totalRewards: number;
  averageTaskDifficulty: number;
  completionRate: number;
  activeUsers: number;
  topPerformers: UserPerformance[];
  taskStatusDistribution: TaskStatusCount[];
  priorityDistribution: TaskPriorityCount[];
  difficultyDistribution: TaskDifficultyCount[];
  rewardDistribution: RewardRange[];
  lastUpdated: number;
}

export interface UserPerformance {
  user: string;
  completedTasks: number;
  totalRewards: number;
  averageRating: number;
  reputation: number;
  achievements: number;
}

export interface TaskStatusCount {
  status: TaskStatus;
  count: number;
  percentage: number;
}

export interface TaskPriorityCount {
  priority: TaskPriority;
  count: number;
  percentage: number;
}

export interface TaskDifficultyCount {
  difficulty: number;
  count: number;
  percentage: number;
}

export interface RewardRange {
  range: string;
  count: number;
  totalRewards: number;
  percentage: number;
}

export interface TrendData {
  date: string;
  tasksCreated: number;
  tasksCompleted: number;
  newUsers: number;
  totalRewards: number;
}

export interface DashboardData {
  platformStats: PlatformStatistics;
  trendData: TrendData[];
  userStats: UserReputation;
  recentAchievements: Achievement[];
}

export interface Achievement {
  id: number;
  name: string;
  description: string;
  type: string;
  rarity: string;
  earnedAt: number;
  user?: string;
}

// 图表数据类型
export interface ChartData {
  name: string;
  value: number;
  percentage?: number;
}

export interface LineChartData {
  date: string;
  [key: string]: string | number;
} 