# 定位

定位样式作用于当前实体的 Bevy UI `Node`，设置 `Node.position_type` 以及 `Node.left`、`Node.right`、`Node.top`、`Node.bottom`。本章描述当前 Bevy UI 能直接表达的相对定位和绝对定位；尚未实现或无法直接映射的 Tailwind CSS 值格式列在文末。

## 先理解 Bevy 的原生规则

Bevy UI 的定位模型与 Web CSS 的核心概念相似，但不是完整的 CSS 定位实现。Bevy 当前只提供 `PositionType::Relative` 和 `PositionType::Absolute`：

- `Relative` 是默认模式。节点先按照父节点的 Flex/Grid 等正常布局参与流式排布，再以这个正常布局位置作为偏移基准。它不会脱离布局，也不会因为使用相对定位而成为绝对定位子节点的特殊参考容器。
- `Absolute` 会脱离同级节点的正常布局，不再占用正常流中的位置，并相对于父节点的边界框定位。它仍然属于当前父节点的 UI 层级，不能脱离父节点独立存在。
- `top`、`right`、`bottom`、`left` 是位置偏移字段，不是四个独立的坐标系统。相对定位时它们相对正常布局结果生效；绝对定位时相对父节点边界生效。

因此，Bevy 的 `relative` 可以理解为“保留正常布局并允许偏移”，而不是简单理解为“相对于父节点定位”。Bevy 的 `absolute` 与 Web 中最常用的绝对定位行为接近，但具体尺寸计算、Flex/Grid 交互和同时设置相对方向的行为仍由 Bevy UI 布局算法决定，不能假定所有 CSS 边界情况都一致。

Web CSS 中，绝对定位元素会向上查找最近的已定位祖先；对 Bevy UI 来说不是这样。Bevy 的绝对定位节点始终直接相对于父节点定位，不会跳过普通父节点去寻找最近的 `Relative` 祖先。父节点是否设置 `PositionType::Relative` 不会改变这个参考关系。

Tailwind CSS 中常见的 `relative` 父元素和 `absolute` 子元素组合，是为了建立 CSS 定位上下文。对 Bevy UI 来说，绝对节点原生就是相对其直接父节点定位；父节点使用 `relative` 主要表示它仍参与正常布局，并让代码意图与 Tailwind 写法保持一致，而不是为了补充 Bevy 缺失的包含块规则。

## 定位模式

定位模式沿用 Tailwind CSS 的 `relative` 和 `absolute` 命名：

| 语法 | Component | 字段 | 值 | 含义 |
| --- | --- | --- | --- | --- |
| `relative` | `Node` | `position_type` | `PositionType::Relative` | 节点参与正常布局，并以正常布局结果为偏移基准 |
| `absolute` | `Node` | `position_type` | `PositionType::Absolute` | 节点脱离兄弟节点布局，并相对于父节点定位 |

`Node.position_type` 的默认值是 `PositionType::Relative`。显式使用 `relative` 可以表达“参与正常布局但允许偏移”的意图；`absolute` 节点的定位参考对象由 Bevy 的父节点关系决定。

```rust
bsn! {
    bstyle!(relative w_320px h_200px)
    Children [
        bstyle!(absolute top_10px left_10px w_100px h_50px)
    ]
}
```

## 四方向偏移

四方向偏移直接对应 Bevy `Node` 的物理方向字段：

| 语法 | Component | 字段 | 含义 |
| --- | --- | --- | --- |
| `top_{值}` | `Node` | `top` | 设置顶部偏移 |
| `right_{值}` | `Node` | `right` | 设置右侧偏移 |
| `bottom_{值}` | `Node` | `bottom` | 设置底部偏移 |
| `left_{值}` | `Node` | `left` | 设置左侧偏移 |

偏移值使用现有尺寸值的单位：

| 写法 | Bevy 值 | 含义 |
| --- | --- | --- |
| `top_10px`、`right_10px`、`bottom_10px`、`left_10px` | `px(10)` | 固定逻辑像素偏移 |
| `top_20per`、`right_20per`、`bottom_20per`、`left_20per` | `percent(20)` | 相对父节点对应尺寸的百分比偏移 |
| `top_30w`、`right_30w`、`bottom_30w`、`left_30w` | `vw(30)` | 相对视口宽度的百分比偏移 |
| `top_40h`、`right_40h`、`bottom_40h`、`left_40h` | `vh(40)` | 相对视口高度的百分比偏移 |
| `top_full`、`right_full`、`bottom_full`、`left_full` | `percent(100)` | 100% 偏移 |

数字按现有规则解析为非负 `u16`。偏移样式可以与 `relative` 或 `absolute` 同时使用；如果没有指定定位模式，Bevy 的默认相对定位仍然生效。

### 负值偏移

Bevy 的 `Val::Px(f32)` 和 `Val::Percent(f32)` 可以表达负值。定位样式设计使用 `n_` 标记负数，放在值的单位前缀之前：

| 写法 | Bevy 值 | 含义 |
| --- | --- | --- |
| `top_n_10px`、`right_n_10px`、`bottom_n_10px`、`left_n_10px` | `px(-10)` | 向对应方向的反方向偏移 10 逻辑像素 |
| `top_n_20per`、`right_n_20per`、`bottom_n_20per`、`left_n_20per` | `percent(-20)` | 使用 -20% 的对应方向偏移 |

同样的 `n_` 规则适用于 `px`、`per`、`w`、`h` 和 `full`，例如 `left_n_30w` 和 `top_n_40h`。这里的 `n_` 是 bevywind 的 token 约定，最终分别生成负的 `Val::Px`、`Val::Percent`、`Val::Vw`、`Val::Vh` 或 `Val::Percent` 值。该语法已加入定位样式实现。

## 暂不支持的设计

以下能力保留在设计范围内，但当前不能直接由现有实现表达：

- `auto` 偏移，例如 `top-auto`。Bevy 支持 `Val::Auto`，但当前尺寸值解析器没有 `auto` 值。
- Tailwind 的任意值和 CSS 变量，例如 `top-[3px]`。当前样式 token 只支持固定格式，运行时也不执行 CSS 表达式。

这些能力暂不加入可用 token，待值系统和定位模型明确后再调整。
