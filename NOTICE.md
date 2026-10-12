# NOTICE

**Discord Quest Completer – unofficial Linux port**

This repository is a fork (derivative work) of
[markterence/discord-quest-completer](https://github.com/markterence/discord-quest-completer),
Copyright (c) 2025 Mark Terence Tiglao, distributed under the MIT License.

## Modifications made in this fork (2026, by Dav (d4ruerk1))

- `src-linux/`: rewritten Linux dummy game runner – freestanding `no_std` static ELF (~1 KB), no GTK dependency.
- `src-tauri/src/lib.rs`: Linux support (per-user games directory under XDG data dir, runner lookup for distro
  packages, tracking/stopping of spawned runners without `pkill`, rejection of unsafe paths).
- `src-tauri/src/main.rs`: WebKitGTK workaround for NVIDIA/Wayland.
- `src-tauri/tauri.linux.conf.json`, `scripts/sync-runner.mjs`, `src/components/GameExecutables.vue`: Linux build/UI tweaks.
- `packaging/`: Arch Linux / CachyOS PKGBUILD, desktop entry; `.github/workflows/build-linux.yaml`: CI for the pacman package.
- README, LICENSE, CREDITS: attribution and Linux documentation.

All other code, design, and the application idea belong to the original author.
The full MIT license text (with all copyright notices) is in [LICENSE](LICENSE) and must be kept in every copy or substantial portion.

## Trademarks

"Discord" is a trademark of Discord Inc. It is used only to describe what this software interoperates with.
This project is not affiliated with, endorsed by, or sponsored by Discord Inc. Game names and identifiers shown in the app come
from Discord's public "detectable applications" list and belong to their respective owners.

## No warranty / use at your own risk

The software is provided "AS IS", without warranty of any kind (see LICENSE). Using it may be against Discord's Terms of Service
or Quest rules and could lead to lost rewards or account actions. You are responsible for how you use it.

## Privacy

The app does not collect or send personal data to the authors. It only downloads the public game list from
`discord.com` / `markterence.github.io` and talks to the local Discord client through its local RPC socket.

## Third-party components

The application is built with open-source components distributed under their own licenses (mostly MIT and/or Apache-2.0),
including Tauri, Vue, Vite, Tailwind CSS, `discord-sdk` (Embark Studios), Tokio, Serde, reqwest.
A complete list can be generated with `cargo about` / `cargo license` (Rust) and `pnpm licenses list` (JavaScript).
