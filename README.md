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

mcat 采用“子命令 + 选项”格式的命令风格。

### 子命令

```text
$ mcat --help
Music Cataloging Tool

Usage: mcat.exe <COMMAND>

Commands:
  display  display the music metadata
  edit     write the music metadata
  init     init a repository
  check    check if files in `media/` are tracked by the database, or if files in the database exist in `media/`
  remove   remove a track's metadata from the repository, along with the file if specified
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 查看文件元数据

```text
$ mcat display --help
display the music metadata

Usage: mcat display <PATH>

Arguments:
  <PATH>  path of music file to display

Options:
  -h, --help  Print help
```

### 编辑文件元数据

```text
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
```

### 初始化仓库

```text
$ mcat init --help
init a repository

Usage: mcat init

Options:
  -h, --help  Print help
```

### 检查仓库数据库同步情况

```text
$ mcat check --help
check if files in `media/` are tracked by the database, or if files in the database exist in `media/`

Usage: mcat.exe check [OPTIONS]

Options:
  -t, --track              only check if files in `media/` are tracked by the database
  -e, --exist              only check if files in the database exist in `media/`
  -r, --repair             repair according to the check results
  -s, --save-to <SAVE_TO>  save results in toml
  -h, --help               Print help
```

### 删除曲目

```text
$ mcat remove --help
remove a track's metadata from the repository, along with the file if specified

Usage: mcat.exe remove [OPTIONS] <TRACK>

Arguments:
  <TRACK>  file hash or the track title

Options:
  -r, --remove-file  remove the file
  -h, --help         Print help
```

## Todo List

- [x] init（重构）-> 初始化数据库，提取所有元数据，计算哈希时去除文件标签（但不写回文件）以保证哈希稳定。
- [x] check -> 检查数据库和 `media/` 下文件数据是否匹配。
- [ ] import -> 导入指定目录下歌曲到仓库。默认是拷贝文件到仓库、去除文件元数据、插入文件元数据信息到数据库，可指定为“移动文件”。
- [ ] edit（重构）-> 更改指定曲目元数据信息。
- [ ] display（重构）-> 指定 title 查询曲目信息。可选参数 filter 实现查询过滤。
- [x] remove -> 从仓库删除指定曲目。可选参数 filter 实现批量删除。
- [ ] export -> 导出曲目到指定文件夹。主要步骤是将元数据写回文件，然后导出。可选参数 filter 实现批量导出，with-list 实现歌单导出。
