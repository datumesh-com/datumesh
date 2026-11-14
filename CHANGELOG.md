# Changelog

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
