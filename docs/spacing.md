# 外边距和内边距

这些样式等价于给当前 Entity 添加 Bevy UI 的 `bevy::ui::Node` Component，并设置对应字段：`m_*`、`ml_*`、`mr_*`、`mt_*`、`mb_*` 设置 `Node.margin`（`UiRect.left`、`UiRect.right`、`UiRect.top`、`UiRect.bottom`）字段；`p_*`、`pl_*`、`pr_*`、`pt_*`、`pb_*` 设置 `Node.padding`（`UiRect.left`、`UiRect.right`、`UiRect.top`、`UiRect.bottom`）字段。

当前支持上述外边距和内边距样式的设计。样式名称参考 Tailwind CSS 的方向前缀，但值格式沿用本项目的尺寸样式，并使用 `_` 连接前缀和值。

## 值格式

外边距和内边距使用与高度、宽度相同的值格式：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `{前缀}_full` | `percent(100)` | 父节点对应尺寸的 100% |
| `{数字}px` | `px({数字})` | 固定逻辑像素 |
| `{数字}per` | `percent({数字})` | 父节点对应尺寸的百分比 |
| `{数字}w` | `vw({数字})` | 视口宽度的百分比 |
| `{数字}h` | `vh({数字})` | 视口高度的百分比 |

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。

## 外边距

外边距样式设置 `Node.margin` 对应字段：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `m_{值}` | `Node.margin.left`、`Node.margin.right`、`Node.margin.top`、`Node.margin.bottom` | 设置四个方向的外边距 |
| `ml_{值}` | `Node.margin.left` | 设置左外边距 |
| `mr_{值}` | `Node.margin.right` | 设置右外边距 |
| `mt_{值}` | `Node.margin.top` | 设置上外边距 |
| `mb_{值}` | `Node.margin.bottom` | 设置下外边距 |

例如：

```rust
Node {
    margin: UiRect {
        left: px(10),
        ..Default::default()
    },
    ..Default::default()
}
```

多个方向样式可以组合使用，未被样式指定的 `Node.margin` 字段保持不变。

## 内边距

内边距样式设置 `Node.padding` 对应字段：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `p_{值}` | `Node.padding.left`、`Node.padding.right`、`Node.padding.top`、`Node.padding.bottom` | 设置四个方向的内边距 |
| `pl_{值}` | `Node.padding.left` | 设置左内边距 |
| `pr_{值}` | `Node.padding.right` | 设置右内边距 |
| `pt_{值}` | `Node.padding.top` | 设置上内边距 |
| `pb_{值}` | `Node.padding.bottom` | 设置下内边距 |

例如：

```rust
bstyle!(p_10px)
bstyle!(pl_20per pr_30w)
bstyle!(mt_10px mb_20px pt_6h)
```

多个方向样式可以组合使用，未被样式指定的 `Node.padding` 字段保持不变。

## 组合规则

- 同一个字段只能指定一次。例如 `ml_10px ml_20px` 应视为重复的 `margin.left` 属性并报错。
- 可以同时指定互不冲突的字段。例如 `mt_10px mb_20px` 同时设置 `margin.top` 和 `margin.bottom`。
- `m_{值}` 会占用全部四个 `margin` 字段；`p_{值}` 会占用全部四个 `padding` 字段。
