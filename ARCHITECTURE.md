# Origin 架构设计文档

## 项目概述

**Origin** 是一个高性能的通用 UI 渲染引擎，专为将设计工具（如 Figma、Pixso、Sketch、Penpot 等）的设计文件转换为高性能原生应用而打造。

### 核心目标
- **输入**: 一个 Rust 二进制 + 一个设计文件（Figma/Pixso/Sketch/Penpot/...）
- **输出**: 120fps、像素级还原、可交互的原生 App
- **平台支持**: Web/Android/iOS/Windows/macOS/Linux 全平台
- **性能标准**: 120fps 稳定帧率

### 技术栈
- **核心语言**: Rust
- **图形渲染**: wgpu
- **GPU API**: WebGPU/Vulkan/Metal/DX12
- **渲染技术**: SDF (Signed Distance Field)
- **优化策略**: Instance Rendering
- **架构原则**: 禁止一切树、Widget、Element、Retained Mode

### 架构分层
Origin 采用"引擎层 + 适配器层"的架构：
- **引擎层 (Engine Core)**: 通用的渲染引擎，与具体设计工具无关（Layer 0-6）
- **适配器层 (Design Adapters)**: 针对各设计工具的适配器（Layer 7）
  - **OriginFigma**: Figma 适配器
  - **OriginPixso**: Pixso 适配器
  - **OriginSketch**: Sketch 适配器
  - **OriginPenpot**: Penpot 适配器
  - 更多适配器...

## 7+1 层架构详解

Origin 采用清晰的分层架构，每一层都有明确的职责和接口。引擎层（Layer 0-6）提供通用能力，适配器层（Layer 7）针对具体设计工具实现。

---

## 引擎层（Engine Core Layers）

### 第 0 层：Platform Layer（平台抽象层）
**模块**: `origin-platform`

**职责**:
- 窗口创建和管理
- 事件循环抽象
- 平台特定的系统调用封装
- 文件系统访问
- 网络 I/O（可选）

**关键组件**:
- `Window`: 窗口抽象
- `EventLoop`: 事件循环
- `PlatformContext`: 平台上下文
- `FileSystem`: 文件系统访问

**平台支持**:
- Desktop: Windows, macOS, Linux (via winit)
- Mobile: Android, iOS (via winit + platform-specific bindings)
- Web: WASM + WebGPU

---

### 第 1 层：GPU Abstraction Layer（GPU 抽象层）
**模块**: `origin-gpu`

**职责**:
- wgpu 封装和抽象
- GPU 资源管理（Buffer, Texture, Pipeline）
- Shader 编译和管理
- 命令队列管理
- 多后端支持（WebGPU/Vulkan/Metal/DX12）

**关键组件**:
- `GpuContext`: GPU 上下文管理
- `Buffer`: 缓冲区抽象
- `Texture`: 纹理管理
- `Pipeline`: 渲染管线
- `ShaderModule`: Shader 模块
- `CommandEncoder`: 命令编码器

**技术细节**:
- 使用 wgpu 作为统一抽象层
- 支持同步和异步资源加载
- 自动资源池化和重用

---

### 第 2 层：SDF Rendering Layer（SDF 渲染层）
**模块**: `origin-sdf`

**职责**:
- SDF 字段生成和管理
- 矢量图形的 SDF 转换
- 抗锯齿和平滑处理
- 距离场计算优化

**关键组件**:
- `SdfGenerator`: SDF 生成器
- `SdfAtlas`: SDF 图集管理
- `DistanceField`: 距离场数据结构
- `VectorToSdf`: 矢量到 SDF 转换器

**技术特性**:
- 支持复杂路径的 SDF 表示
- 高质量抗锯齿
- GPU 加速的 SDF 计算
- 动态 SDF 更新

---

### 第 3 层：Instance Rendering Layer（实例渲染层）
**模块**: `origin-instance`

**职责**:
- 实例数据批处理
- 渲染批次优化
- GPU Instancing 实现
- 绘制调用最小化

**关键组件**:
- `InstanceBuffer`: 实例缓冲区
- `BatchRenderer`: 批处理渲染器
- `DrawCall`: 绘制调用管理
- `InstanceData`: 实例数据结构

**性能优化**:
- 自动批处理相同材质的对象
- 最小化状态切换
- 优化的实例数据布局
- 动态实例缓冲区管理

---

### 第 4 层：Geometry Layer（几何层）
**模块**: `origin-geometry`

**职责**:
- 几何图形定义和计算
- 路径处理和变换
- 布尔运算
- 碰撞检测

**关键组件**:
- `Shape`: 形状基本类型（矩形、圆形、路径等）
- `Path`: 路径数据结构
- `Transform`: 变换矩阵
- `BoundingBox`: 包围盒计算
- `Tessellator`: 曲面细分器

**支持的图形**:
- 矩形（Rectangle）
- 圆形/椭圆（Circle/Ellipse）
- 路径（Path）
- 多边形（Polygon）
- 贝塞尔曲线（Bezier Curves）

---

### 第 5 层：Layout Layer（布局层）
**模块**: `origin-layout`

