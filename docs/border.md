# 边框

这些样式对应 Bevy UI 的两个 Component：边框宽度样式给当前 Entity 添加 `bevy::ui::Node`，并设置 `Node.border`（`UiRect.left`、`UiRect.right`、`UiRect.top`、`UiRect.bottom`）字段；边框颜色样式给当前 Entity 添加 `bevy::ui::BorderColor`，并设置其 `left`、`right`、`top`、`bottom` 字段。

## 边框宽度

边框宽度使用与高度、宽度相同的值格式：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `b_{值}` | `Node.border.left`、`Node.border.right`、`Node.border.top`、`Node.border.bottom` | 设置四个方向的边框宽度 |
| `bl_{值}` | `Node.border.left` | 设置左边框宽度 |
| `br_{值}` | `Node.border.right` | 设置右边框宽度 |
| `bt_{值}` | `Node.border.top` | 设置上边框宽度 |
| `bb_{值}` | `Node.border.bottom` | 设置下边框宽度 |

其中 `{值}` 支持以下格式：

| 写法 | Bevy 值 | 含义 |
| --- | --- | --- |
| `{数字}px` | `px({数字})` | 固定逻辑像素 |
| `{数字}per` | `percent({数字})` | 父节点对应尺寸的百分比 |
| `{数字}w` | `vw({数字})` | 视口宽度的百分比 |
| `{数字}h` | `vh({数字})` | 视口高度的百分比 |

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。

## 边框颜色

边框颜色使用与背景色相同的颜色格式，并设置 `BorderColor` 的对应字段：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `b_{颜色}` | `BorderColor.left`、`BorderColor.right`、`BorderColor.top`、`BorderColor.bottom` | 使用该颜色的 `500` 色阶 |
| `b_{颜色}_{色阶}` | `BorderColor.left`、`BorderColor.right`、`BorderColor.top`、`BorderColor.bottom` | 使用指定色阶，色阶范围为 `50–950` |
| `b_rrggbb` | `BorderColor.left`、`BorderColor.right`、`BorderColor.top`、`BorderColor.bottom` | 设置四个方向的不透明边框颜色 |
| `b_rrggbbaa` | `BorderColor.left`、`BorderColor.right`、`BorderColor.top`、`BorderColor.bottom` | 设置四个方向的边框颜色和透明度 |
| `bl_{颜色}` / `bl_{颜色}_{色阶}` | `BorderColor.left` | 设置左边框颜色 |
| `br_{颜色}` / `br_{颜色}_{色阶}` | `BorderColor.right` | 设置右边框颜色 |
| `bt_{颜色}` / `bt_{颜色}_{色阶}` | `BorderColor.top` | 设置上边框颜色 |
| `bb_{颜色}` / `bb_{颜色}_{色阶}` | `BorderColor.bottom` | 设置下边框颜色 |
| `bl_rrggbb` / `bl_rrggbbaa` | `BorderColor.left` | 设置左边框颜色 |
| `br_rrggbb` / `br_rrggbbaa` | `BorderColor.right` | 设置右边框颜色 |
| `bt_rrggbb` / `bt_rrggbbaa` | `BorderColor.top` | 设置上边框颜色 |
| `bb_rrggbb` / `bb_rrggbbaa` | `BorderColor.bottom` | 设置下边框颜色 |

预设颜色为：

| 颜色 | 可用语法 |
| --- | --- |
| `slate`、`gray`、`zinc`、`neutral`、`stone` | `b_{颜色}`、`b_{颜色}_{色阶}`，以及 `bl_`、`br_`、`bt_`、`bb_` 方向变体 |
| `red`、`orange`、`amber`、`yellow`、`lime` | `b_{颜色}`、`b_{颜色}_{色阶}`，以及方向变体 |
| `green`、`emerald`、`teal`、`cyan`、`sky` | `b_{颜色}`、`b_{颜色}_{色阶}`，以及方向变体 |
| `blue`、`indigo`、`violet`、`purple`、`fuchsia` | `b_{颜色}`、`b_{颜色}_{色阶}`，以及方向变体 |
| `pink`、`rose` | `b_{颜色}`、`b_{颜色}_{色阶}`，以及方向变体 |

例如：

```rust
bstyle!(b_1px b_ffffff)
bstyle!(bt_2px bb_4px bl_red_500)
bstyle!(br_blue_300 bb_33669980)
```

多个方向样式可以组合使用，未被样式指定的字段保持不变。
