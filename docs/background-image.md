# 背景图片

背景图片样式统一使用 `bgi_` 前缀，对应当前 UI Entity 的 Bevy `ImageNode` Component。`bstyle!` 和 `bstyle_r` 可以直接生成这个 Component 的字段补丁，不创建子节点，也不改变 UI 层级。

## Bevy 的原生规则

Bevy 没有独立的 CSS `background-image` Component。图片直接绘制在当前 Entity 上，布局和图片显示分别由以下字段负责：

| Component | 字段 | 作用 |
| --- | --- | --- |
| `Node` | `width`、`height` 等 | 决定实体的布局尺寸 |
| `ImageNode` | `image` | 指向 `Handle<Image>` 图片资源 |
| `ImageNode` | `image_mode` | 决定图片如何适配 Node |
| `ImageNode` | `color` | 对图片进行颜色叠加 |
| `ImageNode` | `flip_x`、`flip_y` | 翻转图片 |
| `ImageNode` | `rect` | 裁剪图片资源中的区域 |
| `BackgroundColor` | — | 设置纯色背景 |

`ImageNode`、`BackgroundColor`、`BorderColor` 可以同时存在。现有 `bg_*` 继续设置 `BackgroundColor`，不表示图片，也不表示图片 tint。

## 图片资源

### 路径样式

`bgi_url("图片路径")` 设置 `ImageNode.image`。路径只保存到 Bevy Scene 的资源模板中，真正的 `AssetServer` 加载发生在 Scene 展开时：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `bgi_url("images/panel.png")` | `ImageNode.image` | 加载并使用指定路径的图片 |

编译期用法：

```rust
bstyle!(w_320px h_200px bgi_url("images/panel.png") bgi_stretch)
```

这里生成的字段语义等价于：

```rust
bsn! {
    ImageNode {
        image: "images/panel.png",
        image_mode: NodeImageMode::Stretch,
    }
}
```

字段名称必须是 Bevy 的 `ImageNode.image`，不是 `ImageNode.img`。`url` 是 `bgi_` 下的资源值操作名，路径作为括号内的字符串参数传入。

### 与 `bsn!` 和 Rust 组合

图片资源也可以单独由 `bsn!` 或应用代码提供，样式只补充显示效果：

```rust
bsn! {
    ImageNode { image: "images/panel.png" }
    bstyle!(bgi_stretch bgi_flip_y)
}
```

应用也可以先通过 `AssetServer` 取得 `Handle<Image>`，再使用 `ImageNode::new(handle)`。无论采用哪种写法，资源字段和显示字段都必须合并到当前 Entity 的同一个 `ImageNode`，不能生成两个同类型 Component，也不能用完整默认组件覆盖已有字段。

如果只使用 `bgi_stretch` 等显示样式，`bstyle!` 仍应生成 `ImageNode`；没有实际图片资源时，Bevy 的默认图片是透明的。样式系统不负责资源生命周期，图片是否加载成功由 Bevy 资产系统管理。

### 运行时用法

`bstyle_r` 使用同一套样式语法：

```rust
bstyle_r(r#"w_320px h_200px bgi_url("images/panel.png") bgi_stretch"#)
```

运行时样式需要先识别 `bgi_url("...")` 函数值，再在 Scene 展开阶段生成 `ImageNode.image` 的资源模板。编译期不检查文件是否存在，也不要求资源文件位于当前工作区；编译期只检查函数形式和字符串参数是否合法。路径不存在属于 Bevy 的资产加载失败，通常由资产系统报告并留下未加载的句柄，不属于编译错误，也不应由 bevywind 主动 panic。括号、引号缺失、空路径或样式冲突则按运行时样式规则 panic。

## 图片适配

以下样式设置 `ImageNode.image_mode`：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `bgi_auto` | `ImageNode.image_mode` | 使用 `NodeImageMode::Auto`，让图片原始尺寸参与布局 |
| `bgi_stretch` | `ImageNode.image_mode` | 使用 `NodeImageMode::Stretch`，拉伸到 Node 尺寸 |
| `bgi_repeat` | `ImageNode.image_mode` | 使用 `NodeImageMode::Tiled`，横向和纵向平铺 |
| `bgi_repeat_x` | `ImageNode.image_mode` | 使用 `NodeImageMode::Tiled`，只横向平铺 |
| `bgi_repeat_y` | `ImageNode.image_mode` | 使用 `NodeImageMode::Tiled`，只纵向平铺 |
| `bgi_no_repeat` | `ImageNode.image_mode` | 保持单张图片，不进行平铺 |

`bgi_repeat`、`bgi_repeat_x` 和 `bgi_repeat_y` 是对 `NodeImageMode::Tiled` 的简化封装，内部使用固定的平铺参数。平铺缩放参数和间距暂不开放为独立单位。

Bevy 的 `NodeImageMode::Sliced(TextureSlicer)` 需要切片边界和缩放策略等结构化参数，暂不设计成单个普通 token。后续支持时应增加专门的结构化写法。

## 图片变换

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `bgi_flip_x` | `ImageNode.flip_x` | 水平翻转图片 |
| `bgi_flip_y` | `ImageNode.flip_y` | 垂直翻转图片 |

图片颜色叠加后续使用 `bgi_tint_*`，写入 `ImageNode.color`。它与 `bg_*` 的语义不同：`bgi_tint_*` 修改图片显示颜色，`bg_*` 设置实体的纯色背景。

## 不纳入当前设计的 CSS 能力

| CSS 能力 | 当前处理 |
| --- | --- |
| `bg-[url(...)]` | 不采用方括号语法，统一使用 `bgi_url("...")` |
| `bg-cover`、`bg-contain` | Bevy 当前没有与 CSS 完全一致的背景尺寸模式，暂不提供同名样式 |
| background position | `ImageNode` 没有独立的背景定位字段，暂不提供 `bgi_center` 等样式 |
| background attachment | Bevy 没有 CSS `fixed`、`local`、`scroll` 背景附着模型 |
| CSS 渐变 | `BackgroundGradient` 是独立 Component，后续单独设计 |
| CSS 变量和任意 CSS 图片表达式 | 当前解析器不执行 CSS 表达式 |

## 组合和冲突

背景图片样式可以和 Node、纯色背景、边框及其它样式组合：

```rust
bstyle!(
    w_320px h_200px
    bg_blue_500
    bgi_url("images/panel.png")
    bgi_stretch
    bgi_flip_y
)
```

上例应生成一个 `Node`、一个 `BackgroundColor` 和一个 `ImageNode`。同一 `ImageNode` 字段不能重复指定：

- 多个 `bgi_url("...")` 冲突，因为都设置 `ImageNode.image`。
- 多个适配样式冲突，因为都设置 `ImageNode.image_mode`。
- `bgi_url("...") bgi_stretch bgi_flip_y` 可以组合，因为分别设置 `image`、`image_mode` 和 `flip_y`。

`bstyle!` 在编译期把样式展开为 `ImageNode` 字段补丁，但不检查图片文件；`bstyle_r` 在运行时解析相同语法并生成相同的 Scene 结果。两者的区别只有样式语法错误的发现时机不同，资源文件是否存在始终由运行时资产系统处理。

Bevy 原生说明可参考 [ImageNode](https://docs.rs/bevy/latest/bevy/ui/widget/struct.ImageNode.html) 和 [NodeImageMode](https://docs.rs/bevy/latest/bevy/ui/widget/enum.NodeImageMode.html)；Tailwind 背景图片语法可参考 [background-image](https://tailwindcss.com/docs/background-image)。
