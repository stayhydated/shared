# stayhydated-dioxus-core

[![Codecov: stayhydated-dioxus-core][codecov-badge]][codecov]

`stayhydated-dioxus-core` provides generic Dioxus components and visual assets
for authors of public repository sites, including landing and portal primitives,
metadata, demo cards, tabs, selects, full-screen demo framing, motion styles,
and the shared shader background.

Requires Rust 1.99 or newer and Dioxus 0.7. This is an unpublished Git-sourced
package; keep it on the same shared revision as any configured facade.

## Overview

Use this crate for individual primitives. Use
[`stayhydated-dioxus`][stayhydated-dioxus] when a site needs consumer-owned
project configuration, routing, and a complete application shell. Those
configured shells include the shared styles automatically.

When rendering multiple `ShaderBackground` components, give each a distinct
`canvas_id` so each renderer selects its own canvas.

## Example

Compose the primitives in a Dioxus component and include `SharedStyles` once at
the application root:

```rust
use dioxus::prelude::*;
use stayhydated_dioxus_core::{CodeBlock, SharedStyles};

#[component]
fn App() -> Element {
    rsx! {
        SharedStyles {}
        CodeBlock { code: "let total = 8 + 13 + 21;" }
    }
}
```

The component displays a code block with the bundled theme.

[stayhydated-dioxus]: https://github.com/stayhydated/shared/tree/master/crates/stayhydated-dioxus
[codecov-badge]: https://codecov.io/github/stayhydated/shared/branch/master/graph/badge.svg?component=stayhydated-dioxus-core
[codecov]: https://app.codecov.io/github/stayhydated/shared
