# CLAUDE_CN.md

此文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

Luxor 是 Python Rich 库的 Rust 实现 - 一个用于在终端中显示富文本和精美格式的库。该项目旨在用 Rust 重新创建 Rich 的功能，并提供更好的性能和内存安全性。

## 仓库结构

**当前实现状态：**
```
luxor/
├── crates/
│   └── luxor/              # 主 Rust 库 crate
│       ├── Cargo.toml      # 包配置和依赖项
│       ├── src/
│       │   ├── lib.rs      # 库入口点和重新导出
│       │   ├── ansi.rs     # ANSI 转义序列生成
│       │   ├── color.rs    # 颜色系统（Standard/8-bit/TrueColor）
│       │   ├── console.rs  # 核心渲染引擎和 Text 结构体
│       │   ├── error.rs    # 使用 thiserror 的错误处理
│       │   ├── measure.rs  # 宽度测量和布局计算
│       │   ├── protocol.rs # 核心特征（Renderable、Measurable）
│       │   ├── segment.rs  # 基础渲染单元
│       │   └── style.rs    # 文本样式和组合
│       ├── tests/
│       │   ├── integration_tests.rs  # 端到端功能测试
│       │   └── property_tests.rs     # 使用 proptest 的属性测试
│       └── benches/
│           ├── render_bench.rs       # 基础基准测试
│           └── comprehensive_bench.rs # 详细性能测试
├── rich/                   # Python Rich 源代码参考
├── CLAUDE.md              # 此文件
└── CLAUDE_CN.md           # 此文件的中文版本
```

## 开发命令

所有命令应在 `crates/luxor/` 目录下运行：

```bash
# 进入主 crate 目录
cd crates/luxor

# 构建库
cargo build

# 运行所有测试（138 个测试：单元 + 集成 + 属性 + 文档测试）
cargo test

# 运行特定测试套件
cargo test --test integration_tests    # 集成测试
cargo test --test property_tests       # 使用 proptest 的属性测试
cargo test --lib                       # 仅单元测试
cargo test --doc                       # 仅文档测试

# 按名称运行特定测试
cargo test test_color_system_compatibility

# 运行测试并显示输出
cargo test -- --nocapture

# 检查代码格式
cargo fmt --check

# 格式化代码
cargo fmt

# 运行 clippy 检查（将警告视为错误）
cargo clippy -- -D warnings

# 构建并打开文档
cargo doc --open

# 运行基准测试
cargo bench

# 运行特定基准测试
cargo bench -- text_rendering
```

## 核心架构

### 已实现第一阶段：核心基础

该库使用基于特征的架构和两阶段渲染：

**核心特征：**
- `Renderable` - 可通过 `render(console, options) -> Vec<Segment>` 渲染为段的对象
- `Measurable` - 可通过 `measure(console, options) -> Measurement` 计算布局维度的对象

**渲染管道：**
1. **测量阶段**：计算最小/最大宽度要求
2. **渲染阶段**：生成带有 ANSI 代码的样式文本段
3. **输出阶段**：将段写入终端并使用适当的转义序列

**关键类型：**
- `Console` - 管理终端状态和选项的中央渲染引擎
- `Segment` - 包含文本 + 样式 + 可选控制代码的基础渲染单元
- `Style` - 具有组合支持的样式属性（颜色、粗体、斜体等）
- `Color` - 多格式颜色系统（Standard 16、8-bit 256、TrueColor 24-bit）
- `Measurement` - 具有最小/最大宽度计算的布局约束

### 模块概述

**核心模块：**
- `protocol.rs` - 定义具有特征对象支持的 `Renderable` 和 `Measurable` 特征
- `color.rs` - 完整的颜色系统，包含 RGB/8-bit/Standard 转换算法
- `style.rs` - 样式组合、继承和字符串解析（"bold red on blue"）
- `segment.rs` - 具有 Unicode 感知宽度计算和分割的文本段
- `console.rs` - 终端管理、渲染管道和 `Text` 结构体
- `measure.rs` - 具有约束求解的布局测量系统
- `ansi.rs` - ANSI 转义序列生成和文本处理实用程序
- `error.rs` - 使用 `thiserror` 的结构化错误处理