**职责**:
- Figma 布局算法实现
- Auto Layout 支持
- 约束求解
- 响应式布局

**关键组件**:
- `LayoutEngine`: 布局引擎
- `AutoLayout`: Auto Layout 实现
- `Constraints`: 约束系统
- `FlexBox`: Flexbox 布局
- `GridLayout`: 网格布局

**布局特性**:
- Figma Auto Layout 完整支持
- 约束和锚点系统
- 响应式尺寸计算
- 嵌套布局处理

---

### 第 6 层：Input Layer（交互层）
**模块**: `origin-input`

**职责**:
- 用户输入处理
- 手势识别
- 交互事件分发
- 动画和过渡

**关键组件**:
- `InputHandler`: 输入处理器
- `GestureRecognizer`: 手势识别
- `EventDispatcher`: 事件分发器
- `Animator`: 动画系统
- `InteractionState`: 交互状态管理

**支持的交互**:
- 点击/触摸
- 拖拽
- 滚动
- 手势（缩放、旋转等）
- 键盘输入
- 悬停效果

---

### 第 7 层：Design Adapter Layer（设计工具适配器层）

**说明**: 这一层不是单一模块，而是针对不同设计工具的多个适配器实现。每个适配器负责解析特定设计工具的文件格式，并将其转换为 Origin 引擎可理解的通用数据结构。

#### 通用适配器接口
**模块**: `origin-adapter-common`

**职责**:
- 定义设计适配器的通用接口
- 提供通用的文档模型抽象
- 共享的解析工具和实用函数

**关键组件**:
- `DesignAdapter` trait: 设计适配器统一接口
- `DesignDocument`: 通用设计文档模型
- `DesignNode` trait: 通用节点抽象
- `StyleSystem`: 通用样式系统
- `ComponentSystem`: 通用组件系统

---

#### OriginFigma（Figma 适配器）
**模块**: `origin-figma`

**职责**:
- Figma 文件解析
- Figma 数据模型实现
- Figma 特有功能支持
- Figma 到 Origin 引擎的转换

**关键组件**:
- `FigmaAdapter`: 实现 `DesignAdapter` trait
- `FigmaParser`: Figma JSON 解析器
- `FigmaDocument`: Figma 文档模型
- `FigmaNode`: Figma 节点类型
- `FigmaComponent`: Figma 组件系统
- `FigmaPrototype`: Figma 原型交互

**Figma 特性支持**:
- 完整的 Figma 节点类型
- 组件和实例
- 变体系统
- Auto Layout
- 原型连接
- 样式和效果
- 布尔运算
- 混合模式

---

#### OriginPixso（Pixso 适配器）
**模块**: `origin-pixso`

**职责**:
- Pixso 文件解析
- Pixso 数据模型实现
- Pixso 特有功能支持
- Pixso 到 Origin 引擎的转换

**关键组件**:
- `PixsoAdapter`: 实现 `DesignAdapter` trait
- `PixsoParser`: Pixso 文件解析器
- `PixsoDocument`: Pixso 文档模型
- Pixso 特有节点和功能实现

**Pixso 特性支持**:
- Pixso 节点类型
- 智能组件
- 交互原型
- 协作功能（可选）

---

#### OriginSketch（Sketch 适配器）
**模块**: `origin-sketch`

**职责**:
- Sketch 文件解析
- Sketch 数据模型实现
- Sketch 到 Origin 引擎的转换

**关键组件**:
- `SketchAdapter`: 实现 `DesignAdapter` trait
- `SketchParser`: Sketch 文件解析器
- `SketchDocument`: Sketch 文档模型

**Sketch 特性支持**:
- Sketch 文档格式（.sketch）
- Symbol 系统
- 样式库
- 插件兼容（可选）

---

#### OriginPenpot（Penpot 适配器）
**模块**: `origin-penpot`

**职责**:
- Penpot 文件解析
- Penpot 数据模型实现
- Penpot 到 Origin 引擎的转换

**关键组件**:
- `PenpotAdapter`: 实现 `DesignAdapter` trait
- `PenpotParser`: Penpot 文件解析器
- `PenpotDocument`: Penpot 文档模型

**Penpot 特性支持**:
- 开源设计格式
- 组件系统
- Flex Layout

---

#### 未来适配器（规划中）
- **OriginAdobeXD**: Adobe XD 适配器
- **OriginProtopie**: ProtoPie 适配器
- **OriginFramer**: Framer 适配器
- **OriginAxure**: Axure 适配器
- **自定义格式**: 支持用户定义的设计格式

---

### 第 7 层设计原则

1. **统一接口**: 所有适配器必须实现 `DesignAdapter` trait
2. **独立开发**: 适配器可独立开发、测试和发布
3. **按需加载**: 只加载需要的适配器，减少二进制体积
4. **可扩展性**: 易于添加新的设计工具支持
5. **版本兼容**: 适配器版本独立于引擎版本

---

### 适配器工作流程

```
设计文件 → DesignAdapter → DesignDocument
    ↓
Layout Engine (L5) → Geometry (L4)
    ↓
Instance Renderer (L3) → SDF Generator (L2)
    ↓
GPU Commands (L1) → Platform Window (L0)
    ↓
Screen Output (120fps)
```

