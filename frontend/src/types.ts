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

export interface ApiContextType {
  api: any;
  accounts: AccountId[];
  selectedAccount: AccountId | null;
  isConnected: boolean;
  connect: () => Promise<void>;
  selectAccount: (account: AccountId) => void;
} 