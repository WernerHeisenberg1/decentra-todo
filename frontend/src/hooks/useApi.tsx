import React, { createContext, useContext, useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';
import { ApiContextType } from '../types';

const ApiContext = createContext<ApiContextType | null>(null);

export const useApi = () => {
  const context = useContext(ApiContext);
  if (!context) {
    throw new Error('useApi must be used within an ApiProvider');
  }
  return context;
};

export const ApiProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [api, setApi] = useState<ApiPromise | null>(null);
  const [accounts, setAccounts] = useState<any[]>([]);
  const [selectedAccount, setSelectedAccount] = useState<any>(null);
  const [isConnected, setIsConnected] = useState(false);

  useEffect(() => {
    const connect = async () => {
      try {
        // 连接到本地节点
        const wsProvider = new WsProvider('ws://127.0.0.1:9944');
        const api = await ApiPromise.create({ provider: wsProvider });
        setApi(api);
        setIsConnected(true);

        // 启用 Web3 扩展
        await web3Enable('DecentraTodo');
        const allAccounts = await web3Accounts();
        setAccounts(allAccounts);
      } catch (error) {
        console.error('Failed to connect:', error);
      }
    };

    connect();
  }, []);

  const selectAccount = (account: any) => {
    setSelectedAccount(account);
  };

  const value: ApiContextType = {
    api,
    accounts,
    selectedAccount,
    isConnected,
    connect: async () => {
      try {
        await web3Enable('DecentraTodo');
        const allAccounts = await web3Accounts();
        setAccounts(allAccounts);
      } catch (error) {
        console.error('Failed to connect:', error);
      }
    },
    selectAccount
  };

  return (
    <ApiContext.Provider value={value}>
      {children}
    </ApiContext.Provider>
  );
}; 