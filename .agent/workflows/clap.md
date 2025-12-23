---
description: Clap CLI 框架最佳实践 - 基于 Derive API 官方教程
---

# Clap SKILL

本文档总结 [clap](https://docs.rs/clap/latest/clap/) Derive API 最佳实践,指导项目内 CLI 开发。

---

## 1. 依赖配置

在 `Cargo.toml` 中启用 `derive` feature:
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

---

## 2. 基本结构 (Parser)

使用 `#[derive(Parser)]` 定义 CLI 入口:

```rust
use clap::Parser;

/// 应用简短描述 (显示在 --help)
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// 配置文件路径
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// 调试级别 (可重复, 如 -ddd)
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let cli = Cli::parse();
    // ...
}
```

### 关键属性
- `#[command(version, about)]` - 从 `Cargo.toml` 读取版本和描述
- `#[arg(short, long)]` - 生成 `-c` 和 `--config` 形式
- `#[arg(value_name = "NAME")]` - 帮助文本中的占位符名称

---

## 3. 子命令 (Subcommand)

使用 `#[derive(Subcommand)]` 定义子命令枚举:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 添加新条目
    Add {
        /// 条目名称
        name: String,
    },
    /// 删除条目
    Remove {
        /// 条目 ID
        #[arg(short, long)]
        id: u32,

        /// 强制删除
        #[arg(short, long)]
        force: bool,
    },
    /// 列出所有条目
    List,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { name } => { /* ... */ }
        Commands::Remove { id, force } => { /* ... */ }
        Commands::List => { /* ... */ }
    }
}
```

### 可选子命令
若子命令为可选, 使用 `Option<Commands>`:
```rust
#[command(subcommand)]
command: Option<Commands>,
```

---

## 4. 参数类型

### 4.1 位置参数 (Positional)
```rust
/// 输入文件
input: String,
```

### 4.2 可选参数 (Optional)
```rust
/// 输出文件 (可选)
output: Option<String>,
```

### 4.3 带默认值
```rust
/// 并发数
#[arg(short, long, default_value_t = 4)]
jobs: u8,
```

### 4.4 多值参数
```rust
/// 要处理的文件列表
#[arg(short, long)]
files: Vec<PathBuf>,
```

### 4.5 枚举值 (限定选项)
```rust
use clap::ValueEnum;

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[arg(short, long, value_enum, default_value_t = OutputFormat::Json)]
format: OutputFormat,
```

---

## 5. 验证 (Validation)

### 5.1 内置验证
```rust
/// 端口号 (1-65535)
#[arg(short, long, value_parser = clap::value_parser!(u16).range(1..))]
port: u16,
```

### 5.2 自定义验证
```rust
fn parse_duration(s: &str) -> Result<Duration, String> {
    let secs: u64 = s.parse().map_err(|_| "无效的秒数")?;
    Ok(Duration::from_secs(secs))
}

#[arg(long, value_parser = parse_duration)]
timeout: Duration,
```

---

## 6. 帮助文本

### 6.1 Doc Comments
使用 `///` 自动生成帮助文本:
```rust
/// 设置日志级别
#[arg(short, long)]
log_level: String,
```

### 6.2 长描述
```rust
#[command(
    about = "简短描述",
    long_about = "详细描述\n支持多行"
)]
```

### 6.3 示例
```rust
#[command(after_help = "示例:\n  mycli add foo\n  mycli remove --id 123")]
```

---

## 7. 测试

使用 `try_parse_from` 进行单元测试:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add() {
        let cli = Cli::try_parse_from(["mycli", "add", "foo"]).unwrap();
        assert!(matches!(cli.command, Commands::Add { name } if name == "foo"));
    }

    #[test]
    fn test_missing_required() {
        let result = Cli::try_parse_from(["mycli"]);
        assert!(result.is_err());
    }
}
```

---

## 8. 项目结构建议

```
src/
├── main.rs       # 入口, 调用 Cli::parse()
├── cli.rs        # Cli 结构体与 Commands 枚举定义
└── commands/     # 每个子命令的实现
    ├── mod.rs
    ├── add.rs
    └── remove.rs
```

---

## 9. 常见问题

### Q: 如何处理无子命令时的默认行为?
使用 `Option<Commands>` 并在 `None` 分支处理。

### Q: 如何支持 `--` 后的透传参数?
```rust
#[arg(last = true)]
passthrough: Vec<String>,
```

---

## 参考资料
- [clap Derive Tutorial](https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html)
- [clap Cookbook](https://docs.rs/clap/latest/clap/_derive/_cookbook/index.html)
- [clap API Docs](https://docs.rs/clap/latest/clap/)
