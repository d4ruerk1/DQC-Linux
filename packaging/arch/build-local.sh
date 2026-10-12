#!/usr/bin/env bash
# Build the pacman package from the current git checkout (no tag needed).
# Usage (on Arch/CachyOS):  packaging/arch/build-local.sh [makepkg args, e.g. -si]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../.." && pwd)"
pkgver="$(sed -n 's/^pkgver=//p' "$here/PKGBUILD")"
git -C "$root" archive --format=tar.gz \
  --prefix="discord-quest-completer-${pkgver}/" \
  -o "$here/discord-quest-completer-${pkgver}.tar.gz" HEAD
cd "$here"
makepkg -f "$@"
ls -lh "$here"/*.pkg.tar.zst
