#!/bin/sh
# Run cargo in a toolchain container on the `datumesh` Docker network:
#   tools/cargo.sh <image-tag> <cargo args...>, e.g. tools/cargo.sh 1.99 test
# Tags: a Rust version (image rust:<tag>), nightly-tools (cargo-public-api), lint.
# The container is named datumesh-cargo-<tag>-<pid> and removed when cargo exits.
#
# DATUMESH_PATCH=<dir>: a directory holding a cargo config file `config.toml` (e.g. a
# [patch.crates-io] of unreleased dependencies, with paths under /patch), mounted read-only
# at /patch and installed as the container's $CARGO_HOME/config.toml (so external
# subcommands such as clippy see it too); DATUMESH_PATCH_FILE names another file there. The
# sibling checkout ../datumesh-local is mounted read-only at /libs/datumesh-local for such a
# patch. Unset: crates.io only.
set -e
tag="$1"; shift
case "$tag" in nightly) image=rustlang/rust:nightly ;; nightly-tools) image=datumesh-nightly-tools ;; lint) image=datumesh-rust-1.99-lint ;; *) image="rust:$tag" ;; esac
root="$(cd "$(dirname "$0")/.." && pwd)"
patch_mount=""
if [ -n "$DATUMESH_PATCH" ]; then
  patch_mount="-v $DATUMESH_PATCH:/patch:ro"
fi
# shellcheck disable=SC2086
exec docker run --rm -i --name "datumesh-cargo-$tag-$$" --network datumesh $patch_mount \
  -v "$HOME/.cache/datumesh-cargo:/usr/local/cargo/registry" -v "$root:/w" -w /w -v "$root/../datumesh-local:/libs/datumesh-local:ro" \
  -e CARGO_TARGET_DIR="/w/target-$tag" -e CARGO_TERM_COLOR=never \
  -e UPSTREAM_DATABASE_URL -e DATUMESH_PATCH_FILE -e RUST_LOG -e RUST_BACKTRACE -e RUSTDOCFLAGS -e RUST_TEST_THREADS -e DATUMESH_LIVE -e DATUMESH_BENCH_OUT \
  "$image" sh -c 'if [ -f /patch/config.toml ]; then cp /patch/${DATUMESH_PATCH_FILE:-config.toml} "${CARGO_HOME:-/usr/local/cargo}/config.toml"; fi; exec cargo "$@"' cargo "$@"
