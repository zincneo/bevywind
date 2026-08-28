# bevywind

`bevywind` 是一个面向 Bevy UI 的 Tailwind CSS 风格样式库。

它只负责为当前 UI 实体生成样式 patch，不负责创建子节点或描述 UI 层级。UI 结构仍然使用 Bevy 的 `bsn!` 编写。

## 使用方式

```rust
use bevy::prelude::*;
use bevywind::bstyle;

fn scene() -> impl Scene {
    bsn! {
        bstyle!(w_full h_full)

        Children [
            Text("Hello")
        ]
    }
}
```

`bstyle!` 是过程宏。样式 token 会在编译期解析，解析失败时会产生编译期错误：

```rust
bstyle!(h_10px w_50per)
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

## 可用样式

### 高度和宽度

当前支持 `h_*` 高度和 `w_*` 宽度样式：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `h_full` | `height: percent(100)` | 高度占父节点 100% |
| `w_full` | `width: percent(100)` | 宽度占父节点 100% |
| `h_10px` | `height: px(10)` | 高度为 10 逻辑像素 |
| `w_20px` | `width: px(20)` | 宽度为 20 逻辑像素 |
| `h_30per` | `height: percent(30)` | 高度为父节点的 30% |
| `w_40per` | `width: percent(40)` | 宽度为父节点的 40% |
| `h_50w` | `height: vw(50)` | 高度为视口宽度的 50% |
| `w_60w` | `width: vw(60)` | 宽度为视口宽度的 60% |
| `h_70h` | `height: vh(70)` | 高度为视口高度的 70% |
| `w_80h` | `width: vh(80)` | 宽度为视口高度的 80% |

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。

最小和最大尺寸使用与高度、宽度相同的格式：

```rust
bstyle!(min_h_100px min_w_20per max_h_80w max_w_90h)
```

它们分别对应 Bevy `Node` 的 `min_height`、`min_width`、`max_height` 和 `max_width` 属性。

### 背景色

使用 `bg_*` 设置当前 UI 节点的 `BackgroundColor`，语法参考 Tailwind CSS：

| 写法 | 解析结果 |
| --- | --- |
| `bg_transparent` | `BackgroundColor(Color::NONE)` |
| `bg_black` | `BackgroundColor(Color::BLACK)` |
| `bg_white` | `BackgroundColor(Color::WHITE)` |
| `bg_{颜色}` | 使用该颜色的 `500` 色阶 |
| `bg_{颜色}_{色阶}` | 使用指定色阶，色阶范围为 `50–950` |
| `bg_rrggbb` | 解析为不透明 `Color::srgb(...)` |
| `bg_rrggbbaa` | 解析为带透明度的 `Color::srgba(...)` |

预设颜色为：

| 颜色 | 可用语法 |
| --- | --- |
| `slate`、`gray`、`zinc`、`neutral`、`stone` | `bg_{颜色}`、`bg_{颜色}_{色阶}` |
| `red`、`orange`、`amber`、`yellow`、`lime` | `bg_{颜色}`、`bg_{颜色}_{色阶}` |
| `green`、`emerald`、`teal`、`cyan`、`sky` | `bg_{颜色}`、`bg_{颜色}_{色阶}` |
| `blue`、`indigo`、`violet`、`purple`、`fuchsia` | `bg_{颜色}`、`bg_{颜色}_{色阶}` |
| `pink`、`rose` | `bg_{颜色}`、`bg_{颜色}_{色阶}` |

```rust
bstyle!(bg_red)
bstyle!(bg_red_50)
bstyle!(bg_ffffff)
bstyle!(bg_ffffff80)
```

同一个节点只能指定一个背景色。

### Flex 布局

#### 方向

| 语法 | Bevy 属性 |
| --- | --- |
| `flex_row` | `display: Display::Flex`, `flex_direction: FlexDirection::Row` |
| `flex_row_reverse` | `display: Display::Flex`, `flex_direction: FlexDirection::RowReverse` |
| `flex_col` | `display: Display::Flex`, `flex_direction: FlexDirection::Column` |
| `flex_col_reverse` | `display: Display::Flex`, `flex_direction: FlexDirection::ColumnReverse` |
| `flex_center` | `flex_row justify_center items_center` |

方向样式自动设置 `display: Display::Flex`。

#### 换行

| 语法 | Bevy 属性 |
| --- | --- |
| `flex_nowrap` | `flex_wrap: FlexWrap::NoWrap` |
| `flex_wrap` | `flex_wrap: FlexWrap::Wrap` |
| `flex_wrap_reverse` | `flex_wrap: FlexWrap::WrapReverse` |

#### 对齐

| 语法 | Bevy 属性 |
| --- | --- |
| `justify_start` / `justify_end` | `justify_content: JustifyContent::FlexStart` / `FlexEnd` |
| `justify_center` | `justify_content: JustifyContent::Center` |
| `justify_between` / `justify_around` | `justify_content: JustifyContent::SpaceBetween` / `SpaceAround` |
| `justify_evenly` / `justify_stretch` | `justify_content: JustifyContent::SpaceEvenly` / `Stretch` |
| `items_start` / `items_end` | `align_items: AlignItems::FlexStart` / `FlexEnd` |
| `items_center` / `items_baseline` | `align_items: AlignItems::Center` / `Baseline` |
| `items_stretch` | `align_items: AlignItems::Stretch` |
| `content_start` / `content_end` | `align_content: AlignContent::FlexStart` / `FlexEnd` |
| `content_center` | `align_content: AlignContent::Center` |
| `content_between` / `content_around` | `align_content: AlignContent::SpaceBetween` / `SpaceAround` |
| `content_evenly` / `content_stretch` | `align_content: AlignContent::SpaceEvenly` / `Stretch` |

```rust
bstyle!(flex_center)
bstyle!(flex_center flex_wrap)
bstyle!(flex_col items_start justify_between)
```
