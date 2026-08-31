# 溢出控制

溢出样式作用于当前实体的 Bevy UI `Node` 组件，设置 `Node.overflow.x` 和 `Node.overflow.y`。每个轴使用 `OverflowAxis` 表示内容溢出时的处理方式。

本章需要区分两层能力：`bstyle!` 或 `bstyle_r` 只负责配置 `Node.overflow`，决定内容是显示、裁剪还是允许滚动；它们不会自动添加交互组件。要让用户能够通过鼠标或触控板滚轮滚动，还必须在应用中额外配置 Bevy 的 UI Widgets 滚动能力。

## 双轴溢出

双轴样式同时设置 `Node.overflow.x` 和 `Node.overflow.y`：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `overflow_visible` | `Node.overflow.x`、`Node.overflow.y` | 显示两个方向的溢出内容 |
| `overflow_clip` | `Node.overflow.x`、`Node.overflow.y` | 裁剪两个方向的溢出内容 |
| `overflow_hidden` | `Node.overflow.x`、`Node.overflow.y` | 隐藏两个方向的溢出内容，按裁剪处理 |
| `overflow_scroll` | `Node.overflow.x`、`Node.overflow.y` | 两个方向启用溢出滚动 |

`overflow_hidden` 按 Tailwind 的视觉语义映射为 Bevy 的 `OverflowAxis::Clip`。Bevy 另有 `OverflowAxis::Hidden`，它会先影响布局再进行裁剪，与 Tailwind `overflow-hidden` 不完全等价，因此不单独设计对应 token。

## 单轴溢出

单轴样式只修改指定方向，另一个方向保持原有值：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `overflow_x_visible` | `Node.overflow.x` | 显示横向溢出内容 |
| `overflow_x_clip` | `Node.overflow.x` | 裁剪横向溢出内容 |
| `overflow_x_hidden` | `Node.overflow.x` | 隐藏横向溢出内容，按裁剪处理 |
| `overflow_x_scroll` | `Node.overflow.x` | 横向启用溢出滚动 |
| `overflow_y_visible` | `Node.overflow.y` | 显示纵向溢出内容 |
| `overflow_y_clip` | `Node.overflow.y` | 裁剪纵向溢出内容 |
| `overflow_y_hidden` | `Node.overflow.y` | 隐藏纵向溢出内容，按裁剪处理 |
| `overflow_y_scroll` | `Node.overflow.y` | 纵向启用溢出滚动 |

## 组合与重复检查

双轴样式需要先展开为两个轴的具体属性，再参与重复属性检查：

```rust
bstyle!(overflow_clip)
bstyle!(overflow_x_clip overflow_y_scroll)
```

`overflow_clip overflow_x_hidden` 会报告 `Node.overflow.x` 重复；`overflow_x_clip overflow_y_scroll` 可以组合使用。未被样式指定的轴保持原有值。

## 与 Bevy 和 Tailwind 的差异

