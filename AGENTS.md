# Bevywind Agent Guide

`bevywind` 是面向 Bevy UI 的 Tailwind CSS 风格样式库，只为当前 Entity 生成样式 Scene patch；UI 层级继续使用 Bevy 的 `bsn!`。

工作区包含：根 crate（`bstyle!`、`bstyle_r`）、`core`（解析规则）和 `macros`（过程宏）。

## 核心约束

- 有效样式至少生成一个 `bevy::ui::Node`，所有 Node 样式合并到同一个 Node
- 其它样式按 Bevy 要求生成对应Component类型，有对应的样式设置才会生成对应组件，没有不会额外生成Node之外的样式组件
- 组合样式必须先展开为实际属性，再执行重复属性检查
- 新值格式优先复用现有解析逻辑，不重复实现解析器

## 责任边界

bevywind 只负责把静态样式值编译或展开为当前 Entity 在一个瞬时状态下的 Scene patch，包括 Node 字段和样式相关 Component。它不表达 CSS 伪类、状态选择器或过渡动画；不同状态应由 ECS 逻辑选择或应用不同的 `bstyle!` Scene patch。`bsn!` 的 `on(...)` Observer 可以处理事件、修改状态组件或启动动画；交互状态、状态机、条件切换和逐帧过渡仍属于 Bevy ECS 系统，不由样式解析器、`bstyle!` 或 `bstyle_r` 自动监听和处理。

## 宏与运行时

`bstyle!` 和 `bstyle_r` 必须共享 `core::parse_classes`，并对相同成功输入生成等价的最终 `ResolvedScene`。区别仅在计算时机：

- `bstyle!` 在编译期展开，错误产生编译错误
- `bstyle_r` 在运行时展开，错误直接 `panic!`

新增或修改样式时要同时验证两种入口的字段、默认值、Component 和 Node 数量。

## 修改与文档

新增样式时同步检查 `core`、根 crate、`macros`、测试、补全列表和 `docs/`。文档中的语法和值格式必须与代码一致；已实现样式不要标记为“未实现”。

## 验证

```bash
cargo fmt --all -- --check
cargo test
git diff --check
```