**测试：**
- 每个模块中的综合单元测试（62 个测试）
- 端到端功能的集成测试（12 个测试）
- 使用 `proptest` 进行不变量验证的属性测试（14 个测试）
- 确保示例正常工作的文档测试（50 个测试）

## 实现状态

**✅ 第一阶段完成：核心基础**
- 具有 `Renderable`/`Measurable` 协议的基于特征的架构
- 完整的颜色系统（Standard/8-bit/TrueColor）和转换
- 样式组合和继承系统
- Unicode 感知的文本处理和测量
- ANSI 转义序列生成
- 具有终端检测的控制台渲染引擎
- 全面的测试覆盖率（138 个测试通过）
- 严格质量执行下的零 clippy 警告

## 开发路线图 - 完整 Rich 实现

**⚠️ 范围更新：** 在分析完整的 Python Rich 库（54 个模块）后，原始路线图仅覆盖了 Rich 功能的约 30%。此扩展路线图旨在实现完整的功能对等。

### ✅ 第一阶段：核心基础（已完成 - Rich 功能的 15%）

**已实现：**
- 具有特征对象支持的核心 `Renderable` 和 `Measurable` 特征
- 完整的 `Color` 系统，包含 Standard/8-bit/TrueColor 转换算法
- 具有继承和字符串解析的 `Style` 组合系统
- 具有 Unicode 感知文本处理的 `Segment` 渲染单元
- 具有终端能力检测的 `Console` 渲染引擎
- 用于布局约束求解的 `Measurement` 系统
- ANSI 转义序列生成和文本处理实用程序
- 具有结构化类型的综合错误处理

### 🔄 第二阶段：文本和标记系统（4-5 周 - 下一优先级）

**所有其他组件的关键基础：**
- **Rich 标记解析器** - BBCode 样式语法 `[bold red]text[/bold red]`
- **文本跨度** - 文本内的样式范围和适当的组合
- **文字换行** - 具有 Unicode 感知的智能文本流
- **文本对齐** - 左、中、右、两端对齐以及溢出处理
- **高亮器系统** - 基于正则表达式的文本高亮框架
- **增强的 Text 结构** - 具有标记支持的全功能文本处理

### 📋 第三阶段：布局基础（3-4 周）

**复杂 UI 组件的必需基础：**
- **`Layout` 类** - 具有约束的灵活水平/垂直分割
- **`Padding`** - 内容周围的空间管理和各种填充模式
- **`Align`** - 容器内的内容对齐（居中、左、右）
- **`Constrain`** - 大小约束和内容溢出处理
- **`Region`** - 矩形屏幕区域，具有交集/裁剪功能
- **比例解析** - 比例布局分配算法

### 🎨 第四阶段：核心 UI 组件（4-5 周）

**最常用的 Rich 组件：**
- **`Panel`** - 带有标题、副标题和各种框样式的边框容器
- **`Table`** - 功能丰富的表格，具有自动调整大小、排序、样式、边框
- **`Rule`** - 带有样式的水平/垂直线和分隔符
- **`Bar`** - 带有渐变的进度条和数据可视化条

### 🌳 第五阶段：高级组件（4-5 周）

**复杂的交互和显示组件：**
- **`Tree`** - 带有引导线和图标的分层树显示
- **`Columns`** - 具有相等/最佳宽度分布的多列布局
- **`Progress`** - 具有实时更新和 ETA 的多进度条系统
- **`Status`** - 具有可自定义指示器的旋转器动画
- **`Live`** - 具有刷新控制和线程的实时更新显示

### 📝 第六阶段：内容渲染（4-6 周）

**丰富的内容处理和显示：**
- **`Markdown`** - 完整的 markdown 渲染，包含代码块、表格、链接
- **`Pretty`** - 具有语法高亮的 Python 对象美化打印
- **`JSON`** - 带有高亮和验证的美化 JSON 渲染
- **`Repr`** - 带有类型信息的增强对象表示
- **`Syntax`** - 代码语法高亮（子集 - 最常见的语言）
- **`Traceback`** - 带有上下文的美化错误回溯渲染

### ⚡ 第七阶段：交互和动态功能（3-4 周）

