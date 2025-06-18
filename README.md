# DecentraTodo - 去中心化任务管理与激励平台

DecentraTodo 是一个基于 Substrate 区块链的去中心化任务管理平台，它结合了任务管理、激励机制和社区治理功能。

## 功能特点

- 任务管理：创建、编辑、删除任务
- 任务状态追踪：待处理、进行中、已完成、已取消、待验证
- 优先级系统：低、中、高、紧急
- 难度评估：1-10 级难度评分
- 奖励机制：完成任务获得代币奖励
- 社区验证：任务完成度由社区投票验证
- 声誉系统：基于完成质量的信誉评分
- NFT 成就：里程碑成就的 NFT 证书

## 技术架构

### 后端
- Substrate 区块链框架
- 自定义 Pallet 开发
- 链下工作者集成
- 智能合约自动分配

### 前端
- React + TypeScript
- Polkadot.js API
- Ant Design UI 框架
- 响应式设计

## 快速开始

### 环境要求
- Rust 1.70.0+
- Node.js 16+
- Yarn 或 npm
- Substrate 开发环境

### 安装步骤

1. 克隆仓库
```bash
git clone https://github.com/yourusername/decentra-todo.git
cd decentra-todo
```

2. 编译区块链节点
```bash
cargo build --release
```

3. 启动本地开发节点
```bash
./target/release/decentra-todo --dev
```

4. 安装前端依赖
```bash
cd frontend
yarn install
```

5. 启动前端开发服务器
```bash
yarn start
```

### 使用说明

1. 连接钱包
   - 安装 Polkadot.js 浏览器扩展
   - 创建或导入账户
   - 在应用中连接钱包

2. 创建任务
   - 点击"创建任务"按钮
   - 填写任务信息
   - 设置优先级和难度
   - 提交任务

3. 管理任务
   - 查看任务列表
   - 分配任务给执行者
   - 更新任务状态
   - 验证任务完成

4. 获取奖励
   - 完成任务获得代币奖励
   - 提升声誉等级
   - 解锁 NFT 成就

## 开发指南

### 项目结构
```
decentra-todo/
├── pallets/
│   ├── tasks/           # 任务管理 pallet
│   ├── rewards/         # 奖励系统 pallet  
│   ├── reputation/      # 声誉系统 pallet
│   └── achievements/    # 成就 NFT pallet
├── frontend/           # React 前端
├── tests/              # 集成测试
├── docs/               # 项目文档
└── scripts/            # 部署脚本
```

### 开发流程

1. 本地开发
   - 启动开发节点
   - 运行前端开发服务器
   - 使用测试账户进行测试

2. 测试
   - 运行单元测试：`cargo test`
   - 运行前端测试：`yarn test`
   - 进行集成测试

3. 部署
   - 编译生产版本
   - 配置网络参数
   - 部署节点和前端

## 贡献指南

1. Fork 项目
2. 创建特性分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 联系方式

- 项目维护者：[Your Name]
- 邮箱：[your.email@example.com]
- GitHub：[your-github-profile]