Bevy 的 `Overflow` 直接提供 `Visible`、`Clip`、`Hidden` 和 `Scroll` 四种轴值；Tailwind 提供 `visible`、`clip`、`hidden`、`auto` 和 `scroll` 五种值，并支持双轴及单轴写法。[Bevy Overflow](https://docs.rs/bevy/latest/bevy/ui/struct.Overflow.html) · [Bevy OverflowAxis](https://docs.rs/bevy/latest/bevy/ui/enum.OverflowAxis.html) · [Tailwind Overflow](https://tailwindcss.com/docs/overflow)

当前设计暂不加入以下 token：

- `overflow_auto`、`overflow_x_auto`、`overflow_y_auto`：Bevy 没有与 CSS `auto` 完全一致的轴值。直接映射为 `Scroll` 会把“仅在需要时滚动”的语义变成“启用滚动”，需要后续确认滚动行为后再决定。
- 任意 CSS 值和 CSS 变量：当前样式值系统不执行 CSS 表达式。

## 样式与交互的关系

`overflow_scroll` 设置后，Bevy 才会把对应轴作为可滚动区域处理，但这只改变 `Node` 的布局和裁剪行为，不会自动响应用户输入。真正可交互的滚动区域至少需要以下配置：

| 配置 | 添加位置 | 作用 |
| --- | --- | --- |
| `bstyle!(overflow_y_scroll)` 或 `bstyle_r("overflow_y_scroll")` | 滚动容器的 `Node` | 将纵轴设置为 `OverflowAxis::Scroll` |
| `ScrollArea` | 与滚动容器相同的 Entity | 接收鼠标、触控板滚轮等输入 |
| `ScrollAreaPlugin` | Bevy App | 注册 `ScrollArea` 所需的交互处理逻辑 |

`ScrollArea` 和 `ScrollAreaPlugin` 来自 `bevy::ui_widgets`。它们属于交互配置，不属于 bevywind 的样式 patch，因此不会由 `bstyle!` 或 `bstyle_r` 自动生成。滚动容器和 `ScrollArea` 必须是同一个 Entity；滚动内容仍然作为该 Entity 的子节点创建。

典型结构可以理解为：

```text
App
└── ScrollAreaPlugin
    └── 滚动容器 Entity
        ├── Node.overflow = Overflow::scroll_y()
        ├── ScrollArea
        └── 滚动内容子节点
```

## 配置用户滚动交互

要让用户通过鼠标或触控板滚动，应用需要在 Bevy App 和滚动容器上分别配置：

| 配置 | 位置 | 作用 |
| --- | --- | --- |
| `ScrollAreaPlugin` | `App` | 注册滚轮和触控板滚动的事件处理逻辑 |
| `ScrollArea` | 滚动容器 Entity | 标记该 Entity 接收滚轮和触控板滚动输入 |
| `ScrollPosition` | 滚动容器 Entity，可选 | 保存、初始化或由代码修改滚动位置 |

滚动容器必须同时满足以下条件：

1. 通过 `bstyle!` 或 `bstyle_r` 将至少一个轴设置为 `Scroll`，例如 `overflow_y_scroll`。
2. 在同一个 Entity 上添加 `ScrollArea`。
3. 在 App 中添加 `ScrollAreaPlugin`。
4. 将需要滚动的内容作为该 Entity 的子节点创建。

`ScrollPosition` 不是用户输入组件。即使没有 `ScrollArea`，应用仍然可以通过代码修改它来实现程序控制滚动；但仅设置 `overflow_y_scroll` 不会自动处理用户输入。

## 配置可见滚动条

Bevy 的 `OverflowAxis::Scroll` 不会自动生成可见滚动条。Bevy UI Widgets 提供的是无样式的 headless scrollbar，滚动条的容器布局和视觉样式需要应用自行创建。

一个完整的纵向滚动条至少由以下部分组成：

| 配置 | 所在 Entity | 作用 |
| --- | --- | --- |
| `ScrollbarPlugin` | `App` | 注册滚动条点击和拖动的交互逻辑 |
| `Scrollbar` | 滚动条轨道 Entity | 通过 `target` 指向滚动容器，并通过 `orientation` 指定横向或纵向 |
| `ScrollbarThumb` | 滚动条 Entity 的子 Entity | 标记会随滚动位置和内容比例移动、缩放的滑块 |
| `Node`、`BackgroundColor` 等 | 轨道和滑块 Entity | 由应用设置轨道和滑块的布局、颜色及其它外观 |

`Scrollbar::new(target, orientation, min_thumb_length)` 的 `target` 必须是设置了对应 `OverflowAxis::Scroll` 的滚动容器。`Scrollbar` 的 `orientation` 决定它控制横向还是纵向滚动；`min_thumb_length` 是滑块的最小长度。`ScrollbarThumb` 必须是 `Scrollbar` Entity 的子节点，Bevy 会根据可视区域、内容尺寸和 `ScrollPosition` 更新它的位置与尺寸。

滚动条可以放在滚动容器旁边，也可以覆盖在滚动容器上方；应用需要自行决定轨道和滑块的 UI 层级与布局。滚动条本身不是 bevywind 样式的一部分，bevywind 只负责滚动容器的 `Node.overflow`。

因此，三种结果应明确区分：

| 配置 | 结果 |
| --- | --- |
| 只有 `bstyle!(overflow_y_scroll)` | 节点具备纵向滚动模式，但没有用户输入和可见滚动条 |
| 加上 `ScrollArea` 与 `ScrollAreaPlugin` | 可以通过鼠标或触控板滚动，但不会自动显示滚动条 |
| 再加上 `Scrollbar`、`ScrollbarThumb` 与 `ScrollbarPlugin` | 可以使用可见滚动条拖动或点击控制目标滚动容器 |
