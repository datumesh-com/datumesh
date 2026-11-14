# datumesh

The `datumesh` command: the [Datumesh](https://datumesh.com) local resolver on your machine.
The library is [`datumesh-local`](https://crates.io/crates/datumesh-local); this crate only
holds the binary, so that

```text
cargo install datumesh
```

gives you `datumesh`. Prebuilt binaries for Linux (x86_64, aarch64), macOS (aarch64, x86_64)
and Windows (x86_64) are attached to each GitHub release.

```text
datumesh get 'ecb/fx_rate(base: "EUR", quote: "USD", date: 2026-10-30)'
datumesh prefetch 'ecb/fx_rate(base: *, quote: *, date: *)'
datumesh --offline get 'ecb/fx_rate(base: "EUR", quote: "GBP", date: 2026-10-29)'
datumesh define local/closed --formula '…' --signature '…'
datumesh define local/pair --class pair.json
datumesh describe 'ecb/fx_pair'
datumesh get 'ecb/fx_pair(EUR, USD).rate(2026-10-30)'
datumesh serve
datumesh --help
```

A request may be a call in the method form (`ecb/fx_pair(EUR, USD).rate(2026-10-30)`), which
gives the same cell as its plain read; the answer's `resolved` says what the call resolved, and
the stderr line names the canonical call. `describe <id>` takes a class name, a key encoding, an
object id, a short id or a cell id; exit status 0 for an answer, 3 for any other.

See datumesh-local's README for the commands, the `local.invalid` namespace, the licence gate,
offline answers (`E_FOREIGN_UNAVAILABLE`), export and import, and the configuration.

## License

Apache-2.0.
