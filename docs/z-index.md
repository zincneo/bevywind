# Z-index

层级样式对应当前 UI Entity 的 Bevy `ZIndex` Component 或 `GlobalZIndex` Component，并分别设置对应 Component 的元组字段 `0`。它们不是 `Node` 的字段，也不应合并到 `Node { ... }` 中：实现时应在基础 `Node` 之外额外生成对应 Component。`ZIndex` 控制同一 UI 层级中节点的前后顺序；`GlobalZIndex` 可以让节点脱离 UI 层级树的隐含排序，与其它 UI 层级进行全局排序。该样式只影响绘制顺序，不改变 `Node` 的布局位置、尺寸或父子层级。

## Bevy 与 Web 的层级规则

Bevy 的 `ZIndex` 与 Web CSS 的 `z-index` 作用相近，但不是完整的 CSS stacking context 实现：

- 没有 `ZIndex` 的节点按 `ZIndex(0)` 处理。
- 同一父节点下，`ZIndex` 较大的兄弟节点显示在前面；值相同时，UI 层级中后加入的节点显示在前面。
- `ZIndex` 主要用于同一 UI 层级内的兄弟节点，不能像 Web 中的已定位祖先那样建立通用 stacking context。
- `GlobalZIndex` 用于跨 UI 层级排序。`GlobalZIndex` 较大的节点显示在前面；当两个节点的 `GlobalZIndex` 相同时，再比较它们的 `ZIndex`。
- `GlobalZIndex` 为负数时可以将节点放到没有该 Component 或值更低的节点之后。

因此，Tailwind CSS 中的 `z-10` 不应被理解为“在整个界面中绝对位于所有内容之上”。在 bevywind 中，普通 `z_*` 只解决同一 UI 层级内的排序；需要跨层级覆盖时必须明确使用全局层级样式。

## 局部层级

局部层级样式额外生成 `ZIndex` Component，并设置其元组字段 `ZIndex.0`：

| 语法 | Bevy Component / 字段 | 含义 |
| --- | --- | --- |
| `z_{数值}` | `ZIndex` Component 的字段 `0` | 设置当前节点在所属 UI 层级中的绘制层级 |
| `z_n_{数值}` | `ZIndex` Component 的字段 `0` | 设置当前节点的负层级 |

数值使用无单位的非负整数。`z_n_10` 表示 `ZIndex(-10)`，`z_0` 表示 `ZIndex(0)`。`z_10 z_n_10` 会因重复设置同一个 `ZIndex.0` 而报错；层级样式可以与尺寸、定位、Flex 和其它 Component 样式组合。

```rust
bsn! {
    bstyle!(relative z_10)
    Children [
        bstyle!(absolute z_20)
    ]
}
```

上例中，两个节点仍然按照 Bevy 的父子关系参与 UI 层级；`z_20` 只表达节点在所属层级中的前后顺序，不会改变绝对定位的参考对象。

## 全局层级

为了明确区别 Bevy 的跨层级能力，`gz_*` 额外生成 `GlobalZIndex` Component，并设置其元组字段 `GlobalZIndex.0`：

| 语法 | Bevy Component / 字段 | 含义 |
| --- | --- | --- |
| `gz_{数值}` | `GlobalZIndex` Component 的字段 `0` | 设置当前节点在不同 UI 层级之间的全局绘制层级 |
| `gz_n_{数值}` | `GlobalZIndex` Component 的字段 `0` | 设置当前节点的负全局层级 |

`gz_*` 与 `z_*` 是两个不同的 Component，允许同时使用。比较顺序是先比较 `GlobalZIndex`，只有全局值相同时才比较 `ZIndex`。例如：

```rust
bstyle!(gz_100 z_10)
```

表示先将节点放入全局层级 `100`，再在该全局层级内使用局部层级 `10` 排序。

## Tailwind CSS 对齐边界

Tailwind 的常用层级值可以直接用无单位整数表达：`z_0`、`z_10`、`z_20`、`z_30`、`z_40` 和 `z_50`。项目使用下划线 token，并通过 `n_` 表示负数，因此不使用 CSS 的连字符写法。

以下能力暂不设计为 token：

| Tailwind/CSS 能力 | 原因 |
| --- | --- |
| `z_auto` | Bevy 的 `ZIndex` 和 `GlobalZIndex` 都是整数 Component，没有 CSS `auto` 的等价值 |
| `z-[...]`、CSS 变量和任意表达式 | 当前样式值系统只接受明确的静态整数 |
| `isolation`、stacking context 控制 | Bevy UI 没有对应的 CSS stacking context Component |
| `transform` 对层级上下文的影响 | Bevy 的 `UiTransform` 是视觉变换，不建立 CSS stacking context |

## `bstyle!` 与 `bstyle_r`

`bstyle!` 在编译期保留基础 `Node`，并在其外额外生成 `ZIndex` 或 `GlobalZIndex` Component；`bstyle_r` 在运行时解析同样的 token，并生成相同的独立 Component 和整数值。两者只负责当前时刻的层级快照，不自动根据 hover、pressed、focused 等状态切换层级，也不负责创建 UI 层级。

状态变化应由 ECS 系统根据交互状态选择不同的 `bstyle!` Scene patch，或直接修改 `ZIndex` / `GlobalZIndex`。`bsn!` 的 `on(...)` 可以触发这类状态更新，但交互判断和动画过程不属于 z-index 样式解析本身。

Bevy 原生说明可参考 [ZIndex](https://docs.rs/bevy/latest/bevy/ui/struct.ZIndex.html) 和 [GlobalZIndex](https://docs.rs/bevy/latest/bevy/ui/struct.GlobalZIndex.html)；Tailwind 的对应能力可参考 [z-index](https://tailwindcss.com/docs/z-index)。
