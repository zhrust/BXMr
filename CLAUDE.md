# CLAUDE.md

> 本文档为 AI 助手 (如 Claude) 提供项目上下文，便于理解和协助开发。

---

## 项目概述

**BXMr** (BXM Manager) 是一个 Rust CLI 工具，用于管理 [rIME-Squirrel](https://rime.im/) 输入法的 **表形码 (BXM)** 码表。

- **仓库**: https://github.com/zhrust/BXMr
- **版本**: 23.1022.1
- **许可**: BSD 2-Clause

---

## 核心功能

| 命令    | 功能                           |
| ------- | ------------------------------ |
| `cfg`   | 配置 rIME 码表 `.yaml` 路径    |
| `env`   | 检查环境变量配置               |
| `ver`   | 显示版本信息                   |
| `gen`   | 导出内存码表到 `.yaml` 文件    |
| `seek`  | 根据编码查找字词是否存在       |
| `find`  | 根据字词查找对应编码           |
| `upd`   | 添加/更新 编码-字词 映射       |
| `dele`  | 删除 编码-字词 映射            |
| `ahead` | 调整字词优先级 (提升至首选)    |

---

## 项目结构

```
bxmr/
├── Cargo.toml          # 项目配置与依赖
├── src/
│   ├── main.rs         # 入口 (rustyline REPL 循环)
│   ├── inv.rs          # 命令分发模块
│   └── inv/            # 子命令实现
│       ├── _util.rs    # 工具函数 (较大，需重构)
│       ├── cfg.rs      # 配置管理
│       ├── env.rs      # 环境检查
│       ├── ver.rs      # 版本输出
│       ├── gen.rs      # 导出 YAML
│       ├── renew.rs    # 加载码表到 BTreeMap
│       ├── seek.rs     # 编码查找
│       ├── find.rs     # 字词查找
│       ├── upd.rs      # 更新映射
│       ├── dele.rs     # 删除映射
│       └── ahead.rs    # 调整优先级
├── test/               # 测试目录 (当前为空)
└── log/                # 日志目录
```

---

## 已知技术债务

详见 [Issue #2](https://github.com/zhrust/BXMr/issues/2):

1. **CLI 架构**: 手动字符串解析命令，未使用 `clap` Derive API
2. **代码冗余**: 全局 `#![allow(unused)]`，存在废弃代码
3. **性能问题**: 数据更新时不必要的 `BTreeMap` 全量 Clone
4. **测试缺失**: 无单元测试覆盖
5. **模块臃肿**: `_util.rs` 职责不清

---

## 开发约定

### 工具链
```bash
# 格式化
cargo fmt

# Lint 检查
cargo clippy -- -D warnings

# 测试
cargo test

# 构建
cargo build --release
```

### 代码风格
参考 `.agent/workflows/rust.md` (Rust 官方 Style Guide)

### CLI 开发
参考 `.agent/workflows/clap.md` (Clap Derive API 最佳实践)

---

## 运维说明

### 本地运行
```bash
# 直接运行 REPL
cargo run

# 进入交互模式后
BXMr> cfg yaml /path/to/bxm.dict.yaml
BXMr> seek aaa
BXMr> upd aaa 叒
BXMr> gen
```

### 发布
```bash
cargo publish
```

---

## 后续规划

- [ ] 使用 Clap Derive API 重构 CLI
- [ ] 清理废弃代码，移除 `#![allow(unused)]`
- [ ] 优化数据结构操作，减少 Clone
- [ ] 添加单元测试
- [ ] 拆分 `_util.rs` 职责

---

*最后更新: 2025-12-22*