**用户交互和动态内容：**
- **`Prompt`** - 具有验证和自动完成的增强输入提示
- **`Screen`** - 全屏应用的备用屏幕缓冲区管理
- **表情符号支持** - 表情符号名称解析（`:smiley:` → 😃）和回退
- **`Inspect`** - 具有交互式探索的对象检查和分析
- **Rich Print** - print() 的替代品，具有自动 rich 格式化
- **输入/输出** - 文件操作和流处理

### 🔧 第八阶段：集成和平台支持（2-3 周）

**导出功能和平台兼容性：**
- **HTML/SVG 导出** - 将 rich 内容导出为带样式的 web 格式
- **主题系统** - 可自定义的颜色主题和样式预设
- **Jupyter 集成** - 在 notebook 中的 Rich 显示和交互功能
- **日志集成** - 带有结构化输出的 Rich 日志处理器
- **平台兼容性** - Windows 控制台支持和终端检测
- **框绘制** - 具有 Unicode 支持的各种框/边框样式

## 更新的技术挑战

### 极高复杂度 (10/10) - 新增

- **Rich 标记解析器** - 具有嵌套标签和验证的复杂 BBCode 样式语法
- **布局约束求解器** - 具有灵活约束的多维布局
- **实时更新系统** - 线程安全的动态内容，最小闪烁
- **终端状态管理** - 复杂的终端控制和能力检测

### 高复杂度 (9/10)

- **Unicode 宽度计算** - 具有表情符号支持的复杂字符宽度规则
- **ANSI 转义序列** - 终端兼容性和颜色处理
- **动态样式** - Rust 所有权模型中的样式继承和组合
- **Markdown/语法渲染** - 具有语法高亮的复杂解析

### 中等复杂度 (6/10)

- **组件组合** - 使用 `Box<dyn Trait>` 进行可渲染集合
- **布局算法** - 灵活布局的约束求解
- **测量系统** - 递归宽度计算
- **内容导出** - 具有适当格式的 HTML/SVG 生成

### 较低复杂度 (3/10)

- **基本渲染** - 字符串构建和输出
- **颜色转换** - RGB/HSL/终端颜色映射
- **简单组件** - 直接的结构体实现

## 现实时间线

**完整 Rich 功能对等：**
- **保守估计：** 26-35 周（6.5-8.5 个月）
- **激进估计：** 20-28 周（5-7 个月）
- **当前第一阶段：** 3 周（已完成）

**MVP 子集（仅核心功能）：**
- **仅第 1-4 阶段：** 12-16 周（3-4 个月）
- **基本组件：** 文本标记、布局、面板、表格、进度、树

## 关键实现模式

### 错误处理
- 在整个 API 中一致使用 `Result<T, LuxorError>`
- 使用 `thiserror` 的结构化错误，提供清晰的错误消息
- 在公共 API 中永不 panic - 对无效输入返回错误

### 样式组合
```rust
// 样式通过分层组合 - 后面的样式覆盖前面的样式
let base = Style::new().bold().color(Color::red());
let overlay = Style::new().italic().color(Color::blue());
let combined = base.combine(overlay); // 蓝色斜体粗体文本
```

### 用于扩展性的特征对象
```rust
// 使用特征对象允许异构集合
let items: Vec<Box<dyn Renderable>> = vec![
    Box::new(Text::new("Hello")),
    Box::new(Panel::new("Content")),
];
```

### 两阶段渲染
```rust
// 所有可渲染对象都遵循 measure -> render 模式
let measurement = item.measure(&console, &options)?;
let segments = item.render(&console, &options)?;
```

## 依赖项

**核心运行时：**
- `crossterm` (0.27) - 跨平台终端操作
- `unicode-width` (0.1) - Unicode 字符宽度计算
- `thiserror` (1.0) - 结构化错误处理

**开发/测试：**
- `criterion` (0.5) - 性能基准测试
- `proptest` (1.0) - 不变量的属性测试

## 开发指南

### 代码质量标准
提交前所有代码必须通过这些检查：

```bash
cd crates/luxor
cargo fmt --check          # 代码格式化
cargo clippy -- -D warnings # 检查（警告视为错误）
cargo test                 # 所有测试必须通过
cargo doc --no-deps        # 文档构建无错误
```

### 测试要求
- **单元测试**：每个模块必须有全面的测试
- **集成测试**：在 `tests/integration_tests.rs` 中的端到端功能
- **属性测试**：在 `tests/property_tests.rs` 中的不变量验证
- **文档测试**：所有公共 API 示例必须工作

