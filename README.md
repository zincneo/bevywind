# bevywind

`bevywind` 是一个面向 Bevy UI 的 Tailwind CSS 风格样式库。

它只负责为当前 UI 实体生成样式 patch，不负责创建子节点或描述 UI 层级。UI 结构仍然使用 Bevy 的 `bsn!` 编写。

## 使用方式

```rust
use bevy::prelude::*;
use bevywind::style;

fn scene() -> impl Scene {
    bsn! {
        style!("w-full h-full")

        Children [
            Text("Hello")
        ]
    }
}
```

`style!` 是过程宏。传入的样式字符串会在编译期解析，解析失败时会产生编译期错误：

```rust
style!("h-10px w-50%")
```

它只能接收非空字符串字面量，不支持空调用：

```rust
style! {}       // 不支持
style!("")      // 不支持
```

## 动态样式

运行时才确定的样式使用 `style_runtime`：

```rust
use bevywind::style_runtime;

fn scene(classes: &String) -> impl Scene {
    bsn! {
        style_runtime(classes)
    }
}
```

`style_runtime` 接收实现 `AsRef<str>` 的值，例如 `&str`、`String` 和 `&String`。动态样式会在运行时解析。

## 可用样式

### 高度和宽度

当前支持 `h-*` 高度和 `w-*` 宽度样式：

| 语法 | Bevy 属性 | 含义 |
| --- | --- | --- |
| `h-full` | `height: percent(100)` | 高度占父节点 100% |
| `w-full` | `width: percent(100)` | 宽度占父节点 100% |
| `h-10px` | `height: px(10)` | 高度为 10 逻辑像素 |
| `w-20px` | `width: px(20)` | 宽度为 20 逻辑像素 |
| `h-30%` | `height: percent(30)` | 高度为父节点的 30% |
| `w-40%` | `width: percent(40)` | 宽度为父节点的 40% |
| `h-50w` | `height: vw(50)` | 高度为视口宽度的 50% |
| `w-60w` | `width: vw(60)` | 宽度为视口宽度的 60% |
| `h-70h` | `height: vh(70)` | 高度为视口高度的 70% |
| `w-80h` | `width: vh(80)` | 宽度为视口高度的 80% |

数字部分按 `u16` 解析，因此必须是非负整数，且不能超过 `u16` 的范围。
