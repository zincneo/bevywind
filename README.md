# bevywind

`bevywind` 是一个面向 Bevy UI 的 Tailwind CSS 风格样式库。

它只负责为当前 UI 实体生成样式 patch，不负责创建子节点或描述 UI 层级。UI 结构仍然使用 Bevy 的 `bsn!` 编写。

## 使用方式

```rust
use bevy::prelude::*;
use bevywind::bstyle;

fn scene() -> impl Scene {
    bsn! {
        bstyle!(w-full h-full)

        Children [
            Text("Hello")
        ]
    }
}
```

`bstyle!` 是过程宏。样式 token 会在编译期解析，解析失败时会产生编译期错误：

```rust
bstyle!(h-10px w-50%)
```

`bstyle!` 只接受样式 token，不接受字符串；它要求至少有一个样式，不支持空调用：

```rust
bstyle! {}       // 不支持
bstyle!("")      // 不支持
```

## 动态样式

运行时才确定的样式使用 `style_runtime`：

```rust
use bevywind::style_runtime;

fn scene(classes: &String) -> impl Scene {
    bsn! {
        style_runtime(classes)
    }
}
```

`style_runtime` 接收实现 `AsRef<str>` 的值，例如 `&str`、`String` 和 `&String`。动态样式会在运行时解析。

## LSP

仓库包含独立的 `bevywind-lsp` 可执行程序，为 Rust 文件中的 token-style `bstyle!(...)` 提供样式合法性错误提示。它不是 `bevywind` 库的依赖。

构建：

```bash
cargo build --manifest-path bevywind-lsp/Cargo.toml --release
```

安装到 Cargo 的可执行程序目录：

```bash
cargo install --path bevywind-lsp
```

Helix 可以在 `~/.config/helix/languages.toml` 中配置：

```toml
[language-server.bevywind-lsp]
command = "bevywind-lsp"

[[language]]
name = "rust"
language-servers = [
    { name = "bevywind-lsp", only-features = ["diagnostics"] },
    "rust-analyzer",
]
```

随后即可使用：

```rust
bstyle!(h-100px w-50w min-h-50%)
```

## 可用样式

### 高度和宽度

当前支持 `h-*` 高度和 `w-*` 宽度样式：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `h-full` | `height: percent(100)` | 高度占父节点 100% |
| `w-full` | `width: percent(100)` | 宽度占父节点 100% |
| `h-10px` | `height: px(10)` | 高度为 10 逻辑像素 |
| `w-20px` | `width: px(20)` | 宽度为 20 逻辑像素 |
| `h-30%` | `height: percent(30)` | 高度为父节点的 30% |
| `w-40%` | `width: percent(40)` | 宽度为父节点的 40% |
| `h-50w` | `height: vw(50)` | 高度为视口宽度的 50% |
| `w-60w` | `width: vw(60)` | 宽度为视口宽度的 60% |
| `h-70h` | `height: vh(70)` | 高度为视口高度的 70% |
| `w-80h` | `width: vh(80)` | 宽度为视口高度的 80% |

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。

最小和最大尺寸使用与高度、宽度相同的格式：

```rust
bstyle!(min-h-100px min-w-20% max-h-80w max-w-90h)
```

它们分别对应 Bevy `Node` 的 `min_height`、`min_width`、`max_height` 和 `max_width` 属性。

### Flex 布局（待实现）

Flex 样式只描述当前节点的布局方式，不创建或嵌套子节点。使用方向样式时会自动启用 Flex 布局，再与换行、主轴对齐和交叉轴对齐样式组合：

```rust
bsn! {
    bstyle!(flex-col items-center justify-center)

    Children [
        Text("内容")
    ]
}
```

#### 显示方式和方向

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `flex-row` | `display: Display::Flex`、`flex_direction: FlexDirection::Row` | 从左到右排列 |
| `flex-row-reverse` | `display: Display::Flex`、`flex_direction: FlexDirection::RowReverse` | 从右到左排列 |
| `flex-col` | `display: Display::Flex`、`flex_direction: FlexDirection::Column` | 从上到下排列 |
| `flex-col-reverse` | `display: Display::Flex`、`flex_direction: FlexDirection::ColumnReverse` | 从下到上排列 |
| `flex-center` | `display: Display::Flex`、`flex_direction: FlexDirection::Row`、`flex_wrap: FlexWrap::Wrap`、`justify_content: JustifyContent::Center`、`align_items: AlignItems::Center` | 以 row 方向排列并允许换行，内容水平、垂直居中 |

未指定方向时不主动启用 Flex 布局；使用 `flex-row` 或其它方向样式后，默认方向为对应样式指定的方向。

`flex-center` 是最常用的居中布局快捷写法，等价于：

```rust
bstyle!(flex-row flex-wrap justify-center items-center)
```

#### 换行

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `flex-nowrap` | `flex_wrap: FlexWrap::NoWrap` | 子节点不换行 |
| `flex-wrap` | `flex_wrap: FlexWrap::Wrap` | 子节点超出主轴空间时换行 |
| `flex-wrap-reverse` | `flex_wrap: FlexWrap::WrapReverse` | 反向换行 |

#### 主轴对齐

`justify-*` 控制子节点在 Flex 主轴上的分布，对应 Bevy 的 `justify_content`：

| 语法 | Bevy 属性值 |
| --- | --- |
| `justify-start` | `JustifyContent::FlexStart` |
| `justify-end` | `JustifyContent::FlexEnd` |
| `justify-center` | `JustifyContent::Center` |
| `justify-between` | `JustifyContent::SpaceBetween` |
| `justify-around` | `JustifyContent::SpaceAround` |
| `justify-evenly` | `JustifyContent::SpaceEvenly` |
| `justify-stretch` | `JustifyContent::Stretch` |

#### 交叉轴对齐

`items-*` 控制子节点在 Flex 交叉轴上的对齐方式，对应 Bevy 的 `align_items`：

| 语法 | Bevy 属性值 |
| --- | --- |
| `items-start` | `AlignItems::FlexStart` |
| `items-end` | `AlignItems::FlexEnd` |
| `items-center` | `AlignItems::Center` |
| `items-baseline` | `AlignItems::Baseline` |
| `items-stretch` | `AlignItems::Stretch` |

例如，下面的写法会让节点内部的子节点水平、垂直都居中：

```rust
bstyle!(flex-row items-center justify-center)
```

#### 多行内容对齐

当节点使用 `flex-wrap` 产生多行内容时，`content-*` 控制多行在交叉轴上的分布，对应 Bevy 的 `align_content`：

| 语法 | Bevy 属性值 |
| --- | --- |
| `content-start` | `AlignContent::FlexStart` |
| `content-end` | `AlignContent::FlexEnd` |
| `content-center` | `AlignContent::Center` |
| `content-between` | `AlignContent::SpaceBetween` |
| `content-around` | `AlignContent::SpaceAround` |
| `content-evenly` | `AlignContent::SpaceEvenly` |
| `content-stretch` | `AlignContent::Stretch` |

同一组中出现多个互相冲突的 Flex 样式时，后续实现应在编译期报错，而不是静默决定覆盖顺序。
