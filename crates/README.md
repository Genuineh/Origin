# Origin 引擎核心模块

本目录包含 Origin 引擎的所有核心模块。这些模块构成了引擎的基础，与具体设计工具无关。

## 模块概览

### 基础层（Layer 0-1）

| 模块 | 层级 | 描述 | 状态 |
|------|------|------|------|
| [origin-platform](origin-platform/) | Layer 0 | 平台抽象层 | 📅 计划中 |
| [origin-gpu](origin-gpu/) | Layer 1 | GPU 抽象层 | 📅 计划中 |

### 渲染层（Layer 2-3）

| 模块 | 层级 | 描述 | 状态 |
|------|------|------|------|
| [origin-sdf](origin-sdf/) | Layer 2 | SDF 渲染层 | 📅 计划中 |
| [origin-instance](origin-instance/) | Layer 3 | 实例渲染层 | 📅 计划中 |

### 几何与布局层（Layer 4-5）

| 模块 | 层级 | 描述 | 状态 |
|------|------|------|------|
| [origin-geometry](origin-geometry/) | Layer 4 | 几何层 | 📅 计划中 |
| [origin-layout](origin-layout/) | Layer 5 | 布局层 | 📅 计划中 |

### 交互层（Layer 6）

| 模块 | 层级 | 描述 | 状态 |
|------|------|------|------|
| [origin-input](origin-input/) | Layer 6 | 交互层 | 📅 计划中 |

### 核心模块

| 模块 | 描述 | 状态 |
|------|------|------|
| [origin-core](origin-core/) | 核心工具和数据结构 | 📅 计划中 |
| [origin-renderer](origin-renderer/) | 渲染器核心 | 📅 计划中 |
| [origin-app](origin-app/) | 应用层框架 | 📅 计划中 |

## 依赖关系

```
origin-app
    │
    ├── origin-renderer
    │   ├── origin-input
    │   ├── origin-layout
    │   │   └── origin-geometry
    │   ├── origin-instance
    │   │   └── origin-sdf
    │   └── origin-gpu
    │       └── origin-platform
    │
    └── origin-core (所有模块依赖)
```

## 开发顺序

按照从底层到高层的顺序开发：

1. **Week 1-2**: Layer 0-1
   - origin-platform
   - origin-gpu
   
2. **Week 3-4**: Layer 2-3
   - origin-sdf
   - origin-instance
   
3. **Week 5-6**: Layer 4
   - origin-geometry
   
4. **Week 7-8**: Layer 5
   - origin-layout
   
5. **Week 9-10**: Layer 6
   - origin-input
   
6. **Week 17-20**: 核心模块
   - origin-renderer
   - origin-core
   - origin-app

## 设计原则

### 1. 层级清晰
- 每层只依赖下层
- 禁止跨层依赖
- 禁止循环依赖

### 2. 接口稳定
- 公共 API 设计要考虑长期稳定性
- 使用 trait 提供抽象
- 版本化接口

### 3. 性能优先
- 零成本抽象
- 最小化运行时开销
- GPU 优先设计

### 4. 可测试性
- 每个模块独立可测试
- 提供 mock 实现
- 完整的测试覆盖

## 构建和测试

```bash
# 构建所有模块
cargo build --workspace

# 测试所有模块
cargo test --workspace

# 构建特定模块
cargo build --package origin-gpu

# 测试特定模块
cargo test --package origin-gpu

# 运行性能基准测试
cargo bench --workspace
```

## 文档

每个模块都应该包含：
- README.md - 模块概述和使用说明
- 完整的 Rustdoc 文档
- 示例代码
- 性能基准测试

生成文档：
```bash
cargo doc --workspace --no-deps --open
```

## 贡献

详见 [贡献指南](../CONTRIBUTING.md)。

## 许可证

MIT
