# haru

A tiny terminal cat companion. Type `haru` when the terminal feels too empty —
it says `meow`, shows a little ASCII cat, and gives you a small, kind tip.

```
$ haru
meow.

 /\_/\
( o.o )
 > ^ <

tip: drink water before debugging. the bug can wait, your kidneys cannot.
```

## Usage

```
haru            summon the cat
haru --pat      pat Haru (counts toward today's pats)
haru --feed     feed Haru (counts toward today's feeds)
haru --status   show mood and today's / total pats & feeds
haru --lang zh  force a language (auto-detected from your locale otherwise)
haru --version
haru --help
```

Haru remembers how many times you've patted and fed it today. It never guilt-trips
you, never decays, and never punishes you for being away.

## Languages

Haru speaks English, 中文, and 日本語. It picks your language from the system locale
automatically; override with `--lang en` / `--lang zh` / `--lang ja`, or the
`HARU_LANG` environment variable. Region and script tags are matched by their
primary subtag (`zh-CN`, `zh-TW` → `zh`). Each language lives in
`assets/lang/<code>/` and is written by hand (not machine-translated) —
contributions welcome.

## Fully offline

haru does not touch the network and collects no data. Everything lives on your machine:

- **State (the save file):** `$XDG_STATE_HOME/haru/state.json` (Linux, default
  `~/.local/state/haru/`); on macOS it falls back to
  `~/Library/Application Support/cat.haru.haru/state.json`. It's plain JSON — read it,
  edit it, delete it to start fresh.
- **Tips & flavor text:** baked into the binary from `assets/lang/<code>/`
  (`tips.txt`, plus `pat.txt` / `feed.txt` for pat & feed reactions). Edit and
  rebuild to make Haru say your own things.

## Build

```
cargo build --release
./target/release/haru
```

## License

MIT — see [LICENSE](LICENSE).