### 样式指南
- 对可能失败的操作使用 `Result<T, LuxorError>`
- 在函数参数中优先使用 `&str` 而不是 `String`
- 所有公共 API 必须有带示例的文档
- 始终遵循 Rust 命名约定
- 对异构集合使用特征对象（`Box<dyn Trait>`）

## Python Rich 参考

`rich/` 目录包含完整的 Python Rich 源代码供参考。理解架构的关键文件：

- `rich/console.py` - 核心渲染引擎（~2000+ 行）
- `rich/segment.py` - 基础渲染单元
- `rich/protocol.py` - Renderable 接口定义
- `rich/style.py` - 样式系统实现
- `rich/text.py` - 支持标记的富文本
- `rich/measure.py` - 宽度测量系统

此参考实现有助于确保 API 兼容性并理解复杂的渲染行为。

## 关键缺失组件分析

基于对 Python Rich 库的全面分析，以下是需要立即关注的最关键缺失组件：

### 高优先级缺失功能

**1. Rich 标记解析器（阻塞所有 UI 组件）**
- 当前：仅基本 `Style` 应用
- 缺失：BBCode 样式标记解析 `[bold red]text[/bold red]`
- 影响：Panel 标题、Table 单元格格式化、所有文本渲染都需要此功能
- Rich 中的示例：`console.print("[bold cyan]Hello[/bold cyan] World!")`

**2. 布局系统（阻塞复杂 UI）**
- 当前：无布局管理
- 缺失：`Layout`、`Padding`、`Align`、`Region` 类
- 影响：无法创建 Panel 边框、Table 布局、多列显示
- Rich 中的示例：具有分割面板的灵活终端应用程序

**3. 文本处理基础设施（阻塞文本功能）**
- 当前：基本 `Text` 结构体
- 缺失：文字换行、对齐、溢出处理、文本跨度
- 影响：所有文本密集型组件都将有糟糕的渲染效果
- Rich 中的示例：Table 单元格换行、对齐文本、文本高亮

**4. 核心 UI 组件（面向用户的功能）**
- 当前：未实现任何组件
- 缺失：`Panel`、`Table`、`Tree`、`Progress`、`Rule`
- 影响：无法实现类似 Rich 的可见输出
- Rich 中的示例：Rich 以其美观的终端 UI 而闻名的所有功能

### 中等优先级缺失功能

**5. 实时显示系统**
- 缺失：`Live`、`Status`、动态更新
- 影响：无交互式进度条或动态内容

**6. 内容渲染器**
- 缺失：`Markdown`、`Syntax`、`Pretty`、`JSON`
- 影响：无法显示代码或文档等丰富内容

**7. 高级功能**
- 缺失：主题、导出功能、Jupyter 集成
- 影响：有限的自定义和集成选项

### 当前覆盖评估

**我们拥有的：** 约 15% 的 Rich 功能
- 基本样式和颜色系统
- 简单文本渲染
- ANSI 转义序列生成
- 终端能力检测

**我们缺少的：** 约 85% 的 Rich 功能
- 所有标记解析
- 所有布局管理
- 所有 UI 组件
- 所有内容渲染
- 所有实时/动态功能
- 所有高级集成

### 推荐的即时下一步

1. **实现 Rich 标记解析器**（第 2 阶段）- 解锁其他所有功能
2. **构建布局基础**（第 3 阶段）- 启用复杂 UI
3. **创建 Panel 组件**（第 4 阶段）- 第一个可见的类 Rich 输出
4. **添加 Table 组件**（第 4 阶段）- 最受欢迎的功能
5. **实现进度条**（第 5 阶段）- 受欢迎的交互功能

### 开发策略选项

**选项 A：功能对等（6-8 个月）**
- 实现所有 8 个阶段以完全兼容 Rich
- 最适合需要完整 Rich 功能集的应用程序

**选项 B：核心子集（3-4 个月）**
- 仅专注于第 1-4 阶段（标记、布局、核心组件）
- 对大多数终端 UI 应用程序来说已足够

**选项 C：MVP（1-2 个月）**
- 仅实现标记解析器和 2-3 个核心组件
- 适合概念验证和早期采用者
