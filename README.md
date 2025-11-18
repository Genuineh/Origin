# Origin

> 高性能通用 UI 渲染引擎 —— 将设计转化为 120fps 原生应用

## 🎯 项目简介

**Origin** 是一个用 Rust 编写的高性能通用 UI 渲染引擎，专为将设计工具（Figma、Pixso、Sketch、Penpot 等）的设计文件直接转换为原生应用而打造。无需手写代码，设计即应用。

### ✨ 核心特性

- 🚀 **极致性能**: 稳定 120fps 渲染，丝滑用户体验
- 🎨 **像素级还原**: 完美还原设计稿的每一个细节
- 🌍 **全平台支持**: Web、Windows、macOS、Linux、Android、iOS 一次构建全平台运行
- 🔌 **多工具支持**: 支持 Figma、Pixso、Sketch、Penpot 等主流设计工具
- ⚡ **零延迟交互**: 基于 GPU 的即时模式渲染
- 📦 **轻量级**: 最小化的二进制体积和运行时开销

### 🏗️ 架构概览

Origin 采用"引擎层 + 适配器层"的分层架构：

```
┌─────────────────────────────────────────┐
│     设计工具适配器层 (Layer 7)          │
│  OriginFigma | OriginPixso | Origin... │
└───────────────────┬─────────────────────┘
                    │
┌───────────────────▼─────────────────────┐
│          Origin 引擎核心                 │
│  • 交互层 (Layer 6)                      │
│  • 布局层 (Layer 5)                      │
│  • 几何层 (Layer 4)                      │
│  • 实例渲染层 (Layer 3)                  │
│  • SDF 渲染层 (Layer 2)                  │
│  • GPU 抽象层 (Layer 1)                  │
│  • 平台抽象层 (Layer 0)                  │
└──────────────────────────────────────────┘
```

### 🎮 快速开始

```bash
# 克隆仓库
git clone https://github.com/Genuineh/Origin.git
cd Origin

# 构建项目
cargo build --release

# 运行示例（使用 Figma 文件）
cargo run --release -- preview design.fig

# 或使用 Pixso 文件
cargo run --release -- preview design.pixso
```

### 📚 支持的设计工具

| 设计工具 | 适配器模块 | 状态 | 文档 |
|---------|-----------|------|------|
| **Figma** | `origin-figma` | 🚧 开发中 | [文档](adapters/origin-figma/README.md) |
| **Pixso** | `origin-pixso` | 📅 计划中 | [文档](adapters/origin-pixso/README.md) |
| **Sketch** | `origin-sketch` | 📅 计划中 | [文档](adapters/origin-sketch/README.md) |
| **Penpot** | `origin-penpot` | 📅 计划中 | [文档](adapters/origin-penpot/README.md) |

更多设计工具支持正在规划中...

### 🛠️ 技术栈

- **核心语言**: Rust
- **图形 API**: wgpu (WebGPU/Vulkan/Metal/DX12)
- **渲染技术**: SDF (Signed Distance Field) + Instance Rendering
- **架构模式**: Immediate Mode (无 Virtual DOM)

### 📖 文档

- [架构设计文档](ARCHITECTURE.md) - 详细的架构设计和实现方案
- [开发路线图](ROADMAP.md) - 项目开发计划和时间线
- [TODO 列表](TODO.md) - 完整的任务清单
- [贡献指南](CONTRIBUTING.md) - 如何参与项目开发

### 🎯 项目目标

**输入**: 一个 Rust 二进制 + 一个设计文件  
**输出**: 120fps、像素级还原、可交互的原生 App

**时间线**: 单人 5.5 个月 / 双人 3.5 个月

### 🤝 如何贡献

我们欢迎各种形式的贡献：

- 🐛 报告 Bug
- 💡 提出新功能建议
- 📝 改进文档
- 🔧 提交代码
- 🎨 添加新的设计工具适配器

详见 [贡献指南](CONTRIBUTING.md)

### 📝 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

### 🌟 为什么选择 Origin？

传统方式：设计 → 切图 → 手写代码 → 调试 → 对齐设计稿 → 重复...

**Origin 方式：设计 → 运行 → 完成！**

- ✅ 无需手写 UI 代码
- ✅ 设计变更即时生效
- ✅ 完美像素级还原
- ✅ 120fps 性能保证
- ✅ 全平台一次构建

### 🔗 相关链接

- [GitHub Issues](https://github.com/Genuineh/Origin/issues)
- [讨论区](https://github.com/Genuineh/Origin/discussions)

---

**注意**: Origin 目前处于早期开发阶段，API 可能会有较大变动。