---

### 第 7 层：Figma Runtime Layer（Figma 运行时层）
**模块**: `origin-figma` (OriginFigma)

**职责**:
- Figma 文件解析和适配（详见上方"OriginFigma"部分）
- 转换为通用 DesignDocument

**注意**: 此层已被重构为独立的适配器模块，详细信息请参考上方"第 7 层：Design Adapter Layer"部分。

---

### +1 层：Application Layer（应用层）
**模块**: `origin-app`

**职责**:
- 应用程序入口
- 主渲染循环
- 资源加载和管理
- 应用级配置

**关键组件**:
- `App`: 应用程序主类
- `Runtime`: 运行时环境
- `ResourceManager`: 资源管理器
- `Config`: 配置管理

---

## 核心模块：Renderer（渲染器核心）
**模块**: `origin-renderer`

**职责**:
- 统一渲染管线
- 帧管理和同步
- 渲染优化
- 场景图管理（Immediate Mode）

**关键组件**:
- `Renderer`: 主渲染器
- `RenderPass`: 渲染通道
- `FrameContext`: 帧上下文
- `SceneManager`: 场景管理（Immediate Mode）

---

## 核心模块：Core（核心工具）
**模块**: `origin-core`

**职责**:
- 通用数据结构
- 数学库封装
- 错误处理
- 日志和调试工具

**关键组件**:
- `Vec2`, `Vec3`, `Vec4`: 向量类型
- `Mat3`, `Mat4`: 矩阵类型
- `Color`: 颜色类型
- `Error`: 错误类型
- `Logger`: 日志系统

---

## 数据流

```
设计文件 (Figma/Pixso/Sketch/...) 
    ↓
DesignAdapter (L7 - 适配器层)
    ↓
DesignDocument (通用文档模型)
    ↓
Layout Engine (L5) → Geometry (L4)
    ↓
Instance Renderer (L3) → SDF Generator (L2)
    ↓
GPU Commands (L1) → Platform Window (L0)
    ↓
Screen Output (120fps)
```

---

## 模块依赖关系

```
┌─────────────────────────────────────────────────┐
│          Application Layer (origin-app)         │
│                      (+1 层)                     │
└───────────────────┬─────────────────────────────┘
                    │
    ┌───────────────┼───────────────┐
    │               │               │
┌───▼────┐   ┌──────▼──────┐   ┌───▼────┐
│ Figma  │   │    Pixso    │   │ Sketch │  ... (适配器层 - L7)
│Adapter │   │   Adapter   │   │Adapter │
└───┬────┘   └──────┬──────┘   └───┬────┘
    │               │               │
    └───────────────┼───────────────┘
                    │
        ┌───────────▼───────────┐
        │ origin-adapter-common │  (适配器公共接口)
        └───────────┬───────────┘
                    │
┌───────────────────▼─────────────────────┐
│        Engine Core (引擎层)              │
│  ┌─────────────────────────────────┐    │
│  │   origin-renderer (渲染器核心)   │    │
│  └─────────────────────────────────┘    │
│                                          │
│  Layer 6: origin-input (交互层)          │
│  Layer 5: origin-layout (布局层)         │
│  Layer 4: origin-geometry (几何层)       │
│  Layer 3: origin-instance (实例渲染层)   │
│  Layer 2: origin-sdf (SDF 渲染层)        │
│  Layer 1: origin-gpu (GPU 抽象层)        │
│  Layer 0: origin-platform (平台抽象层)   │
│                                          │
│         origin-core (核心工具)           │
└──────────────────────────────────────────┘
```

---

## 关键设计原则

### 1. 引擎与适配器分离
- 引擎层完全独立于具体设计工具
- 通过统一的 `DesignAdapter` 接口连接
- 适配器可独立开发和维护

### 2. Immediate Mode 渲染
### 2. Immediate Mode 渲染
- 每帧重新计算和绘制
- 无状态保留
- 简化的数据流

### 3. GPU 优先
### 3. GPU 优先
- 最大化 GPU 利用率
- 最小化 CPU-GPU 数据传输
- 批处理优化

### 4. SDF 渲染
### 4. SDF 渲染
- 矢量图形的高质量渲染
- 缩放无损
- 高效的抗锯齿

### 5. 零拷贝设计
### 5. 零拷贝设计
- 直接内存映射
- 避免不必要的数据复制
- 内存池化

### 6. 跨平台一致性
### 6. 跨平台一致性
- 统一的 API
- 平台特定优化
- 一致的渲染结果

### 7. 可扩展性
- 易于添加新的设计工具支持
- 插件式适配器架构
- 向后兼容

---

## 性能目标

- **帧率**: 稳定 120fps
- **启动时间**: < 100ms
- **内存占用**: < 50MB (基础应用)
- **CPU 占用**: < 5% (空闲时)
- **GPU 占用**: 高效利用，无瓶颈

---

## 开发里程碑

详见 [ROADMAP.md](ROADMAP.md)

---

## 贡献指南

详见 [CONTRIBUTING.md](CONTRIBUTING.md)
