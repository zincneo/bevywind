# Bevywind Agent Guide

## 项目定位

`bevywind` 是面向 Bevy UI 的 Tailwind CSS 风格样式库。它只为当前 UI Entity 生成样式 Scene patch，不创建子节点，也不负责描述 UI 层级；UI 层级继续使用 Bevy 的 `bsn!`。

工作区由三个 crate 组成：

- 根 crate：运行时 `style_runtime` 和公开的 `bstyle!`
- `core`：样式 token 解析、属性和值定义、重复属性检查、补全列表
- `macros`：`bstyle!` 过程宏，将解析结果生成 Bevy Scene

## 样式与 Bevy Component 规则

### Node 是基础组件

只要 `bstyle!` 传入了有效样式，生成的 Scene 必须至少包含一个 `bevy::ui::Node`。所有需要写入 `Node` 的样式必须合并到这一个 `Node` 中，不能按样式类别生成多个 `Node`：

- 尺寸：`Node.width`、`Node.height`、`Node.min_width`、`Node.min_height`、`Node.max_width`、`Node.max_height`
- Flex：`Node.display`、`Node.flex_direction`、`Node.flex_wrap`、`Node.justify_content`、`Node.align_items`、`Node.align_content`
- 间距：`Node.margin` 和 `Node.padding` 的 `UiRect` 方向字段
- 边框宽度：`Node.border` 的 `UiRect` 方向字段

背景色和边框颜色属于特殊情况：它们在基础 `Node` 之外，分别额外生成 `BackgroundColor` 和 `BorderColor` Component。

`style_runtime` 对非空且解析成功的样式也先确保存在一个 `Node`；背景色或边框颜色只是在此基础上额外插入对应颜色 Component。空字符串或解析失败时不生成 patch。

### 当前样式命名

样式 token 使用下划线 `_`，不使用连字符 `-`。当前已实现：

- 尺寸：`h_*`、`w_*`、`min_h_*`、`min_w_*`、`max_h_*`、`max_w_*`
- Flex：`flex_row`、`flex_col`、`flex_wrap`、`justify_*`、`items_*`、`content_*`、`flex_center`
- 背景色：`bg_*`，支持预设色、色阶、`rrggbb`、`rrggbbaa`
- 间距：`m_*`、`ml_*`、`mr_*`、`mt_*`、`mb_*`、`p_*`、`pl_*`、`pr_*`、`pt_*`、`pb_*`
- 边框宽度：`b_*`、`bl_*`、`br_*`、`bt_*`、`bb_*`
- 边框颜色：同样使用 `b_`、`bl_`、`br_`、`bt_`、`bb_`，值为 `rrggbb` 或 `rrggbbaa`

尺寸、间距和边框宽度的数值格式沿用同一套规则：

- `full`：`percent(100)`
- `{数字}px`：`px({数字})`
- `{数字}per`：`percent({数字})`
- `{数字}w`：`vw({数字})`
- `{数字}h`：`vh({数字})`

数字按非负 `u16` 解析。新增样式时优先复用 `core/src/dimension.rs` 的值解析逻辑，不要重新实现一套数值解析器。

### 全方向样式与重复检查

`m_*`、`p_*` 和 `b_*` 等全方向样式必须展开成四个方向属性，再参与重复属性检查。例如：

- `m_10px` 占用 `margin.left/right/top/bottom`
- `m_10px ml_20px` 必须报重复属性错误
- `mt_10px mb_20px` 可以同时使用

同理适用于 padding 和 border。新增组合样式时，也必须展开后再检查冲突。

## 代码实现约定

新增样式通常需要同步修改以下位置：

1. `core/src/lib.rs`：增加 `Property`、`Value`，接入解析和必要的补全项。
2. 对应的 `core/src/*.rs`：实现 token 解析和全方向展开。
3. `src/*.rs`：实现运行时对 Bevy Component 字段的赋值。
4. `macros/src/lib.rs`：让 `bstyle!` 生成正确的字段或额外 Component。
5. `tests/*.rs`：增加宏编译、解析展开、冲突检查和运行时 Scene 测试。
6. `docs/*.md` 和 `README.md`：同步文档和样式索引。

过程宏生成的 Node 样式字段要汇总到同一个 `Node { ... }`。需要构造 `UiRect` 时，使用 Bevy 可被 `bsn!` 接受的表达式形式，例如 `UiRect::new(...)`，不要生成 `bsn!` 不接受的嵌套结构字面量。

当一个新样式写入新的 Component 时，应保留基础 `Node`，并将新 Component 作为额外节点组件生成。颜色类 Component 的默认未指定方向应保持 Bevy 的默认值语义。

## 文档规范

每个样式文档遵循统一顺序：

1. 大标题；只有尚未实现的文档在标题后写 `（未实现）`。
2. 标题后的第一段必须先声明对应的 Bevy Component，以及具体会设置哪些字段。
3. 按功能分章节说明语法。
4. 使用统一表格格式：`语法 | Bevy 属性 | 含义`。
5. 值格式、限制、组合规则和示例放在映射声明之后。

已实现的样式不能继续保留“未实现”标记；只有设计文档才在大标题标记，正文不要重复堆叠状态说明。

文档中的映射必须与代码一致。例如边框文档应明确：宽度写入 `Node.border`，颜色写入 `BorderColor`；间距文档应明确：margin/padding 写入 `Node` 的 `UiRect` 字段。

## 测试与验证

完成代码修改后至少执行：

```bash
cargo fmt --all -- --check
cargo test
git diff --check
```

测试应覆盖：

- 每个新 token 的 `bstyle!` 编译接受
- `core::parse_classes` 返回的属性和值
- 全方向展开为四个方向字段
- 冲突方向被拒绝，互不冲突方向可以组合
- `style_runtime` 可以解析新样式
- 新样式与尺寸、Flex、间距、背景色等组合时仍只产生一个基础 `Node`

仅修改文档时不需要运行 Rust 测试，但应执行 `git diff --check`。
