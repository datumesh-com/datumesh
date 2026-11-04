#!/bin/sh
# Packaging test: `cargo package`, check the .crate is the binary only, `cargo install` it
# from the unpacked package into a fresh root, then run the installed datumesh: --version,
# and a first run offline in a directory of its own (key, an unfetched cell: exit 3 with
# E_FOREIGN_UNAVAILABLE).
#   tools/package_test.sh [toolchain]   in fresh rust:<toolchain> containers (tools/cargo.sh)
#   tools/package_test.sh --native      with the local cargo (CI)
set -e
root="$(cd "$(dirname "$0")/.." && pwd)"
version="$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/Cargo.toml" | head -n 1)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
script='
  set -e
  cargo install --locked --path "datumesh-'"$version"'" --root /tmp/inst 2>&1 | tail -n 2 || cargo install --path "datumesh-'"$version"'" --root /tmp/inst 2>&1 | tail -n 2
  /tmp/inst/bin/datumesh --version
  /tmp/inst/bin/datumesh --home /tmp/h key > /dev/null
  set +e
  /tmp/inst/bin/datumesh --home /tmp/h --offline get "ecb/fx_rate(base: \"EUR\", quote: \"USD\", date: 2026-10-30)" > /tmp/out
  code=$?
  set -e
  test "$code" = 3
  grep -q E_FOREIGN_UNAVAILABLE /tmp/out
'
if [ "$1" = "--native" ]; then
  (cd "$root" && cargo package --locked)
  cp "$root/target/package/datumesh-$version.crate" "$work/"
  (cd "$work" && tar xzf "datumesh-$version.crate" && sh -c "$script")
else
  tag="${1:-1.99}"
  if [ -n "$DATUMESH_PATCH" ]; then verify="--no-verify"; else verify="--locked"; fi
  "$root/tools/cargo.sh" "$tag" package --allow-dirty $verify
  cp "$root/target-$tag/package/datumesh-$version.crate" "$work/"
  (cd "$work" && tar xzf "datumesh-$version.crate")
  patch_mount=""
  if [ -n "$DATUMESH_PATCH" ]; then patch_mount="-v $DATUMESH_PATCH:/patch:ro -v $root/../datumesh-local:/libs/datumesh-local:ro"; fi
  # shellcheck disable=SC2086
  docker run --rm --name "datumesh-package-$$" $patch_mount -e DATUMESH_PATCH_FILE \
    -v "$HOME/.cache/datumesh-cargo:/usr/local/cargo/registry" -v "$work:/p" -w /p "rust:$tag" \
    sh -c "if [ -f /patch/config.toml ]; then cp /patch/\${DATUMESH_PATCH_FILE:-config.toml} /usr/local/cargo/config.toml; fi; $script"
fi
if tar tzf "$work/datumesh-$version.crate" | grep -vE '/(src/main\.rs|Cargo\.toml|Cargo\.toml\.orig|Cargo\.lock|LICENSE|README\.md|\.cargo_vcs_info\.json)$'; then
  echo "more than the binary in the packaged crate" >&2
  exit 1
fi
echo "package test passed ($version)"
