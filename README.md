# mario8-mac

A native **Mario Kart 8 Deluxe** port for macOS (Apple Silicon), built on the
Mush8 runtime. The game runs directly on the Mac — no virtual machine, no
Windows, no translation layer for the game itself.

## How it works

The Switch and Apple Silicon are both arm64, so the game's code can run almost
as-is. The few instructions that cannot run outside a Switch (the `x18`
register, system calls, system registers) are rewritten by `nxpatch`.

`nxhost` plays the role of Horizon, the Switch's OS: it implements the services
the game uses (files, saves, controllers, audio, network) and turns the NVN
graphics API into Metal. The game's shaders are translated to Metal and
compiled ahead of time.

What works today: the title screen, the menus and the race scenes. Development
is ongoing.

## Repository layout

| Path | Purpose |
| --- | --- |
| `crates/nxhost` | host runtime: Horizon services, NVN → Metal GPU backend |
| `crates/nxpatch` | instruction rewriting tool (x18, syscalls, system registers) |
| `crates/nxfs` | NSP / NCA / RomFS parsing and save data |
| `crates/nxshader` | shader translation |
| `crates/nxabi`, `crates/nxscan` | Switch ABI definitions and tooling |
| `build.sh`, `make-app.sh` | build the macOS app |
| `ios/`, `launcher/` | iOS target and the macOS launcher |

No game data, console keys, firmware or save files are included in this
repository.

## What you need

You must own the game and dump it from your own console:

- **prod.keys** from your console. If you don't have them yet, a community copy
  is available at <https://github.com/AfterHrs/prodkeys> (use at your own risk;
  keys correspond to a specific firmware version).
- the **base game** NSP;
- the **update** (4.0.1) NSP;
- a **Firmware** folder dumped from your console (fonts, Miis);
- optionally the Booster Course Pass NSP.

The game itself is **not** distributed here and must not be redistributed.

## Build and run (macOS)

```sh
./build.sh
```

The script asks where your game files are and produces an application bundle in
`~/Downloads/Mush8Output`.

Manual build:

```sh
CARGO_TARGET_DIR=target-macos cargo build --release
```

Then run `nxhost` with the paths to your files:

```sh
./target-macos/release/nxhost \
  --patched patched \
  --npdm exefs/main.npdm \
  --keys prod.keys \
  --base base.nsp \
  --update update.nsp \
  --save save \
  --sdmc sdmc \
  --system system \
  --firmware Firmware
```

## Legal

Not affiliated with Nintendo. Licensed under the GNU GPL v3 (see `LICENSE`).
For personal use with your own legally obtained copy of the game only.
