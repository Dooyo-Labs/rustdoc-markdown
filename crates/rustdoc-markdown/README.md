# rustdoc-markdown

This crate extracts documentation from crates.io packages and renders it into
a monolithic markdown document.

The tool is designed to generate clear but concise documentation context for
large language models when being used to program in Rust.

In some places the markdown is not technically valid since some headers nest
higher than the h6 level, but it's assumed an LLM would benefit more from the
consistent structure.

As a trade-off that reduces the size of the output, but makes the documentation
of types more indirect, there is support for rolling up "Common Traits" for the
crate and for each module so that the list of implemented traits, per-type,
can be more concise. This feature can be disabled.

If a package contains a `README.md` and examples in an `examples/` directory,
these are also folded into the documentation.

## Command Line

Generate Markdown for `rustdoc-types` version `0.39.0`:
```bash
cargo run -- print rustdoc-types 0.39.0 --output rustdoc-types-api.md
```

Generate Markdown for a local crate:
```bash
cargo run -- print my-local-crate --manifest path/to/my-local-crate/Cargo.toml --output my-local-crate-api.md
```

## Library Usage

`rustdoc-markdown` can also be used as a library to integrate documentation generation
into other tools.

See the [library documentation](https://docs.rs/rustdoc-markdown) for `rustdoc_markdown::Printer`
for an example of how to:
1. Find a crate on crates.io.
2. Download and unpack it.
3. Run `rustdoc` to get the `rustdoc_types::Crate` data.
4. Read extra information like README and examples.
5. Use the `Printer` to generate the Markdown documentation.

## Dependencies

It depends on:
- `nightly-2025-03-24` Rust toolchain.
- `rustdoc-types` version `0.39`.

See the changelog for `rustdoc-types` for compatibility information when updating the toolchain:
<https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md>