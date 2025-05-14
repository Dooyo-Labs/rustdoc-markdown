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
can be more concise.

If a package contain a `README` and `examples/` these are also folded into the
documentation.

## Command Line

```bash
cargo run -- rustdoc-types 0.39 --output rustdoc-types-api.md
```

## Dependencies

It depends on:
- `nightly-2025-03-24` Rust toolchain.
- `rustdoc-types` version `0.39`.

See the changelog for `rustdoc-types` for compatibility information when updating the toolchain:
<https://github.com/rust-lang/rustdoc-types/blob/trunk/CHANGELOG.md>