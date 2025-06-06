This is a Rust implementation of [Bill Dueber's library_stdnums ruby gem](https://github.com/billdueber/library_stdnums).

# To get started

1. Install Rust using the command provided at https://www.rust-lang.org/tools/install
1. Restart your shell
1. `git clone git@github.com:pulibrary/library_stdnums.git`
1. `cd library_stdnums`
1. Run `cargo test`

# Mutation testing

This crate uses [cargo-mutants](https://mutants.rs) for mutation testing in CI.
To run it locally:

```
cargo install --locked cargo-mutants
cargo mutants
```

# Compare the performance of two branches

```
git checkout branch1
cargo bench
git checkout branch2
cargo bench
```

The second `cargo bench` run will run various benchmarks and, for each one, report either "Performance has improved." or "Performance has regressed."
