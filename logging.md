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
- [x] find ~ 根据 word/文字找 code
- [x] upd ~ 给定 code,word 追加码表条目
- [x] dele ~ 给定 code,word 删除对应条目
- [x] gen ~ 从 .toml -> .yaml 以供 rIME 编译加载
    - [x] 本地? -> cfg 指令配置到 .env 中保存
    - [x] 真实目标 rIME 目录, 检验编译
- [x] ahead ~ 给定 code,word 调整上屏推荐顺序到最前
- [ ] 加速全序 .toml 使用
    - [x] 类似 REPL 交互, 在不退出前, 只加载一次码表到内存中
    - [ ] 迁移所有指令到对应识别中
      - [ ] help
      - [ ] cfg
      - [ ] env
      - [ ] renew ~ 每次都使用 .yaml 在内存中重建 字典 全集,不再从 toml 加载
      - [ ] seek
      - [ ] find
      - [ ] upd
      - [ ] dele
      - [ ] ahead
      - [ ] gen
    - [ ] 检验多次写出到 .yaml 时是否冲突
    - [ ] ...
- [ ] ?用 SQLite3 来替代 .toml 加速
    - [ ] 设计 SQLite 数据库表
    - [ ] 从 .toml 加载回原有 码表数据
    - [ ] 替换原有 .toml 加载/回写行为

### show
> 如何动态的展示所有关键功能? 当然是操作动画了

- [Terminalizer](https://www.terminalizer.com/view/4884aa0e7) ~ NODE 的败退
- [asciinema/asciinema: Terminal session recorder 📹](https://asciinema.org/a/335480?autoplay=1) ~ Python 的, 不过将录制下来的脚本变成 gif 动画的工具已经失传了
- [How to create a Screencast GIF. Use free existing tools on Mac OSX. | by Andreas Heissenberger | Mac O’Clock | Medium](https://medium.com/macoclock/how-to-create-a-screencast-gif-75ef6931f43c) ([How to create a Screencast GIF | Andreas Heissenberger LABORATORY](https://medium.com/p/75ef6931f43c/responses/show))
    - ~ FFmpeg 加入, 还是标准些
    - 先录制标准的屏幕录像
    - 然后, 在 FFmpeg 的支持下抽帧加速变成 gif
    - 适用所有年代所有平台所有系统...

关键指令:

1. Create a palette image from the video:

    $ ffmpeg -y -i in.mov -vf fps=10,palettegen palette.png

2. Convert into a GIF using the palette

    $ ffmpeg -i in.mov -i palette.png -filter_complex “fps=10,paletteuse” out.gif




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




