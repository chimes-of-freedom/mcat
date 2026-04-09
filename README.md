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
  display  Displays music metadata stored in the repository
  edit     Edits metadata for a music file
  init     Initializes a repository from files under `media/`
  check    Checks consistency between files under `media/` and repository records
  remove   Removes tracks from the repository, optionally removing files
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 初始化仓库

```text
$ mcat init --help
Initializes a repository from files under `media/`

Usage: mcat.exe init

Options:
  -h, --help  Print help
```

### 检查仓库数据库同步情况

```text
$ mcat check --help
Checks consistency between files under `media/` and repository records

Usage: mcat.exe check [OPTIONS]

Options:
  -t, --track              Checks only whether files under `media/` are tracked
  -e, --exist              Checks only whether tracked files still exist under `media/`
  -r, --repair             Repairs repository state according to check results
  -s, --save-to <SAVE_TO>  Saves check results as TOML
  -h, --help               Print help
```

### 删除曲目

```text
$ mcat remove --help
Usage: mcat.exe remove [OPTIONS] <--title <TITLES>|--artist <ARTISTS>|--album <ALBUMS>|--album-artist <ALBUM_ARTISTS>|--genre <GENRES>|--hash <HASHES>>

Options:
      --title <TITLES>                Track title filter (repeatable)
      --artist <ARTISTS>              Track artist filter (repeatable)
      --album <ALBUMS>                Album title filter (repeatable)
      --album-artist <ALBUM_ARTISTS>  Album artist filter (repeatable)
      --genre <GENRES>                Genre filter (repeatable)
      --hash <HASHES>                 File hash filter (repeatable)
  -r, --remove-file                   Removes the media file as well
  -h, --help                          Print help
```

### 查看文件元数据

```text
$ mcat display --help
Displays music metadata stored in the repository

Usage: mcat.exe display <--title <TITLES>|--artist <ARTISTS>|--album <ALBUMS>|--album-artist <ALBUM_ARTISTS>|--genre <GENRES>|--hash <HASHES>>

Options:
      --title <TITLES>                Track title filter (repeatable)
      --artist <ARTISTS>              Track artist filter (repeatable)
      --album <ALBUMS>                Album title filter (repeatable)
      --album-artist <ALBUM_ARTISTS>  Album artist filter (repeatable)
      --genre <GENRES>                Genre filter (repeatable)
      --hash <HASHES>                 File hash filter (repeatable)
  -h, --help                          Print help
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

## Todo List

- [x] init（重构）-> 初始化数据库，提取所有元数据，计算哈希时去除文件标签（但不写回文件）以保证哈希稳定。
- [x] check -> 检查数据库和 `media/` 下文件数据是否匹配。
- [x] import -> 导入指定目录下歌曲到仓库。默认是拷贝文件到仓库、插入文件元数据信息到数据库，可指定为“移动文件”。
- [x] edit（重构）-> 更改指定曲目元数据信息。
- [x] display（重构）-> 指定 title 查询曲目信息。可选参数 filter 实现查询过滤。
- [x] remove -> 从仓库删除指定曲目。可选参数 filter 实现批量删除。
- [ ] export -> 导出曲目到指定文件夹。主要步骤是将元数据写回文件，然后导出。可选参数 filter 实现批量导出，with-list 实现歌单导出。
