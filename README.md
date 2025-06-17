# DecentraTodo 🚀

**去中心化任务管理与激励平台**

基于 Substrate 构建的创新型区块链项目，将传统任务管理与区块链激励机制相结合。

## 🎯 项目亮点

### 核心功能
- **任务管理系统** - 创建、编辑、删除和分配任务
- **智能激励机制** - 完成任务获得代币奖励
- **去中心化验证** - 社区投票验证任务完成度
- **声誉系统** - 基于完成质量的信誉评分
- **NFT 成就徽章** - 里程碑成就的 NFT 证书

### 技术特色
- **模块化架构** - 多个自定义 Pallet 设计
- **链下工作者** - 定时任务和外部数据集成
- **完整测试覆盖** - 单元测试与集成测试
- **现代化前端** - React + Polkadot.js 用户界面

## 🏗️ 架构设计

### Pallets 结构
```
pallets/
├── tasks/           # 任务管理核心功能
├── rewards/         # 奖励分发系统
├── reputation/      # 声誉评分系统
└── achievements/    # NFT 成就系统
```

### 开发阶段
- **Phase 1** ✅ 基础任务管理
- **Phase 2** 🔄 激励机制
- **Phase 3** ⏳ 社区功能
- **Phase 4** ⏳ 高级特性

## 🚀 快速开始

### 环境要求
- Rust 1.70+
- Substrate 开发环境
- Node.js 16+ (前端开发)

### 构建项目
```bash
# 克隆项目
git clone https://github.com/your-username/decentra-todo.git
cd decentra-todo

# 构建节点
cargo build --release

# 运行开发节点
./target/release/solochain-template-node --dev
```

### 连接前端
- 访问 [Polkadot.js Apps](https://polkadot.js.org/apps/)
- 连接到本地节点：`ws://127.0.0.1:9944`

## 📚 功能说明

### 任务管理 (Phase 1)
- ✅ 创建任务 (`create_task`)
- ✅ 更新任务状态 (`update_task_status`)
- ✅ 分配任务 (`assign_task`)
- ✅ 删除任务 (`delete_task`)

### 任务状态流转
```
Pending → InProgress → Completed
   ↓           ↓
Cancelled   Cancelled
```

### 任务属性
- **基础信息**: 标题、描述、创建者
- **管理属性**: 状态、优先级、分配者
- **激励属性**: 难度等级 (1-10)、预期奖励
- **时间属性**: 创建时间、更新时间

## 🔧 开发指南

### 添加新的 Pallet
1. 在 `pallets/` 目录创建新模块
2. 更新 `Cargo.toml` 的 `members` 和 `dependencies`
3. 在 `runtime/src/lib.rs` 中配置 Pallet

### 测试
```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test --features runtime-benchmarks
```

## 🎨 面试展示亮点

### 技术深度
- **自定义 Pallet 开发**: 展示 Substrate 框架深度理解
- **状态管理**: 复杂的链上数据结构设计
- **权限控制**: 细粒度的访问控制机制

### 创新性
- **实用性**: 解决真实世界的任务管理问题
- **激励机制**: 区块链原生的经济模型设计
- **社区驱动**: 去中心化的质量保证机制

### 工程实践
- **模块化设计**: 可扩展的架构
- **完整文档**: 详细的开发和使用文档
- **测试覆盖**: 保证代码质量

## 🤝 贡献指南

欢迎社区贡献！请查看我们的 [Contributing Guidelines](CONTRIBUTING.md)。

### 开发流程
1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT-0 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [Substrate 官方文档](https://docs.substrate.io/)
- [Polkadot.js 文档](https://polkadot.js.org/docs/)
- [项目演示视频](https://your-demo-link.com)

## 👥 团队

- **架构设计**: DecentraTodo 核心团队
- **技术实现**: Substrate 专家团队
- **产品设计**: 区块链 UX 团队

---

**Built with ❤️ on Substrate**
