# mcat

Music Catalog（mcat）是一款音乐文件管理工具，旨在优雅地管理音乐文件及其元数据。

## 快速开始

### 从源码构建

```sh
git clone https://github.com/chimes-of-freedom/mcat.git
cd mcat
cargo build --release   # target at `target/release/mcat`
```

## 用法

```text
$ mcat --help
Music Cataloging Tool

Usage: mcat <COMMAND>

Commands:
  display  
  edit     
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ mcat display --help
Usage: mcat display --path <PATH>

Options:
      --path <PATH>  path of music file to be displayed
  -h, --help         Print help

$ mcat edit --help   
Usage: mcat edit [OPTIONS] --src <SRC>

Options:
      --src <SRC>        path of music file to be edited
      --title <TITLE>    new title
      --artist <ARTIST>  new artist
      --album <ALBUM>    new album
      --genre <GENRE>    new genre
      --dst <DST>        path of edited music file to be saved at
  -h, --help             Print help
```
