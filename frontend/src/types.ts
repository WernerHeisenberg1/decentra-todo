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