# UI 变换

UI 变换样式对应当前 UI Entity 的 Bevy `UiTransform` Component，设置 `translation`、`scale` 和 `rotation`。它只改变节点最终的视觉变换，不参与 `Node` 的布局计算，也不创建子节点。`bstyle!` 在编译期生成 `UiTransform`，`bstyle_r` 在运行时修改相同的字段；未指定的字段分别保持 `translation` 为零、`scale` 为 `1.0`、`rotation` 为单位旋转。

`UiTransform` 被添加到实体时，Bevy 会按原生组件要求维护对应的 `UiGlobalTransform`。bevywind 只负责生成 `UiTransform` 的字段补丁，不直接生成或管理 `UiGlobalTransform`。

## 与布局定位的区别

`left`、`right`、`top`、`bottom` 属于 `Node` 的布局定位：会参与相对或绝对定位计算；`UiTransform.translation` 属于视觉变换：在布局结果确定后移动渲染结果，不改变其它节点的布局位置。

```rust
bstyle!(absolute top_10px left_10px)
```

上例改变布局定位；而下面的样式只改变视觉位置：

```rust
bstyle!(tr_x_10px tr_y_n_5px)
```

## 平移

平移样式设置 `UiTransform.translation`。数值复用尺寸单位：`full`、`px`、`per`、`w`、`h`；负值使用统一的 `n_` 形式。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `tr_x_{数值}` | `UiTransform.translation.x` | 沿 X 轴视觉平移 |
| `tr_y_{数值}` | `UiTransform.translation.y` | 沿 Y 轴视觉平移 |
| `tr_{数值}` | `UiTransform.translation.x/y` | 同时沿 X、Y 轴使用相同平移值 |
| `tr_x_n_{数值}` | `UiTransform.translation.x` | 沿 X 轴反向视觉平移 |
| `tr_y_n_{数值}` | `UiTransform.translation.y` | 沿 Y 轴反向视觉平移 |

`tr_{数值}` 是组合样式，应展开为 X、Y 两个实际属性后参与重复检查。因此 `tr_10px tr_x_20px` 冲突，`tr_x_10px tr_y_20px` 可以组合。

## 缩放

缩放样式设置 `UiTransform.scale`。缩放值只支持 `per` 百分比单位：`100per` 表示 `1.0`，`150per` 表示 `1.5`，`0per` 表示完全缩小。不支持 `px`、`w`、`h` 或无单位写法。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `sc_{数值}per` | `UiTransform.scale.x/y` | X、Y 使用相同缩放比例 |
| `sc_x_{数值}per` | `UiTransform.scale.x` | 只设置 X 轴缩放比例 |
| `sc_y_{数值}per` | `UiTransform.scale.y` | 只设置 Y 轴缩放比例 |

`sc_{数值}per` 展开为 `scale.x` 和 `scale.y` 后参与重复检查。`sc_150per sc_x_200per` 冲突；`sc_x_150per sc_y_200per` 可以组合。

缩放值允许大于 `100per`，也允许 `0per`；不支持负值、`px`、`w`、`h`、`full` 或无单位写法。缩放只接受非负整数百分比，并转换为 Bevy 使用的浮点比例。

## 旋转

旋转样式设置 `UiTransform.rotation`。旋转值只接受非负整数角度，使用 `deg` 单位；`n_` 表示负角度。正值按 Bevy `Rot2::degrees` 的语义逆时针旋转，负值顺时针旋转：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `rt_{角度}deg` | `UiTransform.rotation` | 逆时针旋转指定角度 |
| `rt_n_{角度}deg` | `UiTransform.rotation` | 顺时针旋转指定角度 |
| `rt_0deg` | `UiTransform.rotation` | 恢复默认旋转 |

旋转只允许出现一次；`rt_45deg rt_90deg` 应报告重复属性错误。角度解析使用独立规则，不复用尺寸单位。

## 组合规则

不同变换字段可以组合：

```rust
bstyle!(tr_x_10px tr_y_n_5px sc_110per rt_3deg)
```

上例生成一个 `UiTransform`，同时设置 `translation.x`、`translation.y`、`scale.x`、`scale.y` 和 `rotation`。其中 `tr_...` 会展开为 X、Y 两个字段，`sc_...` 会展开为 X、Y 两个字段；`bstyle!` 与 `bstyle_r` 的最终 Component、字段值和默认值一致。

同一字段不能重复指定；组合样式必须先展开后执行冲突检查。变换样式与 `Node` 的尺寸、间距、定位和 Flex 样式可以组合，但不会改变这些样式参与的布局结果。

## 状态和动画边界

`bstyle!` 和 `bstyle_r` 只描述当前时刻的静态变换值，不提供 Tailwind 的 `hover:`、`focus:`、`active:`、`transition-*` 或 `animate-*` 语义。

应用应由 ECS 系统根据交互状态选择不同的样式 Scene patch；需要连续变化时，由动画系统逐帧修改 `UiTransform`。`bsn!` 的 `on(...)` 可以触发状态变化或启动动画，但不改变本样式的静态语义。

## 暂不支持的 Tailwind Transform 能力

| 能力 | 原因 |
| --- | --- |
| `skew` | Bevy `UiTransform` 没有 skew 字段 |
| `transform-origin` | Bevy `UiTransform` 当前没有等价的原点字段 |
| `perspective`、3D transform | `UiTransform` 是 2D UI 变换 |
| `transform-style`、`backface-visibility` | 没有对应的 Bevy UI 字段 |
| CSS 任意 transform 表达式 | 当前解析器只接受明确的静态值 |

Bevy 原生说明可参考 [UiTransform](https://docs.rs/bevy/latest/bevy/ui/struct.UiTransform.html)；Tailwind 的对应能力可参考 [translate](https://tailwindcss.com/docs/translate)、[scale](https://tailwindcss.com/docs/scale) 和 [rotate](https://tailwindcss.com/docs/rotate)。
