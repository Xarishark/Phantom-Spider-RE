# How the port works

Phantom Spider shipped in 2004 as an obfuscated J2ME MIDlet for Nokia
Series 40 phones (MIDP 2.0, CLDC 1.1, 128x128 screen). This port was made
by decompiling the JAR with [Vineflower](https://github.com/Vineflower/vineflower)
and rewriting every class in Rust, on top of small stand-ins for the phone
APIs the game used. Nothing Java runs at any point.

## Layout

```
src/
├─ main.rs        SDL3 front end: start-up, first-start setup screen, window,
│                 scaling, keyboard/gamepad, rumble, audio stream,
│                 fixed-step loop, headless test driver
├─ paths.rs       the portable game folder: save/, data/, logs/, settings.toml
├─ setup.rs       checks the player's JAR and unpacks it into data/
├─ settings.rs    settings.toml (written with comments on first start)
├─ log.rs         logs/phantom-spider.log, last three runs
├─ midp/          phone API stand-ins
│  ├─ gfx.rs      Graphics, Image, game.Sprite on a 128x128 ARGB framebuffer
│  ├─ font.rs     proportional pixel font (stands in for the S40 small font)
│  ├─ lcdui.rs    List / Form / Alert screens, commands, soft keys, key codes
│  ├─ rms.rs      RecordStore, saved as save/<name>.rms
│  ├─ audio.rs    MMAPI MIDI Player via rustysynth + a SoundFont
│  ├─ lang.rs     the game's string tables (lang.xx etc.)
│  └─ assets.rs   getResourceAsStream: data/ read into memory at start
└─ game/          the game itself, one Rust file per group of Java classes
   ├─ midlet.rs   PhantomSpider (MIDlet): menus, forms, commands, save format
   ├─ kcanvas.rs  k: menu canvas (eye list, combining, pilot/spider info, high scores)
   ├─ lcanvas.rs  L: the shooting game
   ├─ battle.rs   G + i: battle manager, spider pool, collisions
   ├─ ents.rs     F and subclasses: weapons, protectors, eggs, webs
   ├─ spider.rs   r, z (tarantula), l (pilot's ship)
   ├─ eyes.rs     J, v, j: eye collection, combining, pilot stats
   ├─ sfx.rs      s, M: explosions
   └─ tutorial.rs u: animated instruction pages
```

## Conventions

- **Names follow the obfuscated Java.** Every Java static `Cls.fld` is a
  field `Cls_fld` on `game::St` (or `game::Images` for images), and method
  doc comments name the Java method they port (`/// \`G.b(int, int)\``).
  This keeps the port checkable line by line against the decompiled source
  (`tools/decompile.sh` regenerates it from your own JAR).
- **Java exceptions are kept.** The original wrapped `paint()` in
  `try/catch` and relied on exceptions to abort a frame (null images, bad
  indices). Those code paths return `Res = Result<(), Jex>` and use `?`, so a
  frame stops exactly where Java's would have.
- **Java quirks are kept on purpose.** Shadowed fields (`q.j` and `f.j`
  separate from `B.j`, `a.a`/`a.h` fixed at 9), a never-assigned static
  (`f.c == 0`), precedence slips like `b + 1 % n`. Changing them changes the
  game.
- **Deliberate differences** from the phone are listed in the README.

## 60 fps without changing the pace

The phone ran all game logic inside `paint()`, which a timer thread called
every 150 ms (100 ms from pilot level 100). Everything moved in 3 to 15 px
jumps at about 6.7 fps.

The shooting game (`L`) now runs at a fixed 60 Hz. Each original frame (a
"beat") is split into `n` sub-steps (9 at 150 ms, 6 at 100 ms):

| Runs | What | Where |
| --- | --- | --- |
| first sub-step of each beat | everything the original counted in frames: AI decisions, fire cooldown, weapon charge, animation frames, missile acceleration, timers; damage from things that keep overlapping a target (lasers, protectors, burns, spider contact, web hold) | `L_beat`, `G_beat`, `Ent::beat`, `Z::think`, `Plane::beat`, `SFx::tick` |
| every sub-step | movement (each object moves `part(v, k, n)` of its per-frame velocity, which sums to exactly `v` per beat), input direction, hits that consume a projectile | `L_sub`, `G_sub`, `Ent::sub`, `Z::sub`, `Plane::sub` |
| every display frame | drawing, with no side effects | `L_draw`, `G_draw`, `Ent::draw` |

Notes:

- A projectile launched mid-beat is marked `fresh` and gets its first beat
  on the next sub-step, so it starts moving immediately.
- Random per-draw effects (spider leg wobble, charge sparkle) are rolled once
  per beat so they don't flicker 9x faster.
- Scripted motions (intro fly-in, mission-complete fly-out and banner) decide
  on the beat whether they move (`lc.mv`), so they stop on exactly the
  original pixel.
- The menu canvas (`k`) still repaints at the original rate; nothing there
  moves continuously.

Headless comparisons against the original-rate build (same scripts, 20 runs
each) gave the same movement per second, scores per weapon type and damage
taken within run-to-run noise.

## The player's JAR

None of the original files ship with the port. `setup.rs` accepts only the
known release (`Phantom_Spider_s40v2.jar`, 118,744 bytes, checked by
SHA-256), copies every resource out of it (PNGs, MIDI files, `lang.*`; no
`.class` files or manifest) into `data.new/`, then swaps it in as `data/`
with a `.prepared` marker (hash + layout version). `assets.rs` reads
`data/` into memory at start and serves the same paths the Java used
(`/frontpage.png`, `/sound/opening.mid`, ...).

The JAR can come from four places, checked in this order:

1. `--prepare --game <jar>`: RecompLauncher's setup step. No window; prints
   `progress <0..1> <text>` lines and exits 0, or with an error code.
2. `--game <jar>`: prepares on start if `data/` isn't ready.
3. A supported `.jar` next to the executable.
4. The first-start screen: drop the file on the window or pick it (SDL3's
   file dialog).

Exit codes (also in `recomplauncher.json`): 2 can't read the JAR, 3 not the
supported JAR, 4 the game folder isn't writable, 5 not set up and no JAR
given.

## RecompLauncher

`recomplauncher.json` (validated against the launcher's schema) asks for the
JAR as a named game input (`{input:jar}`), runs `--prepare` as the setup
step, maps `settings.toml` to the launcher's settings page, lists `save/`
for backups and `logs/` for crash reports, and explains the exit codes. The
game reads `RECOMPLAUNCHER_MODE` (Big Picture starts full screen) and keeps
SDL's background controller input off unless the player turns it on, since
the launcher's overlay uses the controller too.

## Saves

`save/Phoenix7.rms` holds a list of records written with Java's
`writeUTF`, in the exact order the original wrote them (see `ps_save` /
`ps_parse` in `midlet.rs`): mission-in-progress flag, pilot stats, equipped
eyes, the eye list, high scores, and, when a mission was paused, the whole
battle state (spiders, eggs, webs, shots, weapons).

## Testing

`main.rs` has a headless mode that runs a key script and saves screenshots.
`PHANTOM_SPIDER_HOME` moves the whole game folder (save, data, logs,
settings) so tests never touch a real one:

```bash
PHANTOM_SPIDER_HOME=/tmp/ps ./phantom-spider --headless --game game-files/Phantom_Spider_s40v2.jar \
    --script tests/scripts/smoke.txt --out shots
```

Script commands: `wait <ms>`, `press|release|tap <key>` (keys: `up`,
`down`, `left`, `right`, `fire`, `softl`, `softr`, `clear` or a MIDP key
code such as `53` for keypad 5), `shot <file.png>`, `pos <tag>` (ship
position), `stat <tag>` (state, score, life, spiders, level), `quit`.
Lines starting with `#` are ignored. `tools/run-tests.sh` runs everything in
`tests/scripts/` with your JAR from `game-files/` (or `PHANTOM_SPIDER_JAR`).
`cargo test` renders the first-start screen to `target/setup-screen-*.png`.

Things to know when writing scripts: a fresh save starts the main menu at
"New game"; digits typed on the game canvases also act as keypad directions
(2 = up, 4 = left, 6 = right, 8 = down), as on the phone.
