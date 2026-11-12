# Phantom Spider (PC port)

Native Windows/Linux port of **Phantom Spider**, a 2004 Nokia Series 40
(MIDP 2.0, 128x128) J2ME game. The original JAR was decompiled and the game
logic rewritten in Rust; SDL3 provides the window, keyboard, gamepads,
rumble and audio. No Java runtime is involved.

## Running

`dist/PhantomSpider/` holds the Windows build:

- `PhantomSpider.exe`: the game (SDL3 linked statically, no DLLs needed)
- `soundfont.sf2`: General MIDI bank for the music ([GeneralUser GS](https://github.com/mrbumpy409/GeneralUser-GS), see `SOUNDFONT-LICENSE.txt`).
  Any `.sf2` next to the exe works; without one the game runs silently.

Options: `--scale N` (window size, default 5), `--fullscreen`,
`--soundfont file.sf2`, `--mute`, `--no-rumble`, `--lang tl-PH`.

Saves go to a `save` folder next to the exe (`save\Phoenix7.rms`), so the
whole folder is portable. Saves from older builds in `%APPDATA%\PhantomSpider`
are copied over on first launch. If the exe's folder isn't writable the game
falls back to `%APPDATA%\PhantomSpider` (`~/.local/share/PhantomSpider` on
Linux); `PHANTOM_SPIDER_DATA` overrides the location.

## Controls

| Action | Keyboard | Gamepad |
| --- | --- | --- |
| Move / navigate | Arrows, WASD, keypad 2/4/6/8 | D-pad, left stick, LB/RB (left/right) |
| Fire / select (hold to charge eye weapons) | Enter, Space, keypad 5 | A, X, right trigger |
| Menu / pause / back (right soft key) | Esc | B, Start |
| Left soft key | Tab, F1 | Y, Back, left trigger |
| Clear digit | Backspace | |
| Phone keypad digits (cheat codes) | 0-9 | |
| Fullscreen | F11, Alt+Enter | |
| Mute / volume | M, +/- | |

Gamepads are hot-pluggable; any number may be connected at once.

## 60 fps

The phone updated the game once per repaint, every 150 ms (100 ms from
pilot level 100), so everything moved in 3 to 15 px jumps at ~6.7 fps. The
shooting game here runs at a fixed 60 Hz without changing its pace:

- **Every 1/60 s:** movement (each object covers exactly its original
  per-frame distance spread over the 9 or 6 sub-steps), your direction
  input, and hits that use up a projectile (shots, missiles, eggs).
- **Once per original frame:** everything the original counted in frames:
  AI decisions, fire cooldown, weapon charge, animation frames, missile
  acceleration, and damage from things that stay on a target (lasers,
  protectors, burns, a spider touching you, a web holding you).

Scripted comparisons against the 6.7 fps build give the same movement per
second, mission scores and damage taken (within run-to-run noise). The menu
screens still update at the original rate since nothing moves there.

## What changed from the phone version

- The SMS "Challenge player" mode needs a phone network, so it only shows
  a notice. Everything else (missions, eye collecting/combining/equipping,
  pilot & spider stats, high scores, instructions, saving/continuing,
  the original cheat codes) behaves as on the phone.
- Backlight control (`DeviceControl.setLights`) is ignored; taking damage
  rumbles the gamepad instead.

## Layout

- `src/gfx.rs`, `font.rs`, `lcdui.rs`, `rms.rs`, `audio.rs`, `lang.rs`:
  re-implementations of the MIDP APIs the game used (Graphics/Image/Sprite,
  List/Form/Alert, RecordStore, MMAPI MIDI player, Nokia string table).
- `src/game/`: the game classes, ported one-to-one. Java statics are named
  `Class_field` so code can be compared with `reference/java/`.
- `src/main.rs`: SDL3 front end, plus a headless test driver
  (`--headless --script file --out dir` dumps PNG screenshots).
- `assets/`: resources from the JAR (embedded into the exe at build time).
- `reference/java/`: Vineflower decompilation of the original classes
  (`U_` prefix marks upper-case class names, since Windows file names
  are case-insensitive).
- `original/`: the original JAR/JAD.

## Building

Linux (needs SDL3 development files):

```bash
cargo build --release
```

Windows `.exe`, cross-compiled from Linux with MinGW (`mingw64-gcc`,
`cmake`, `ninja`; SDL3 is built from source and linked statically):

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```
