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
datumesh serve
datumesh --help
```

See datumesh-local's README for the commands, the `local.invalid` namespace, the licence gate,
offline answers (`E_FOREIGN_UNAVAILABLE`), export and import, and the configuration.

## License

Apache-2.0.
