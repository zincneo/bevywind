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

## Flex 子项增长与收缩

以下样式作用于当前 Flex 子项的 `Node`，控制它在 Flex 容器中如何分配可用空间：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `grow` | `Node.flex_grow` | 设置增长因子为 `1` |
| `grow_{数字}` | `Node.flex_grow` | 设置指定的非负增长因子 |
| `shrink` | `Node.flex_shrink` | 设置收缩因子为 `1` |
| `shrink_{数字}` | `Node.flex_shrink` | 设置指定的非负收缩因子 |

增长因子和收缩因子是无单位的非负整数，最终转换为 Bevy 所需的 `f32`。`grow_0` 和 `shrink_0` 分别表示不增长和不收缩。

## Flex 基础尺寸

`basis_*` 设置 Flex 子项参与增长或收缩计算前的基础尺寸，对应 `Node.flex_basis`：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `basis_auto` | `Node.flex_basis` | 使用 `Val::Auto`，由内容和布局规则决定基础尺寸 |
| `basis_0` | `Node.flex_basis` | 使用 `px(0)` 作为基础尺寸 |
| `basis_full` | `Node.flex_basis` | 使用 `percent(100)` 作为基础尺寸 |
| `basis_{数字}px` | `Node.flex_basis` | 使用固定逻辑像素基础尺寸 |
| `basis_{数字}per` | `Node.flex_basis` | 使用父节点对应尺寸的百分比基础尺寸 |
| `basis_{数字}w` | `Node.flex_basis` | 使用视口宽度百分比基础尺寸 |
| `basis_{数字}h` | `Node.flex_basis` | 使用视口高度百分比基础尺寸 |

除 `basis_auto` 外，`basis_*` 的单位值沿用尺寸样式的值格式。`flex_basis` 在水平方向通常影响宽度，在垂直方向通常影响高度，并遵循 Bevy 对最小和最大尺寸的约束。

## Flex 子项交叉轴对齐

`self_*` 只修改当前子项的交叉轴对齐方式，不修改父容器的 `align_items`：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `self_auto` | `Node.align_self` | 继承父节点的 `align_items` |
| `self_start` | `Node.align_self` | 在交叉轴起点对齐 |
| `self_end` | `Node.align_self` | 在交叉轴终点对齐 |
| `self_center` | `Node.align_self` | 在交叉轴居中对齐 |
| `self_baseline` | `Node.align_self` | 按基线对齐 |
| `self_stretch` | `Node.align_self` | 拉伸填充交叉轴空间 |

## Flex 与 Grid 间距

间距样式设置当前容器子项之间的间隔。`x`、`y` 按物理方向命名：横向对应 `column_gap`，纵向对应 `row_gap`。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `gap_{值}` | `Node.row_gap`、`Node.column_gap` | 同时设置横向和纵向间距 |
| `gap_x_{值}` | `Node.column_gap` | 设置横向间距 |
| `gap_y_{值}` | `Node.row_gap` | 设置纵向间距 |

`{值}` 支持 `0`、`full`、`{数字}px`、`{数字}per`、`{数字}w` 和 `{数字}h`。其中 `0` 等价于 `px(0)`。百分比和视口单位的具体计算方式由 Bevy 的 `Val` 和布局算法决定。

全方向 `gap_{值}` 会先展开为 `row_gap` 和 `column_gap`，再执行重复属性检查：

```rust
bstyle!(flex_col gap_10px)
bstyle!(gap_x_8px gap_y_12px)
```

`gap_10px gap_x_20px` 会报告 `Node.column_gap` 重复；`gap_x_8px gap_y_12px` 可以组合使用。

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
