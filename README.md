# basic-rust
Simple Basic Rust Examples

It's assumed that Rust is [configured and running](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Compile and execute unit, integration, and documentation [tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
```bash
cargo test
```

Build the [documentation](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for the local package and all dependencies. The output is placed in target/doc.
```bash
cargo doc
```

[Compile](https://doc.rust-lang.org/cargo/commands/cargo-build.html) local packages and all of their dependencies.
```bash
cargo build
```

Execute the compiled code.
```bash
./target/debug/basic-rust --dev
```

[Remove](https://doc.rust-lang.org/cargo/commands/cargo-clean.html) artifacts from the target directory that Cargo has generated in the past.
```bash
cargo clean
```

References:
[The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
[rustlings](https://github.com/rust-lang/rustlings/)
