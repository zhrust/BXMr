---
description: Rust 语言开发最佳实践 - 基于官方 Style Guide 与社区惯例
---

# Rust SKILL

本文档综合 [Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/) 及官方最佳实践,指导项目内 Rust 代码开发。

---

## 1. 格式化 (Formatting)

### 1.1 工具
- **必须使用 `rustfmt`** 保持代码格式一致。
- 运行 `cargo fmt` 或 `cargo fmt --check` (CI 检查)。

### 1.2 缩进与行宽
- 使用 **4 空格** 缩进, **禁止 Tab**。
- 行宽上限 **100 字符**。

### 1.3 尾随逗号 (Trailing Commas)
多行列表/参数结尾使用尾随逗号:
```rust
function_call(
    argument,
    another_argument, // <- 尾随逗号
);
```

### 1.4 空行
- 使用 **0 或 1 个空行** 分隔条目。
- 禁止行尾空白。

### 1.5 块缩进优先
优先使用块缩进 (block indent) 而非视觉缩进 (visual indent):
```rust
// 推荐: 块缩进
let result = a_long_function_call(
    foo,
    bar,
);

// 避免: 视觉缩进
let result = a_long_function_call(foo,
                                   bar);
```

---

## 2. 命名规范 (Naming Conventions)

| 类型           | 规范                   | 示例                  |
| -------------- | ---------------------- | --------------------- |
| 类型/Trait     | `UpperCamelCase`       | `BTreeMap`, `Parser`  |
| 函数/方法/变量 | `snake_case`           | `load_config`, `name` |
| 常量/静态变量  | `SCREAMING_SNAKE_CASE` | `MAX_SIZE`            |
| 模块/Crate     | `snake_case`           | `my_module`           |
| 枚举成员       | `UpperCamelCase`       | `Option::Some`        |

---

## 3. 代码惯例 (Idioms)

### 3.1 所有权与借用
- 优先使用 **引用 (`&T`, `&mut T`)** 而非克隆。
- 避免不必要的 `.clone()` 或 `Box` 堆分配。

### 3.2 错误处理
- 使用 `Result<T, E>` 处理可失败操作。
- 使用 `?` 操作符传播错误, 避免 `.unwrap()` (测试代码除外)。
- 错误信息应具描述性。

### 3.3 Option 与模式匹配
- 使用 `if let` / `while let` 简化单分支匹配。
- 利用 `match` 穷尽枚举所有变体。

### 3.4 迭代器
优先使用迭代器方法链而非显式循环:
```rust
// 推荐
let sum: i32 = numbers.iter().filter(|x| **x > 0).sum();

// 避免
let mut sum = 0;
for x in &numbers {
    if *x > 0 { sum += x; }
}
```

### 3.5 表达式优先
Rust 是表达式语言, 善用表达式返回值:
```rust
let status = if success { "ok" } else { "fail" };
```

---

## 4. Cargo.toml 规范

- 字段顺序: `[package]` -> `[dependencies]` -> `[dev-dependencies]` -> `[build-dependencies]`。
- 依赖按字母排序。
- 指定明确版本号 (避免 `*`)。
- 配置 `edition = "2021"` 或更新版本。

---

## 5. 文档与注释

- 使用 `///` 为公开 API 编写文档注释。
- 使用 `//` 进行行内注释, 避免使用 `/* */`。
- 注释应解释 **为什么**, 而非 **是什么**。

---

## 6. Clippy 检查

- **必须运行 `cargo clippy`** 并修复所有警告。
- 禁止全局 `#![allow(unused)]` 等抑制警告。
- CI 中添加 `cargo clippy -- -D warnings`。

---

## 7. 测试

- 单元测试位于同文件 `#[cfg(test)] mod tests { ... }`。
- 集成测试位于 `tests/` 目录。
- 运行: `cargo test`。

---

## 参考资料
- [The Rust Style Guide](https://doc.rust-lang.org/stable/style-guide/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [The Rust Book](https://doc.rust-lang.org/book/)
