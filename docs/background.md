# 背景色

背景色样式作用于当前实体的 Bevy UI `BackgroundColor` 组件。指定背景色样式后，效果等价于给实体添加 `BackgroundColor`（如果尚未添加），并设置其 `0` 字段：

```rust
BackgroundColor(Color::srgba(red, green, blue, alpha))
```

其中颜色通道会从 `0–255` 整数换算为 `0.0–1.0` 浮点数。

## 语法

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `bg_transparent` | `BackgroundColor` | `0` | `Color::srgba(0.0, 0.0, 0.0, 0.0)` |
| `bg_black` | `BackgroundColor` | `0` | `Color::srgba(0.0, 0.0, 0.0, 1.0)` |
| `bg_white` | `BackgroundColor` | `0` | `Color::srgba(1.0, 1.0, 1.0, 1.0)` |
| `bg_{颜色}` | `BackgroundColor` | `0` | 该颜色的 `500` 色阶 |
| `bg_{颜色}_{色阶}` | `BackgroundColor` | `0` | 指定色阶，范围为 `50–950` |
| `bg_rrggbb` | `BackgroundColor` | `0` | `Color::srgba(r, g, b, 1.0)` |
| `bg_rrggbbaa` | `BackgroundColor` | `0` | `Color::srgba(r, g, b, a)` |

十六进制颜色中的 `rr`、`gg`、`bb` 和 `aa` 分别表示红、绿、蓝和透明度通道；每个通道都会除以 `255.0` 后传给 `Color::srgba`。

## 预设颜色

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

同一个实体只能指定一个背景色样式。
