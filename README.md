# BXMr
> BXM manager ~ a CLI tool for servicing code table of BXM under rIME-Squirrel
> 叕一个 CLI 工具为 rIME-Squirrel 维护 BXM 输入法码表而创立

![bxmr_v0.3.43_usage](https://ipic.zoomquiet.top/2023-03-06-bxmr_v0.3.43_usage.gif)



------
## Command-line options

```
a CLI tool for managment BXM's code with rIME-Squirrel

Usage: bxmr <COMMAND>

Commands:
  cfg    yaml|toml path/2/u/loc./AIM.yaml|toml ~ set rIME aim .yaml & BXMr usage .toml as ENV AT FIRST...
  env    check bind ENV setting, work with coomad:cfg
  init   default -> ./log/bxm_dama_loc.toml, config by command: cfg
  gen    re-generating .yaml -> ~/Library/Rime/[U BXM].yaml, , config by command: cfg
  renew  default -> ~/Library/Rime/[U BXM].yaml, , config by command: cfg
  seek   base code SEEK word is exist?
  find   base word FIND code is exist?
  upd    aaa 叒 <~ base code word UPGRADE the define in BXM
  dele   aaa 叒 ~> base code word DELET the define from BXM
  ahead  aaa 叒 => base code word UP the word define 1st in BXM
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

> BXMr Usage:

```
0: must setup .env for all Commands;
    $ bxmr cfg yaml path/2/u/local/bxm4zq2mac.dict.yaml
        ~ point u rIME-Squirrel usage .yaml
    $ bxmr cfg toml path/2/u/local/bxmr_loc_temp.toml
        ~ point u local temporary .toml, BXMr need this for cache data

> daily usage flow
1: seek the code is exist?
    $ bxmr seek aaa

2: if not exist, u can append it:
    $ bxmr upd aaa 叒

3: or find the word's code is exist? ~> find 字词
    or upd more code into temporary .toml

4: if enough now, must export to .yaml:
    $ bxmr gen

at last, always need usage rIME's re-deploy menu, 
    for load new code-table .yaml,
    so we can enjoy new BXM now ;-)
```

use --help can print this mini handbook



------
## Installation

### Cargo
If you already have a Rust environment set up, you can use the cargo install command:

> $ cargo install bxmr

Cargo will build the `bxmr` binary and place it in $HOME/.cargo.


### Manual installation from GitHub
Compiled binary versions of `bxmr` are uploaded to GitHub when a release is made. You can install `bxmr` manually by downloading a release, extracting it, and copying the binary to a directory in your `$PATH`, such as `/usr/local/bin`.

For more information, 

...TBD

### Homebrew

..TBD

## background
[Animated Ferris - JSFiddle](https://jsfiddle.net/Diggsey/3pdgh52r/embedded/result/)

## logging

- ...
- 230306 ZQ ++ahead ~ adjust word suggest order
- 230304 ZQ push github
- 230227 ZQ mod/clap/tracing/... define
- 230225 ZQ re-re-re-init.


### refer.


- [clap::_derive::_cookbook::git_derive - Rust](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
- [Building a CLI from scratch with Clapv3 | by Ukpai Ugochi | Medium](https://medium.com/javascript-in-plain-english/coding-wont-exist-in-5-years-this-is-why-6da748ba676c)
    - odd, can not cargo check
- [CN1581030A - 自由形码中文输入法3.0版 - Google Patents](https://patents.google.com/patent/CN1581030A/zh)
- ...
- [RIME | 中州韻輸入法引擎](https://rime.im/) ~ so great IME
- BXM [表形码](https://zh.wikipedia.org/wiki/Windows_95) ~ one great [中文输入法](https://zh.wikipedia.org/wiki/%E4%B8%AD%E6%96%87%E8%BE%93%E5%85%A5%E6%B3%95)
- but not tools for support BXM's code managment


### goal
> as Rustacean homework ...

as crate, can:

- easy install
- usage at local
- support rIME's input code table management:
    - seek ~ base code check word is defined?
    - find ~ base word check code is exist?
    - upd ~ define new code-word and upgrade BXM-code table
    - delet ~ base code+word remove the define date and upgrade BXM-code table
    - gen ~ re-generating BXM-code table as .yaml, so rIME-Squirrel can re-built and reload new BXM for usage.
    - ...


------
```
        _~~+~~_
    () /  > #  \ ()
      '_   ♢   _'
      | '--∽--' )

...act by ferris-actor v0.2.4 (built on 23.0303.201916)
```




