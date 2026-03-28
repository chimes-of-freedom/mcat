# mcat

Music Catalog（mcat）是一款音乐文件管理工具，旨在优雅地管理音乐文件及其元数据。

## 项目特色

- 元数据与文件分离，管理灵活；
- 采用 TOML 记录元数据，可读性强，且方便日后使用 Git 进行版本控制；
- 命令行工具，接口完备，方便批处理与日后的图形化前端开发。

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
  init     init a repository
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
      --title <TITLE>                new title
      --artist <ARTIST>              new artist
      --album <ALBUM>                new album
      --album-artist <ALBUM_ARTIST>  new album artist
      --genre <GENRE>                new genre
  -o, --output <DST>                 path of edited music file to be saved at (default `src`)
  -h, --help                         Print help

$ mcat init --help
init a repository

Usage: mcat init

Options:
  -h, --help  Print help
```

## Todo List

- [x] init（重构）-> 初始化数据库，提取所有元数据，去除文件标签以保证哈希稳定。
- [ ] check -> 检查数据库和 `media/` 下文件数据是否匹配。
- [ ] import -> 导入指定目录下歌曲到仓库。默认是拷贝文件到仓库、去除文件元数据、插入文件元数据信息到数据库，可指定为“移动文件”。
- [ ] edit（重构）-> 更改指定曲目元数据信息。
- [ ] display（重构）-> 指定 title 查询曲目信息。可选参数 filter 实现查询过滤。
- [ ] remove -> 从仓库删除指定曲目。可选参数 filter 实现批量删除。
- [ ] export -> 导出曲目到指定文件夹。主要步骤是将元数据写回文件，然后导出。可选参数 filter 实现批量导出，with-list 实现歌单导出。
