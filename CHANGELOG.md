# Changelog

## 0.4.0 (2026-11-19)

The `datumesh` binary of datumesh-local 0.4.0 (with datumesh-http 0.4.1, spec-0.6.0): `find`
fills a plain reference in one step and shows each match's canonical call and label; over MCP,
the `whatif` tool evaluates a scenario (value and slice overrides, `solve`) locally or at the
cell's mesh, never stored; `define --tests` takes tests with a formula; `publish --to <mesh>`
roots names and posts the bundle to a public mesh, which needs tests for a calculated function
(the refusal lists what is missing).

## 0.3.3 (2026-11-18)

The `datumesh` binary of datumesh-local 0.3.3 (with datumesh-http 0.3.2): over MCP, `get` returns
a text blob's content (up to 64 KiB) first, checked against its SHA-256 (`blob_checked`), or
otherwise its `blob://` address and size; a mismatch is `E_BLOB_MISMATCH`, never a partial
answer. The `blob://<sha256>` resource reads a held blob, checked.

## 0.3.2 (2026-11-18)

The `datumesh` binary of datumesh-local 0.3.2: `find` falls through to the configured meshes
(`find_meshes`, default `datumesh.com`) when the local names give nothing, one request per mesh,
the answer kept 24 hours, and says where it looked (`searched`); classes and methods are found
with plain descriptions. The Linux binaries are static (musl).

## 0.3.1 (2026-11-16)

The `datumesh` binary of datumesh-local 0.3.1 (0.3.0 of the library was tagged, not published; 0.3.1 also fixes `publish` across file systems): `datumesh mcp` (MCP over stdio, revision
2026-07-28 with the legacy `initialize` for 2025-11-25 and 2025-06-18; `--http [--port N]`,
Streamable HTTP on 127.0.0.1 and [::1] only, define and publish behind a bearer token kept in a
0600 file), `key rotate`, `trust add|revoke|list`, `root --namespace --key [--export]`, and
`publish` printing each cell's call form and URI. Rule A: no binding starts before now.

## 0.2.2 (2026-11-14)

The `datumesh` binary of datumesh-local 0.2.2 (spec-0.5.0; 0.2.1 was tagged, not published): `describe <id>` (a class name, a key
encoding, an object id, a short id or a cell id; exit 0 for an answer, 3 for any other),
`define <name> --class <file>` (a class under `local/`), and `get` of a method call
(`N(keys).m(args)`, `N[label]`), whose envelope carries `resolved` and whose stderr line names the
canonical call. A class is fetched from its forward mesh once and then answered offline.

## 0.2.0 (2026-11-13)

The `datumesh` binary of datumesh-local 0.2.0: the licence gate is the engine's licence
inheritance (spec-0.4.3 8.5) with the publisher's declarations; a declared restricted input
is forwarded, an undeclared one answered `unlicensed` here, open and local inputs computed
here with the inherited licence.

## 0.1.1 (2026-11-05)

The `datumesh` binary of datumesh-local 0.1.1: on Windows, a host name that does not resolve
answers `E_FOREIGN_UNAVAILABLE` with `detail` `dns`, not `network`
([datumesh-local#1](https://github.com/datumesh-com/datumesh-local/issues/1)).

## 0.1.0 (2026-11-04)

The first release: the `datumesh` binary of datumesh-local 0.1.0's CLI.
