#!/bin/bash

set -e

cargo run -- print rustdoc-types 0.39 > .ai/project-instructions.md
cargo run -- print cargo-manifest 0.19 >> .ai/project-instructions.md
cargo run -- print pulldown-cmark 0.13 >> .ai/project-instructions.md
cargo run -- print pulldown-cmark-to-cmark 21 >> .ai/project-instructions.md
cargo run -- print glob 0.3 >> .ai/project-instructions.md