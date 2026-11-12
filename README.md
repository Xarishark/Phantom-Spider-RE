# Phantom Spider: PC port

A native Windows/Linux port of **Phantom Spider**, a 2004 Nokia Series 40
J2ME shooter. You pilot the Phantom Spider against the Evil Tarantula's
army, collect the eyes of the spiders you defeat, and combine them into
stronger eyes that give your ship new weapons.

The original game was decompiled and every class rewritten in Rust, so no
Java runtime or emulator is involved. SDL3 handles the window, keyboard,
gamepads, rumble and audio, and the shooting game runs at a smooth 60 fps at
the original pace.

| | | | |
| :-: | :-: | :-: | :-: |
| ![Battle](docs/images/battle.png) | ![Protector eyes](docs/images/protectors.png) | ![Mission complete](docs/images/mission-complete.png) | ![Eye reward](docs/images/eye-reward.png) |

## Playing

Windows: put `PhantomSpider.exe` in a folder of its own and run it. It's a
single file (SDL3 is linked in), and saves go to a `save` folder next to it,
so the whole folder is portable. The game won't start from a folder it can't
write to, such as Program Files.

Music needs a General MIDI SoundFont: put any `.sf2` file next to the exe
(named `soundfont.sf2` or anything else). [GeneralUser
GS](https://github.com/mrbumpy409/GeneralUser-GS) sounds good. Without one
the game runs silently.

Command line options: `--scale N` (window size, default 5x), `--fullscreen`,
`--soundfont file.sf2`, `--mute`, `--no-rumble`, `--lang tl-PH` (Filipino
text; English is the default).

## Controls

| Action | Keyboard | Gamepad |
| --- | --- | --- |
| Move / navigate | Arrows, WASD, keypad 2/4/6/8 | D-pad, left stick, LB/RB |
| Fire / select (hold to charge eye weapons) | Enter, Space, keypad 5 | A, X, right trigger |
| Menu / pause / back | Esc | B, Start |
| Left soft key | Tab, F1 | Y, Back, left trigger |
| Clear a digit in forms | Backspace | |
| Phone keypad digits | 0-9 | |
| Fullscreen | F11, Alt+Enter | |
| Mute / volume | M, +/- | |

Gamepads can be connected or removed while playing, several at once.

## Differences from the phone version

- **60 fps.** The phone updated the game every 150 ms. Movement now runs at
  60 Hz while everything the original counted in frames (AI, cooldowns,
  charge, damage) keeps its original timing, so the game plays at the same
  pace. Details in [docs/PORTING.md](docs/PORTING.md).
- **No SMS challenges.** "Challenge player" sent SMS messages between phones;
  it now only shows a notice.
- **Score is kept between sessions.** The original reset it on every launch
  unless a mission was paused. Only "New game" resets it now.
- **Controls:** releasing one direction while another is held keeps you
  moving (the phone only had one direction at a time). Taking damage rumbles
  the gamepad.

Everything else (missions, eye collecting, combining and equipping, pilot
and spider stats, high scores, the animated instructions, pause/continue,
the original cheat codes) works as on the phone.

## Building

A recent stable Rust toolchain and SDL3. On Linux, install the SDL3 development package and:

```bash
cargo build --release
```

The Windows `.exe` is cross-compiled from Linux with MinGW (`mingw64-gcc`,
`cmake`, `ninja`); SDL3 is built from source and linked statically:

```bash
rustup target add x86_64-pc-windows-gnu
tools/build-windows.sh        # SOUNDFONT=/path/file.sf2 to bundle music
```

`tools/run-tests.sh` runs the headless test scripts in `tests/scripts/`,
which play through the game and save screenshots. See
[docs/PORTING.md](docs/PORTING.md) for how the port is organized.

## Credits and copyright

Phantom Spider (2004) lists its vendor as **ANDREW-LVIV / Nokia**. All rights to the original game, its name, artwork, music and text
belong to their respective owners. The files in `assets/` come from the
original game and are included for preservation; they are not covered by
this project's license.

This project is not affiliated with or endorsed by Nokia or ANDREW-LVIV.
The port's source code is released under the [MIT License](LICENSE).
