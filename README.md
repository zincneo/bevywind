# bevywind

`bevywind` 是一个面向 Bevy UI 的 Tailwind CSS 风格样式库。

它只负责为当前 UI 实体生成样式 patch，不负责创建子实体或描述 UI 层级。UI 结构仍然使用 Bevy 的 `bsn!` 编写。

## 使用方式

```rust
use bevy::prelude::*;
use bevywind::bstyle;

fn scene() -> impl Scene {
    bsn! {
        bstyle!(w_full h_full)

        Children [
            Text("Hello")
        ]
    }
}
```

`bstyle!` 是过程宏。样式 token 会在编译期解析，解析失败时会产生编译期错误：

```rust
bstyle!(h_10px w_50per)
```

`bstyle!` 只接受样式 token，不接受字符串；它要求至少有一个样式，不支持空调用：

```rust
bstyle! {}       // 不支持
bstyle!("")      // 不支持
```

## 动态样式

运行时才确定的样式使用 `bstyle_r`：

```rust
use bevywind::bstyle_r;

fn scene(classes: &String) -> impl Scene {
    bsn! {
        bstyle_r(classes)
    }
}
```

`bstyle_r` 接收实现 `AsRef<str>` 的值，例如 `&str`、`String` 和 `&String`。动态样式会在运行时解析。

## 样式文档

1. [高度和宽度](docs/dimensions.md)
2. [背景色](docs/background.md)
3. [Flex 布局](docs/flex.md)
4. [外边距和内边距](docs/spacing.md)
5. [边框](docs/border.md)
6. [文字与排版](docs/typography.md)
