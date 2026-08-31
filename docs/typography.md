# 文字与排版

本组样式作用于当前实体的 Bevy UI 文字组件：颜色写入 `TextColor`，字号、字重和斜体写入 `TextFont`，对齐和换行写入 `TextLayout`，字高写入 `LineHeight`。字体资源（`.ttf`、`.otf`）和字体族由 `bsn!` 或调用方直接设置；其他字体效果由 `bstyle!` 使用 `t_` 前缀指定。样式不创建 `Text`、`TextSpan` 或子实体。

## 文字颜色

文字颜色沿用背景色的预设颜色和十六进制颜色格式，但使用 `t_` 前缀：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_transparent` | `TextColor.0` | 完全透明 |
| `t_black` | `TextColor.0` | 黑色 |
| `t_white` | `TextColor.0` | 白色 |
| `t_{颜色}` | `TextColor.0` | 使用预设颜色的 `500` 色阶 |
| `t_{颜色}_{色阶}` | `TextColor.0` | 使用指定色阶，范围为 `50–950` |
| `t_rrggbb` | `TextColor.0` | 不透明十六进制颜色 |
| `t_rrggbbaa` | `TextColor.0` | 带透明度的十六进制颜色 |

预设颜色与 `bg_*` 相同，完整列表如下。每种颜色均支持不带色阶的写法（默认使用 `500`）以及 `50`、`100`、`200`、`300`、`400`、`500`、`600`、`700`、`800`、`900`、`950` 色阶：

| 颜色 | 可用语法 |
| --- | --- |
| `slate` | `t_slate`、`t_slate_{色阶}` |
| `gray` | `t_gray`、`t_gray_{色阶}` |
| `zinc` | `t_zinc`、`t_zinc_{色阶}` |
| `neutral` | `t_neutral`、`t_neutral_{色阶}` |
| `stone` | `t_stone`、`t_stone_{色阶}` |
| `red` | `t_red`、`t_red_{色阶}` |
| `orange` | `t_orange`、`t_orange_{色阶}` |
| `amber` | `t_amber`、`t_amber_{色阶}` |
| `yellow` | `t_yellow`、`t_yellow_{色阶}` |
| `lime` | `t_lime`、`t_lime_{色阶}` |
| `green` | `t_green`、`t_green_{色阶}` |
| `emerald` | `t_emerald`、`t_emerald_{色阶}` |
| `teal` | `t_teal`、`t_teal_{色阶}` |
| `cyan` | `t_cyan`、`t_cyan_{色阶}` |
| `sky` | `t_sky`、`t_sky_{色阶}` |
| `blue` | `t_blue`、`t_blue_{色阶}` |
| `indigo` | `t_indigo`、`t_indigo_{色阶}` |
| `violet` | `t_violet`、`t_violet_{色阶}` |
| `purple` | `t_purple`、`t_purple_{色阶}` |
| `fuchsia` | `t_fuchsia`、`t_fuchsia_{色阶}` |
| `pink` | `t_pink`、`t_pink_{色阶}` |
| `rose` | `t_rose`、`t_rose_{色阶}` |

此外还提供 `t_transparent`、`t_black` 和 `t_white` 三种特殊颜色。一个实体只能指定一个文字颜色。

```rust
bstyle!(t_white)
bstyle!(t_blue_500)
bstyle!(t_ffffffcc)
```

## 字号

字号对应 Tailwind 的 `text-{size}`，在本项目中使用下划线命名，并写入 `TextFont.font_size`。字号值使用非负数字和 `px` 后缀；不使用当前 Node 尺寸的 `per`、`w`、`h` 单位，因为 Bevy 的 `FontSize` 表示逻辑像素字号。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_xs` | `TextFont.font_size` | `12px` |
| `t_sm` | `TextFont.font_size` | `14px` |
| `t_base` | `TextFont.font_size` | `16px` |
| `t_lg` | `TextFont.font_size` | `18px` |
| `t_xl` | `TextFont.font_size` | `20px` |
| `t_2xl` | `TextFont.font_size` | `24px` |
| `t_3xl` | `TextFont.font_size` | `30px` |
| `t_4xl` | `TextFont.font_size` | `36px` |
| `t_5xl` | `TextFont.font_size` | `48px` |
| `t_6xl` | `TextFont.font_size` | `60px` |
| `t_7xl` | `TextFont.font_size` | `72px` |
| `t_8xl` | `TextFont.font_size` | `96px` |
| `t_9xl` | `TextFont.font_size` | `128px` |
| `t_{数字}px` | `TextFont.font_size` | 设置自定义字号 |

预设字号参考 Tailwind 默认主题；自定义字号中的数字按非负 `u16` 解析。

## 文本对齐

