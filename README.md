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
  display  display the music metadata
  edit     write the music metadata
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

$ mcat display --help
display the music metadata

Usage: mcat display <PATH>

Arguments:
  <PATH>  path of music file to display

Options:
  -h, --help  Print help

$ mcat edit --help   
write the music metadata

Usage: mcat edit [OPTIONS] <SRC>

Arguments:
  <SRC>  path of music file to be edited

Options:
      --title <TITLE>    new title
      --artist <ARTIST>  new artist
      --album <ALBUM>    new album
      --genre <GENRE>    new genre
  -o, --output <DST>     path of edited music file to be saved at (default `src`)
  -h, --help             Print help
```
