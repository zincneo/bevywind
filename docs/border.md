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

## 圆角

圆角样式属于 Bevy UI `Node` 的边框属性，设置 `Node.border_radius` 的四个角：`top_left`、`top_right`、`bottom_right` 和 `bottom_left`。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `b_r_{值}` | `Node.border_radius` 四个角 | 设置四个角的圆角；`{值}` 支持下方所有圆角值 |
| `bl_r_{值}` | `Node.border_radius.top_left`、`Node.border_radius.bottom_left` | 设置左侧两个角的圆角；`{值}` 支持下方所有圆角值 |
| `br_r_{值}` | `Node.border_radius.top_right`、`Node.border_radius.bottom_right` | 设置右侧两个角的圆角；`{值}` 支持下方所有圆角值 |
| `bt_r_{值}` | `Node.border_radius.top_left`、`Node.border_radius.top_right` | 设置顶部两个角的圆角；`{值}` 支持下方所有圆角值 |
| `bb_r_{值}` | `Node.border_radius.bottom_left`、`Node.border_radius.bottom_right` | 设置底部两个角的圆角；`{值}` 支持下方所有圆角值 |

以上五种前缀都支持以下完整的 `{值}` 格式。`b_r` 不带下划线后缀时等同于 `b_r_4px`：

| 写法 | Bevy 值 | 含义 |
| --- | --- | --- |
| `b_r_none`、`bl_r_none`、`br_r_none`、`bt_r_none`、`bb_r_none` | `CornerRadius::circular(px(0))` | 清除对应角的圆角 |
| `b_r_sm`、`bl_r_sm`、`br_r_sm`、`bt_r_sm`、`bb_r_sm` | `px(2)` | 小圆角 |
| `b_r` | `px(4)` | 默认圆角 |
| `b_r_md`、`bl_r_md`、`br_r_md`、`bt_r_md`、`bb_r_md` | `px(6)` | 中等圆角 |
| `b_r_lg`、`bl_r_lg`、`br_r_lg`、`bt_r_lg`、`bb_r_lg` | `px(8)` | 大圆角 |
| `b_r_xl`、`bl_r_xl`、`br_r_xl`、`bt_r_xl`、`bb_r_xl` | `px(12)` | 超大圆角 |
| `b_r_2xl`、`bl_r_2xl`、`br_r_2xl`、`bt_r_2xl`、`bb_r_2xl` | `px(16)` | `2xl` 圆角 |
| `b_r_3xl`、`bl_r_3xl`、`br_r_3xl`、`bt_r_3xl`、`bb_r_3xl` | `px(24)` | `3xl` 圆角 |
| `b_r_full`、`bl_r_full`、`br_r_full`、`bt_r_full`、`bb_r_full` | `CornerRadius::MAX` | 最大圆角，形成胶囊或圆形 |
| `b_r_{数字}px` / `bl_r_{数字}px` / `br_r_{数字}px` / `bt_r_{数字}px` / `bb_r_{数字}px` | `px({数字})` | 固定逻辑像素圆角 |
| `b_r_{数字}per` / `bl_r_{数字}per` / `br_r_{数字}per` / `bt_r_{数字}per` / `bb_r_{数字}per` | `percent({数字})` | 相对父节点对应尺寸的百分比圆角 |
| `b_r_{数字}w` / `bl_r_{数字}w` / `br_r_{数字}w` / `bt_r_{数字}w` / `bb_r_{数字}w` | `vw({数字})` | 相对视口宽度的百分比圆角 |
| `b_r_{数字}h` / `bl_r_{数字}h` / `br_r_{数字}h` / `bt_r_{数字}h` / `bb_r_{数字}h` | `vh({数字})` | 相对视口高度的百分比圆角 |

数字按非负 `u16` 解析。全方向和组合方向样式会展开为具体角，再参与重复属性检查：

```rust
bstyle!(b_r_lg)
bstyle!(bl_r_12px br_r_12px)
```

`b_r_lg bl_r_12px` 会报告重复属性错误；`bl_r_12px br_r_12px` 可以组合使用。未被指定的角保持原有值。