文本对齐对应 Tailwind 的 `text-left`、`text-center` 等工具，并设置 `TextLayout.justify`。它控制多行文本内部各行的水平排列，不改变实体在父节点中的 Flex/Grid 位置。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_left` | `TextLayout.justify` | `Justify::Left` |
| `t_center` | `TextLayout.justify` | `Justify::Center` |
| `t_right` | `TextLayout.justify` | `Justify::Right` |
| `t_justify` | `TextLayout.justify` | `Justify::Justified` |
| `t_start` | `TextLayout.justify` | `Justify::Start`，按文字方向起始端对齐 |
| `t_end` | `TextLayout.justify` | `Justify::End`，按文字方向结束端对齐 |

## 字高

字高对应 Tailwind 的 `leading-*`，写入 `LineHeight`。支持固定像素字高和相对于字号的倍数：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_leading_none` | `LineHeight` | `RelativeToFont(1.0)` |
| `t_leading_tight` | `LineHeight` | `RelativeToFont(1.25)` |
| `t_leading_snug` | `LineHeight` | `RelativeToFont(1.375)` |
| `t_leading_normal` | `LineHeight` | `RelativeToFont(1.5)` |
| `t_leading_relaxed` | `LineHeight` | `RelativeToFont(1.625)` |
| `t_leading_loose` | `LineHeight` | `RelativeToFont(2.0)` |
| `t_leading_{数字}px` | `LineHeight` | `Px({数字})` |
| `t_leading_{数字}rel` | `LineHeight` | `RelativeToFont({数字}/100)` |

同一个实体只能指定一个 `LineHeight`。`t_leading_normal` 的设计值为 `1.5`，与 Tailwind 默认语义一致；未指定时保留 Bevy 默认的 `1.2`。

## 文本换行

文本换行对应 Tailwind 的 `whitespace-*` 和 `break-*` 的主要语义，并设置 `TextLayout.linebreak`。换行只影响文字布局；若需要让文本区域产生可见的宽度约束，仍需同时设置 `Node.width` 或 `Node.max_width`。

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_whitespace_normal` | `TextLayout.linebreak` | `LineBreak::WordBoundary`，优先在单词边界换行 |
| `t_whitespace_nowrap` | `TextLayout.linebreak` | `LineBreak::NoWrap`，只保留显式换行 |
| `t_break_words` | `TextLayout.linebreak` | `LineBreak::AnyCharacter`，允许在任意字符处换行 |
| `t_break_word` | `TextLayout.linebreak` | `LineBreak::WordOrCharacter`，优先单词，单词过长时按字符换行 |

这些 token 不能与其他换行 token 重复使用。Tailwind 中依赖浏览器 CSS 的 `break_normal`、`break_all`、`break_keep` 等细粒度行为，在 Bevy 的 `LineBreak` 枚举中没有完全一一对应的值，因此暂不单独提供。

## 字体资源与字体样式

字体资源和字体族不属于 `bstyle!` 的职责；`.ttf` 和 `.otf` 应由 `bsn!` 或调用方加载并设置到 `TextFont.font`。字重和斜体属于文字效果，使用 `bstyle!` 的 `t_` 前缀样式，分别写入 `TextFont.weight` 和 `TextFont.style`：

| Bevy 写法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `t_thin` | `TextFont.weight` | `FontWeight::THIN`，100 |
| `t_extralight` | `TextFont.weight` | `FontWeight::EXTRA_LIGHT`，200 |
| `t_light` | `TextFont.weight` | `FontWeight::LIGHT`，300 |
| `t_normal` | `TextFont.weight` | `FontWeight::NORMAL`，400 |
| `t_medium` | `TextFont.weight` | `FontWeight::MEDIUM`，500 |
| `t_semibold` | `TextFont.weight` | `FontWeight::SEMIBOLD`，600 |
| `t_bold` | `TextFont.weight` | `FontWeight::BOLD`，700 |
| `t_extrabold` | `TextFont.weight` | `FontWeight::EXTRA_BOLD`，800 |
| `t_w_black` | `TextFont.weight` | `FontWeight::BLACK`，900 |
| `t_italic` | `TextFont.style` | `FontStyle::Italic`，斜体 |
| `t_not_italic` | `TextFont.style` | `FontStyle::Normal`，常规 |

在 `bsn!` 中可以直接使用字体资源路径：

```rust
bsn! {
    Text("Hello Bevywind")
    TextFont {
        font: FontSourceTemplate::Handle("fonts/NotoSans-Regular.ttf"),
        font_size: 24.0,
    }
}
```

字体资源示例中的 `.ttf` 和 `.otf` 使用相同的 `Font` 资源类型。普通 Rust 代码中则使用 `asset_server.load("fonts/NotoSans-Regular.ttf").into()`。字重是否生效取决于字体资源是否包含对应字形。字体效果示例：

```rust
bstyle!(t_bold t_italic)
```

## 组合规则与组件语义

- `t_{颜色}` 只生成或修改 `TextColor`；文字样式 patch 仍会保留基础 `Node`。
- `t_{字号}`、`t_leading_*`、`t_center`、`t_whitespace_*`、`t_bold` 和 `t_italic` 需要当前实体是 Bevy 文字实体；样式 patch 不创建 `Text`、`TextSpan` 或子实体。
- 颜色、字号、对齐、字高和换行分别占用独立属性，同一类别重复指定时应报告重复属性错误。
- `t_center` 与 `flex_center` 的语义不同：前者设置 `TextLayout.justify`，后者设置 `Node` 的 Flex 布局字段。
- 文字样式可以与 `w_*`、`max_w_*`、`p_*` 等 Node 样式组合；最终文字换行宽度由 Node 的布局约束决定。

## 示例

```rust
bsn! {
    Text("Hello Bevywind")
    bstyle!(
        t_blue_500
        t_lg
        t_leading_relaxed
        t_center
        t_whitespace_normal
        p_10px
        max_w_320px
    )
}
```
