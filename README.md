# BXMr
> BXM manager ~ a CLI tool for servicing code table of BXM under rIME-Squirrel
> 叕一个 CLI 工具为 rIME-Squirrel 维护 BXM 输入法码表而创立

## background

- [RIME | 中州韻輸入法引擎](https://rime.im/) ~ so great IME
- BXM [表形码](https://zh.wikipedia.org/wiki/Windows_95) ~ one great [中文输入法](https://zh.wikipedia.org/wiki/%E4%B8%AD%E6%96%87%E8%BE%93%E5%85%A5%E6%B3%95)
- but not tools for support BXM's code managment


## goal
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
## Command-line options

```
a CLI tool for managment BXM's code with rIME-Squirrel

Usage: bxmr <COMMAND>

Commands:
  echo   print all BXM code-words
  gen    gen. rIME .yaml...
  upd    aaa 叒 <~ code word
  dele   aaa 叒 <~ code word
  init   path/2/[aim BXMr manag.].toml
  seek   base code SEEK word is there?
  find   base word FIND code is there?
  renew  path/2/[res BXM].yaml
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```



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
- 230304 ZQ push github
- 230227 ZQ mod/clap/tracing/... define
- 230225 ZQ re-re-re-init.


### refer.


- [clap::_derive::_cookbook::git_derive - Rust](https://docs.rs/clap/latest/clap/_derive/_cookbook/git_derive/index.html)
- [Building a CLI from scratch with Clapv3 | by Ukpai Ugochi | Medium](https://medium.com/javascript-in-plain-english/coding-wont-exist-in-5-years-this-is-why-6da748ba676c)
    - odd, can not cargo check
- [CN1581030A - 自由形码中文输入法3.0版 - Google Patents](https://patents.google.com/patent/CN1581030A/zh)
- ...


------
```
        _~~+~~_
    () /  > #  \ ()
      '_   ♢   _'
      | '--∽--' )

...act by ferris-actor v0.2.4 (built on 23.0303.201916)
```




