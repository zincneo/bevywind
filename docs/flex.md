# Flex 布局

Flex 样式作用于当前实体的 Bevy UI `Node` 组件。指定任意 Flex 样式后，效果等价于给实体添加 `Node`（如果尚未添加），并设置对应字段。

## 方向

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `flex_row` | `Node` | `display`, `flex_direction` | `Display::Flex`, `FlexDirection::Row` |
| `flex_row_reverse` | `Node` | `display`, `flex_direction` | `Display::Flex`, `FlexDirection::RowReverse` |
| `flex_col` | `Node` | `display`, `flex_direction` | `Display::Flex`, `FlexDirection::Column` |
| `flex_col_reverse` | `Node` | `display`, `flex_direction` | `Display::Flex`, `FlexDirection::ColumnReverse` |

方向样式会同时设置 `display: Display::Flex`，使当前节点成为 Flex 容器。

## 换行

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `flex_nowrap` | `Node` | `flex_wrap` | `FlexWrap::NoWrap` |
| `flex_wrap` | `Node` | `flex_wrap` | `FlexWrap::Wrap` |
| `flex_wrap_reverse` | `Node` | `flex_wrap` | `FlexWrap::WrapReverse` |

## 主轴对齐

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `justify_start` | `Node` | `justify_content` | `JustifyContent::Start` |
| `justify_end` | `Node` | `justify_content` | `JustifyContent::End` |
| `justify_center` | `Node` | `justify_content` | `JustifyContent::Center` |
| `justify_between` | `Node` | `justify_content` | `JustifyContent::SpaceBetween` |
| `justify_around` | `Node` | `justify_content` | `JustifyContent::SpaceAround` |
| `justify_evenly` | `Node` | `justify_content` | `JustifyContent::SpaceEvenly` |
| `justify_stretch` | `Node` | `justify_content` | `JustifyContent::Stretch` |

## 交叉轴对齐

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `items_start` | `Node` | `align_items` | `AlignItems::Start` |
| `items_end` | `Node` | `align_items` | `AlignItems::End` |
| `items_center` | `Node` | `align_items` | `AlignItems::Center` |
| `items_baseline` | `Node` | `align_items` | `AlignItems::Baseline` |
| `items_stretch` | `Node` | `align_items` | `AlignItems::Stretch` |

## 多行内容对齐

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `content_start` | `Node` | `align_content` | `AlignContent::Start` |
| `content_end` | `Node` | `align_content` | `AlignContent::End` |
| `content_center` | `Node` | `align_content` | `AlignContent::Center` |
| `content_between` | `Node` | `align_content` | `AlignContent::SpaceBetween` |
| `content_around` | `Node` | `align_content` | `AlignContent::SpaceAround` |
| `content_evenly` | `Node` | `align_content` | `AlignContent::SpaceEvenly` |
| `content_stretch` | `Node` | `align_content` | `AlignContent::Stretch` |

## 组合样式

`flex_center` 是组合样式，等价于对同一实体的 `Node` 设置以下字段：

| 字段 | 值 |
| --- | --- |
| `display` | `Display::Flex` |
| `flex_direction` | `FlexDirection::Row` |
| `justify_content` | `JustifyContent::Center` |
| `align_items` | `AlignItems::Center` |

```rust
bstyle!(flex_center)
bstyle!(flex_center flex_wrap)
bstyle!(flex_col items_start justify_between)
```
