# mcat

mcat (Music CATalog) is a music cataloging tool aimed at providing a graceful way to manage music files along with their metadata.

> [!Warning]
>
> mcat is under active development and may have lots of bugs and missing features. For developers, see [Development Guide](docs/DEVELOPMENT.md).

## Quick Start

Build mcat from source.

```sh
git clone https://github.com/chimes-of-freedom/mcat.git && cd mcat
cargo build -r # target under `target/release`
```

Make a directory as your repository, and place your music files under `media/`.

```sh
mkdir my_music_repo && cd my_musci_repo
cp -r /path/to/your/music/files/directory media
```

Initialize the repository. mcat will scan music files under `media/` and create a directory named `.mcat/` to store metadata.

```sh
mcat init
```

**That's that!**

## Usage

The CLI of mcat consists of subcommands and options.

```text
$ mcat --help
mcat is a music cataloging tool aimed at providing a graceful way
to manage music files along with their metadata.

Usage: mcat <COMMAND>

Commands:
  display  Displays music metadata stored in the repository. Display all tracks if no filter specified
  edit     Edits metadata of a track. Does nothing if no filter specified
  init     Initializes a repository from files under `media/`
  check    Checks consistency between files under `media/` and repository records
  remove   Removes tracks from the repository, optionally removing files. Does nothing if no filter specified
  import   Imports music files from a directory
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Init

`mcat init` initializes a repository.

### Display

`mcat display` shows metadata of selected tracks in the repository. If no filter is specified, it will display metadata of all tracks.

The usage of filters is as follows.

```text
$ mcat display --artist 陳奕迅
┌─────────────────────┬────────┬──────────────┬──────────────┬──────┬──────────────┬─────────────┬───────────────────┬──────────┬──────────┐
│ title               │ artist │ album        │ album_artist │ date │ track_number │ disc_number │ genre             │ composer │ lyricist │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 黃金時代            │ 陳奕迅 │ 我的快樂時代 │              │      │ 4            │ 1           │ 粵語流行音樂      │ 柳重言   │ 林夕     │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 反高潮              │ 陳奕迅 │ 我的快樂時代 │              │      │ 10           │ 1           │ 粵語流行音樂      │ 陳奕迅   │ 黃偉文   │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 垃圾 (Live Version) │ 陳奕迅 │ 幸福         │              │      │ 4            │ 2           │ 廣東歌/香港流行樂 │ 陳輝陽   │ 黃偉文   │
└─────────────────────┴────────┴──────────────┴──────────────┴──────┴──────────────┴─────────────┴───────────────────┴──────────┴──────────┘
$ mcat display --artist 陳奕迅 --artist=張國榮
┌─────────────────────┬────────┬──────────────┬──────────────┬──────┬──────────────┬─────────────┬───────────────────┬──────────┬──────────┐
│ title               │ artist │ album        │ album_artist │ date │ track_number │ disc_number │ genre             │ composer │ lyricist │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 黃金時代            │ 陳奕迅 │ 我的快樂時代 │              │      │ 4            │ 1           │ 粵語流行音樂      │ 柳重言   │ 林夕     │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 玻璃之情            │ 張國榮 │ 一切隨風     │              │      │ 2            │ 1           │ Pop               │ 張國榮   │ 林夕     │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 反高潮              │ 陳奕迅 │ 我的快樂時代 │              │      │ 10           │ 1           │ 粵語流行音樂      │ 陳奕迅   │ 黃偉文   │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 垃圾 (Live Version) │ 陳奕迅 │ 幸福         │              │      │ 4            │ 2           │ 廣東歌/香港流行樂 │ 陳輝陽   │ 黃偉文   │
├─────────────────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼───────────────────┼──────────┼──────────┤
│ 春夏秋冬            │ 張國榮 │ 陪你倒數     │              │      │ 3            │ 1           │ 粵語流行音樂      │ 葉良俊   │ 林振強   │
└─────────────────────┴────────┴──────────────┴──────────────┴──────┴──────────────┴─────────────┴───────────────────┴──────────┴──────────┘
$ mcat display --artist 陳奕迅 --artist 張國榮 --album 我的快樂時代
┌──────────┬────────┬──────────────┬──────────────┬──────┬──────────────┬─────────────┬──────────────┬──────────┬──────────┐
│ title    │ artist │ album        │ album_artist │ date │ track_number │ disc_number │ genre        │ composer │ lyricist │
├──────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼──────────────┼──────────┼──────────┤
│ 黃金時代 │ 陳奕迅 │ 我的快樂時代 │              │      │ 4            │ 1           │ 粵語流行音樂 │ 柳重言   │ 林夕     │
├──────────┼────────┼──────────────┼──────────────┼──────┼──────────────┼─────────────┼──────────────┼──────────┼──────────┤
│ 反高潮   │ 陳奕迅 │ 我的快樂時代 │              │      │ 10           │ 1           │ 粵語流行音樂 │ 陳奕迅   │ 黃偉文   │
└──────────┴────────┴──────────────┴──────────────┴──────┴──────────────┴─────────────┴──────────────┴──────────┴──────────┘
```

### Remove

`mcat remove` removes metadata of selected tracks from the repository, alternatively along with music files. If no filter is specified, it won't remove anything.

### Edit

`mcat edit` updates metadata of the track specified by title or BLAKE3 hash. If no field of metadata is provided, it won't update anything.

### Import

`mcat import` imports music files into the repository. Metadata will be recorded, and if `-m / --move` is specified, files will be moved into `media/` instead of copying them.

### Check

`mcat check` checks the consistency between files under `media/` and repository records using BLACK3 hash.

- If `-t / --track` is specified, mcat checks only whether files under `media/` are tracked;
- If `-e / --exist` is specified, mcat checks only whether tracked files still exist under `media/`.

By default, mcat checks both of them.

You can also specify `-r / --repair` to repairs repository state according to check results and `-s / --save-path <path>` to save check results as TOML.
