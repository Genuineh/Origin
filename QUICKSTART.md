# Origin 快速开始指南

欢迎来到 Origin 项目！本指南将帮助您快速了解项目并开始开发。

## 📖 项目概述

Origin 是一个高性能的通用 UI 渲染引擎，可将设计工具（Figma、Pixso、Sketch 等）的设计文件直接转换为 120fps 的原生应用。

### 核心特点
- 🚀 极致性能：稳定 120fps
- 🎨 像素级还原：完美还原设计稿
- 🌍 全平台支持：Web/Desktop/Mobile
- 🔌 多工具支持：Figma/Pixso/Sketch/Penpot 等
- ⚡ 零延迟交互：基于 GPU 的即时渲染

## 🏗️ 项目结构

```
Origin/
├── crates/              # 引擎核心（Layer 0-6）
│   ├── origin-platform  # 平台抽象
│   ├── origin-gpu       # GPU 抽象
│   ├── origin-sdf       # SDF 渲染
│   ├── origin-instance  # 实例渲染
│   ├── origin-geometry  # 几何计算
│   ├── origin-layout    # 布局引擎
│   ├── origin-input     # 交互处理
│   ├── origin-renderer  # 渲染核心
│   ├── origin-core      # 核心工具
│   └── origin-app       # 应用框架
│
├── adapters/            # 设计工具适配器（Layer 7）
│   ├── origin-adapter-common  # 通用接口
│   ├── origin-figma           # Figma 适配器
│   ├── origin-pixso           # Pixso 适配器
│   ├── origin-sketch          # Sketch 适配器
│   └── origin-penpot          # Penpot 适配器
│
└── docs/                # 文档
```

## 📚 重要文档

在开始之前，请阅读以下文档：

1. **[README.md](README.md)** - 项目概述和特性介绍
2. **[ARCHITECTURE.md](ARCHITECTURE.md)** - 详细的架构设计（必读！）
3. **[ROADMAP.md](ROADMAP.md)** - 开发路线图和时间线
4. **[TODO.md](TODO.md)** - 完整任务列表（280+ 任务）
5. **[CONTRIBUTING.md](CONTRIBUTING.md)** - 贡献指南

### 模块文档

- [引擎模块概览](crates/README.md)
- [适配器概览](adapters/README.md)
- [OriginFigma 文档](adapters/origin-figma/README.md)

## 🚀 快速开始

### 环境准备

#### 1. 安装 Rust

```bash
# 安装 Rust（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 重新加载环境变量
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

#### 2. 克隆项目

```bash
git clone https://github.com/Genuineh/Origin.git
cd Origin
```

#### 3. 安装开发工具（可选）

```bash
# 自动重新编译工具
cargo install cargo-watch

# 依赖管理工具
cargo install cargo-edit

# 性能分析工具
cargo install cargo-flamegraph
```

### 构建项目

```bash
# 构建所有模块
cargo build

# 构建发布版本（优化）
cargo build --release

# 构建特定模块
cargo build --package origin-gpu
```

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --package origin-core

# 显示测试输出
cargo test -- --nocapture
```

### 开发模式

```bash
# 自动监控文件变化并重新编译
cargo watch -x check -x test

# 生成并打开文档
cargo doc --open
```

## 📖 开发流程

### 当前状态

✅ **已完成**：
- 仓库初始化
- 架构设计
- 目录结构
- 文档体系

🚧 **下一步**（Week 1-2）：
- 实现 `origin-platform` (Layer 0)
- 实现 `origin-gpu` (Layer 1)
- 第一个示例：显示空窗口

### 开发顺序

1. **阶段 1**（Week 1-4）：基础架构
   - Layer 0-1: Platform + GPU
   - Layer 2-3: SDF + Instance Rendering

2. **阶段 2**（Week 5-8）：几何和布局
   - Layer 4: Geometry
   - Layer 5: Layout

3. **阶段 3**（Week 9-12）：交互和适配器
   - Layer 6: Input
   - Layer 7: Adapter Interface + OriginFigma

4. **阶段 4**（Week 13-16）：高级功能
   - OriginFigma 完整实现
   - 组件和变体系统

5. **阶段 5**（Week 17-22）：优化和跨平台
   - 性能优化
   - 全平台支持

6. **阶段 6**（Week 23-24）：测试和发布
   - 完整测试
   - 0.1.0 发布

## 🎯 如何参与

### 选择任务

1. 查看 [TODO.md](TODO.md) 找到待完成的任务
2. 查看 [GitHub Issues](https://github.com/Genuineh/Origin/issues)
3. 在 [Discussions](https://github.com/Genuineh/Origin/discussions) 提问

### 开发流程

1. **Fork 项目**
2. **创建分支**：`git checkout -b feature/your-feature`
3. **编写代码**：遵循代码风格，添加测试
4. **提交代码**：使用清晰的提交信息
5. **推送分支**：`git push origin feature/your-feature`
6. **创建 PR**：描述您的更改

详见 [CONTRIBUTING.md](CONTRIBUTING.md)

### 适合新手的任务

- 📝 改进文档
- 🧪 编写测试
- 🐛 修复简单 Bug
- 📊 添加示例代码

## 🔑 关键概念

### 1. 引擎层 vs 适配器层

- **引擎层**：通用渲染能力，不依赖任何设计工具
- **适配器层**：解析特定设计工具的文件格式

### 2. Immediate Mode 渲染

- 每帧重新计算和绘制
- 无状态保留，简化架构
- 提高性能和可预测性

### 3. SDF 渲染

- 使用 Signed Distance Field 渲染矢量图形
- 高质量抗锯齿
- 无损缩放

### 4. Instance Rendering

- GPU 实例化渲染
- 批处理优化
- 最小化绘制调用

## 🛠️ 开发工具推荐

### IDE 和编辑器

- **VS Code** + rust-analyzer
- **RustRover** (JetBrains)
- **Vim/Neovim** + rust.vim

### 调试工具

- `cargo-expand` - 宏展开
- `cargo-asm` - 查看汇编
- `cargo-flamegraph` - 性能分析

### 性能工具

- `cargo bench` - 性能基准测试
- `perf` - Linux 性能分析
- `Instruments` - macOS 性能分析

## 📊 性能目标

- ⚡ **帧率**：稳定 120fps
- 🚀 **启动时间**：< 100ms
- 💾 **内存占用**：< 50MB（基础应用）
- 🔋 **CPU 占用**：< 5%（空闲时）

## 🤝 社区

- GitHub Issues - Bug 报告和功能请求
- GitHub Discussions - 讨论和问答
- Pull Requests - 代码贡献

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE)

## 🙏 致谢

感谢所有为 Origin 项目做出贡献的开发者！

---

**准备好开始了吗？**

1. 阅读 [ARCHITECTURE.md](ARCHITECTURE.md) 了解架构
2. 查看 [TODO.md](TODO.md) 选择任务
3. 参考 [CONTRIBUTING.md](CONTRIBUTING.md) 开始贡献

有问题？在 [Discussions](https://github.com/Genuineh/Origin/discussions) 提问！
