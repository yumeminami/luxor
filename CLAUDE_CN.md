# CLAUDE_CN.md

此文件为 Claude Code (claude.ai/code) 在此仓库中工作时提供指导。

## 项目概述

Luxor 是 Python Rich 库的 Rust 实现 - 一个用于在终端中显示富文本和精美格式的库。该项目旨在用 Rust 重新创建 Rich 的功能，并提供更好的性能和内存安全性。

## 仓库结构

```
luxor/
├── src/
│   ├── lib.rs              # 主库入口点
│   ├── color.rs            # 颜色系统和颜色类型
│   ├── console.rs          # 核心渲染引擎
│   ├── style.rs            # 文本样式系统
│   ├── segment.rs          # 最小渲染单元
│   ├── text.rs             # 富文本实现
│   ├── protocol.rs         # 核心特征 (Renderable, Measurable)
│   ├── measure.rs          # 宽度测量系统
│   ├── layout.rs           # 布局和定位系统
│   └── components/         # UI 组件
│       ├── table.rs        # 表格组件
│       ├── panel.rs        # 面板组件
│       ├── progress.rs     # 进度条
│       └── ...
├── rich/                   # Python Rich 源代码参考
├── Cargo.toml             # Rust 包配置
└── README.md
```

## 开发设置

### 构建命令

```bash
# 构建项目
cargo build

# 运行测试
cargo test

# 运行基准测试
cargo bench

# 检查代码格式
cargo fmt --check

# 运行 clippy 进行代码检查
cargo clippy -- -D warnings

# 构建文档
cargo doc --open
```

### 开发依赖

- `crossterm` - 跨平台终端操作
- `unicode-width` - Unicode 字符宽度计算
- `serde` - 序列化支持
- `thiserror` - 错误处理
- `syntect` (可选) - 语法高亮
- `criterion` (开发) - 基准测试

## Rich Python 库分析

### 核心架构

Rich 使用分层渲染架构：

1. **Console** - 管理输出流和渲染选项的中央渲染引擎
2. **Segment** - 包含文本、样式和控制代码的最小渲染单元
3. **Renderable 协议** - 对象实现 `__rich_console__()` 或 `__rich__()` 方法
4. **两阶段渲染** - 测量阶段然后是渲染阶段

### 关键组件

**样式系统：**

- `Style` 类管理文本属性（颜色、粗体、斜体等）
- `Color` 类支持标准、8位和真彩色
- `Theme` 类管理样式集合

**布局系统：**

- `Measurement` 计算最小/最大宽度要求
- `Layout` 提供灵活的分割（水平/垂直）
- `Padding` 处理内容周围的间距

**组件系统：**

- `Text` - 支持标记的富文本
- `Table` - 自动调整大小的表格
- `Panel` - 带边框的容器
- `Progress` - 进度条和旋转器
- `Tree`、`Columns`、`Rule` - 其他 UI 组件

**渲染管道：**

1. 对象通过 `__rich_console__()` 实现渲染协议
2. Console 调用测量阶段确定布局要求
3. Console 调用渲染阶段生成 `Segment` 对象
4. Segments 使用 ANSI 代码写入输出流

### 使用的设计模式

- **面向协议设计** - 通过 `__rich__()` 方法进行鸭子类型
- **组合模式** - 组件可以包含其他可渲染对象
- **访问者模式** - Console "访问" 可渲染对象进行测量和渲染
- **策略模式** - 不同的对齐、溢出和对齐策略

## Rust 实现策略

### 第一阶段：核心基础 (2-3 周)

**核心特征：**

```rust
trait Renderable {
    fn render(&self, console: &Console, options: &ConsoleOptions) -> RenderResult;
}

trait Measurable {
    fn measure(&self, console: &Console, options: &ConsoleOptions) -> Measurement;
}
```

**基本类型：**

- 具有颜色、粗体、斜体标志的 `Style` 结构体
- 支持 Standard/EightBit/TrueColor 的 `Color` 枚举
- 具有文本、样式和控制代码的 `Segment` 结构体
- 管理输出和渲染的 `Console` 结构体

### 第二阶段：文本和测量 (2-3 周)

- 支持富文本的 `Text` 实现
- 使用 `unicode-width` crate 进行 Unicode 宽度计算
- 用于布局计算的 `Measurement` 系统
- ANSI 转义序列生成

### 第三阶段：核心组件 (3-4 周)

- 带边框的 `Panel` 组件
- 自动布局的 `Table` 组件
- `Progress` 进度条和旋转器
- 水平线的 `Rule` 组件

### 第四阶段：高级功能 (2-3 周)

- 灵活定位的 `Layout` 系统
- 实时更新支持
- 语法高亮集成
- Markdown 渲染

### 第五阶段：优化 (1-2 周)

- 性能调优
- 内存优化
- 并发安全
- 附加组件

## 技术挑战

### 高复杂度 (9/10)

- **Unicode 宽度计算** - 复杂的字符宽度规则
- **ANSI 转义序列** - 终端兼容性和颜色处理
- **动态样式** - 在 Rust 所有权模型中的样式继承和组合

### 中等复杂度 (6/10)

- **组件组合** - 使用 `Box<dyn Trait>` 进行可渲染集合
- **布局算法** - 灵活布局的约束求解
- **测量系统** - 递归宽度计算

### 较低复杂度 (3/10)

- **基本渲染** - 字符串构建和输出
- **颜色转换** - RGB/HSL/终端颜色映射
- **简单组件** - 直接的结构体实现

## 性能目标

目标比 Python Rich 性能提升 10-100 倍：

- 尽可能使用零成本抽象
- 使用 `String`/`&str` 进行高效字符串处理
- 热点路径中最少的分配
- 在适用的地方为文本处理使用 SIMD 优化

## API 设计原则

1. **熟悉性** - 尽可能保持与 Python Rich 相似的 API
2. **Rust 习惯用法** - 正确使用 Result 类型、迭代器和所有权
3. **零成本** - 抽象而不产生运行时开销
4. **可组合性** - 组件应该无缝协作
5. **线程安全** - 所有类型在合理情况下应该是 Send + Sync

## 测试策略

- 每个组件的单元测试
- 完整渲染管道的集成测试
- 布局算法的基于属性的测试
- 性能回归测试
- 跨平台终端兼容性测试

## 时间估算

**总计**：10-15 周（2.5-4 个月）

- **MVP（核心功能）**：6-8 周
- **功能完整**：10-12 周
- **优化版本**：13-15 周

实现应该是增量式的，每个阶段都产生功能的工作子集。

## 常用开发任务

### 添加新组件

1. 在 `src/components/` 中创建新文件
2. 实现 `Renderable` 和 `Measurable` 特征
3. 添加单元测试
4. 在 `src/lib.rs` 中导出
5. 添加集成测试和示例

### 性能优化

1. 使用 `cargo bench` 识别瓶颈
2. 使用 `cargo flamegraph` 进行分析
3. 优化热点路径
4. 验证改进没有回归

### 调试渲染问题

1. 使用 `Console::debug()` 模式查看 Segments
2. 检查 ANSI 转义序列输出
3. 在不同终端中测试
4. 验证 Unicode 宽度计算

## 贡献指南

- 遵循 Rust 标准格式：`cargo fmt`
- 通过所有检查：`cargo clippy`
- 添加测试覆盖所有新功能
- 更新文档和示例
- 在 PR 中包含性能影响说明