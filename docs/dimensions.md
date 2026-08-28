# 高度和宽度

尺寸样式作用于当前实体的 Bevy UI `Node` 组件。指定任意尺寸样式后，效果等价于给实体添加 `Node`（如果尚未添加），并设置下表中的字段。

## 尺寸

| 语法 | Component | 字段 | 值 | 含义 |
| --- | --- | --- | --- | --- |
| `h_full` | `Node` | `height` | `percent(100)` | 高度占父节点 100% |
| `w_full` | `Node` | `width` | `percent(100)` | 宽度占父节点 100% |
| `h_10px` | `Node` | `height` | `px(10)` | 高度为 10 逻辑像素 |
| `w_20px` | `Node` | `width` | `px(20)` | 宽度为 20 逻辑像素 |
| `h_30per` | `Node` | `height` | `percent(30)` | 高度为父节点的 30% |
| `w_40per` | `Node` | `width` | `percent(40)` | 宽度为父节点的 40% |
| `h_50w` | `Node` | `height` | `vw(50)` | 高度为视口宽度的 50% |
| `w_60w` | `Node` | `width` | `vw(60)` | 宽度为视口宽度的 60% |
| `h_70h` | `Node` | `height` | `vh(70)` | 高度为视口高度的 70% |
| `w_80h` | `Node` | `width` | `vh(80)` | 宽度为视口高度的 80% |

其中，`px`、`percent`、`vw` 和 `vh` 分别对应 Bevy 的 `px(...)`、`percent(...)`、`vw(...)` 和 `vh(...)`。

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。

## 最小和最大尺寸

最小和最大尺寸使用与高度、宽度相同的值格式，并设置 `Node` 的对应字段：

| 语法 | Component | 字段 | 值 |
| --- | --- | --- | --- |
| `min_h_10px` | `Node` | `min_height` | `px(10)` |
| `min_w_20per` | `Node` | `min_width` | `percent(20)` |
| `max_h_30w` | `Node` | `max_height` | `vw(30)` |
| `max_w_40h` | `Node` | `max_width` | `vh(40)` |

例如：

```rust
bstyle!(min_h_100px min_w_20per max_h_80w max_w_90h)
```

等价于对同一实体的 `Node` 设置 `min_height`、`min_width`、`max_height` 和 `max_width` 四个字段。
