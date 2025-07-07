# RustTI-benchmarks

Benchmarks for [RustTI](https://github.com/0100101001010000/RustTI) and related technical indicator libraries.

## Purpose

- Fair, reproducible benchmarks for RustTI and similar libraries
- Use realistic OHLCV data and common indicators.
- Help guide optimization and provide transparency for users.

## Structure

- `benches/`: Criterion benchmarks for each indicator.
- `data/`: Place for real-world price data (CSV files, not under version control for large/private datasets).
- `scripts/`: Helper scripts (optional).

## Running

```sh
cargo bench
```

## Adding New Libraries

Add them as `[dev-dependencies]` in Cargo.toml and create parallel benchmarks (see `benches/rsi.rs` for template).

## Contributing

- PRs welcome for new indicators, datasets, or improvement suggestions!
