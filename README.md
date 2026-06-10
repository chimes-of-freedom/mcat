# mcat

mcat (Music CATalog) is a music cataloging tool aimed at providing a graceful way to manage music files along with their metadata.

> [!Warning]
>
> mcat is under active development and may have lots of bugs and missing features.

## Quick Start

Build mcat from source.

```sh
git clone https://github.com/chimes-of-freedom/mcat.git && cd mcat
cargo build -r # target under `target/release`
```

Make a directory as your repository, and place your music files under `media/`.

```sh
mkdir my_music_repo && cd my_music_repo
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
mcat is a music cataloging tool aimed at providing a
graceful way to manage music files along with their metadata.

Usage: mcat <COMMAND>

Commands:
  init    Initialize a repository
  list    List selected tracks in the repository
  add     Add tracks into the repository
  remove  Remove selected tracks from the repository
  update  Update fields of selected tracks in the repository

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### Init

`mcat init` creates structures needed under `.mcat/`, scans supported track files under `media/` and adds them to the repository.

### List

`mcat list` / `mcat ls` shows metadata of selected tracks in the repository. The title, artist and album of all tracks are listed by default.

```text
$ mcat ls
┌────┬───────────────────┬───────────┬───────────────────────────┐
│ id │ title             │ artist    │ album                     │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 1  │ Chimes Of Freedom │ Bob Dylan │ Another Side Of Bob Dylan │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 2  │ All We Cannot Say │ Ally Kerr │ Upgrade Me                │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 3  │ Toledo            │ Ally Kerr │ Upgrade Me                │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 4  │ Upgrade Me        │ Ally Kerr │ Upgrade Me                │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 5  │ Whatever Happens  │ Ally Kerr │ Viva Melodia              │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 6  │ Saeglopur         │ Sigur Rós │ We Play Endlessly         │
└────┴───────────────────┴───────────┴───────────────────────────┘

$ mcat ls \
  --column title --column album --column artist \
  --artist="Ally Kerr" --artist="Bob Dylan" --album="Upgrade Me"
┌───────────────────┬──────────────┬───────────┐
│ title             │ album        │ artist    │
├───────────────────┼──────────────┼───────────┤
│ All We Cannot Say │ Upgrade Me   │ Ally Kerr │
├───────────────────┼──────────────┼───────────┤
│ Toledo            │ Upgrade Me   │ Ally Kerr │
├───────────────────┼──────────────┼───────────┤
│ Upgrade Me        │ Upgrade Me   │ Ally Kerr │
├───────────────────┼──────────────┼───────────┤
│ Whatever Happens  │ Viva Melodia │ Ally Kerr │
└───────────────────┴──────────────┴───────────┘

$ mcat ls -j --title Saeglopur
[
  {
    "id": 6,
    "title": "Saeglopur",
    "artist": "Sigur Rós",
    "album": "We Play Endlessly"
  }
]
```

### Add

`mcat add` adds tracks to the repository. Multiple tracks can be added at a time, and the option `-r` / `--recursive` allows you to recursively add all tracks under a directory.

> [!Warning]
>
> mcat has not supported automatic track deduplication yet.

```text
$ ls ../new_tracks/
'Shiver - Lucy Rose.flac'  'Tempest - Bob Dylan.flac'

$ mcat add ../new_tracks
Error: Path "../new_tracks" is a directory. Option `-r` needed

$ mcat add ../new_tracks/"Tempest - Bob Dylan.flac"
1 tracks added, 0 paths omitted.

$ mcat ls --title Tempest
┌────┬─────────┬───────────┬─────────┐
│ id │ title   │ artist    │ album   │
├────┼─────────┼───────────┼─────────┤
│ 7  │ Tempest │ Bob Dylan │ Tempest │
└────┴─────────┴───────────┴─────────┘

$ mcat add -r ../new_tracks
2 tracks added, 0 paths omitted.

$ mcat ls --title Tempest --title Shiver
┌────┬─────────┬───────────┬────────────────┐
│ id │ title   │ artist    │ album          │
├────┼─────────┼───────────┼────────────────┤
│ 7  │ Tempest │ Bob Dylan │ Tempest        │
├────┼─────────┼───────────┼────────────────┤
│ 8  │ Shiver  │ Lucy Rose │ Like I Used To │
├────┼─────────┼───────────┼────────────────┤
│ 9  │ Tempest │ Bob Dylan │ Tempest        │
└────┴─────────┴───────────┴────────────────┘

$ ls media         
'All We Cannot Say - Ally Kerr.flac'  'Shiver - Lucy Rose.flac'   'Upgrade Me - Ally Kerr.flac'
'Chimes Of Freedom - Bob Dylan.flac'  'Tempest - Bob Dylan.flac'  'Whatever Happens - .mp3'
'Saeglopur - Sigur Rós.flac'          'Toledo - Ally Kerr.flac'
```

### Remove

`mcat remove` / `mcat rm` removes selected tracks from the repository. No tracks are removed by default.

> [!Warning]
>
> `mcat remove` is not designed to delete BLOB rows in the database and track files under `media/` automatically.

```text
$ mcat ls --title Tempest
┌────┬─────────┬───────────┬─────────┐
│ id │ title   │ artist    │ album   │
├────┼─────────┼───────────┼─────────┤
│ 7  │ Tempest │ Bob Dylan │ Tempest │
├────┼─────────┼───────────┼─────────┤
│ 9  │ Tempest │ Bob Dylan │ Tempest │
└────┴─────────┴───────────┴─────────┘

$ mcat rm
0 tracks removed.

$ mcat rm --id 9
1 tracks removed.

$ mcat ls --title Tempest
┌────┬─────────┬───────────┬─────────┐
│ id │ title   │ artist    │ album   │
├────┼─────────┼───────────┼─────────┤
│ 7  │ Tempest │ Bob Dylan │ Tempest │
└────┴─────────┴───────────┴─────────┘

$ ls media         
'All We Cannot Say - Ally Kerr.flac'  'Shiver - Lucy Rose.flac'   'Upgrade Me - Ally Kerr.flac'
'Chimes Of Freedom - Bob Dylan.flac'  'Tempest - Bob Dylan.flac'  'Whatever Happens - .mp3'
'Saeglopur - Sigur Rós.flac'          'Toledo - Ally Kerr.flac'
```

### Update

`mcat update` updates fields of selected tracks. `--set` / `--clear` options allows you to set / clear value of a field. No tracks are updated by default.

```text
$ mcat update --artist="Bob Dylan" --set artist="bob dylan"
2 tracks updated.

$ mcat ls --artist="Bob Dylan"
┌────┬───────┬────────┬───────┐
│ id │ title │ artist │ album │
└────┴───────┴────────┴───────┘

$ mcat ls --artist="bob dylan"
┌────┬───────────────────┬───────────┬───────────────────────────┐
│ id │ title             │ artist    │ album                     │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 1  │ Chimes Of Freedom │ bob dylan │ Another Side Of Bob Dylan │
├────┼───────────────────┼───────────┼───────────────────────────┤
│ 7  │ Tempest           │ bob dylan │ Tempest                   │
└────┴───────────────────┴───────────┴───────────────────────────┘
```
