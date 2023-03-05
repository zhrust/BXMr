# BXMr
> rIME-Squirrel 表形码维护工具

## backgroung
作为一名程序猿, 总是要对输入法也要有全面控制

所以, 一直有折腾:
[ZqBXM/mac at master · ZoomQuiet/ZqBXM](https://github.com/ZoomQuiet/ZqBXM/tree/master/mac)

## goal

全面使用 Rust 重构原有 Python 版本的维护指令集


## trace

- 最早是 手工维护
- 后来用 Python 写了个脚本, 但是还是要人工复制到对应目录中再重新编译码表
- 又后来, 使用 invoke 框架拓展并增强了 BXM 码表的维护内部编辑
- 现在, 轮到 Rust 来重构了...

### plan

- [x] 合理工程结构+日志
    - [x] CLI 指令识别,响应
- [x] init ~ 空白全量.toml
    - [x] 选定数据类型
    - [x] 转化为 TOML 写入指定文件 ~ bxmr.local.toml
- [x] .env 来缓存所有关键配置
    - [x] 合理创建
    - [x] 增删改查
    - [x] 替代原有 src/_settings.toml
- [x] renew ~ 从 rIME 导入原有码表
    - [x] Yaml 加载/解析
    - [x] Yaml 数据对应写入 Toml
- [x] seek ~ 根据 code/键码找文字
- [ ] find ~ 根据 word/文字找 code
- [ ] upd ~ 给定 code,word 追加码表条目
- [ ] dele ~ 给定 code,word 删除对应条目
- [ ] gen ~ 从 .toml -> .yaml 以供 rIME 编译加载
    - [x] 本地? -> cfg 指令配置到 .env 中保存
    - [ ] 真实目标 rIME 目录, 检验编译
- [ ] 使用 SQLite3 来替代 .toml 加速
    - [ ] 设计 SQLite 数据库表
    - [ ] 从 .toml 加载回原有 码表数据
    - [ ] 替换原有 .toml 加载/回写行为

## refer.

- [clap::_derive::_cookbook::git_derive - Rust](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
    - 简化官方示例,完成结构性探索
- [Building a CLI from scratch with Clapv3 | by Ukpai Ugochi | Medium](https://medium.com/javascript-in-plain-english/coding-wont-exist-in-5-years-this-is-why-6da748ba676c)
    - 很囧的案例, 看起来很美却根本编译不过...
- [Interacting with databases in Rust using Diesel vs. SQLx - LogRocket Blog](https://blog.logrocket.com/interacting-databases-rust-diesel-vs-sqlx/)
    - [Which one to use postgres vs. sqlx - The Rust Programming Language Forum](https://users.rust-lang.org/t/which-one-to-use-postgres-vs-sqlx/63680)
    - [diesel-rs/metrics](https://github.com/diesel-rs/metrics/) ~while sqlx seems to be about 2 times slower according to our benchmarks ...
    - [diesel vs sqlx - compare differences and reviews? | LibHunt](https://diesel.rs/)
    - [Compare with Diesel | SeaORM 🐚 An async & dynamic ORM for Rust](https://www.sea-ql.org/SeaORM/docs/internal-design/diesel/)
    - ...
- ...

## logging

- ...
- 230227 ZQ mod/clap/tracing/... 项目结构厘定
- 230225 ZQ re-re-re-init.



```
      _~∽--~_
  () /  * -  \ ()
    '_   ▽   _'
    > '--∽--' /

...act by ferris-actor v0.2.4 (built on 23.0303.201916)
```




