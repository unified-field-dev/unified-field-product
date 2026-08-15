# uf-codegen

Build-time route discovery for Unified Field product apps. Scans workspace packages for `uf_app!` and emits Rust the host `build.rs` includes into the shell.

## What it emits

| File | Role |
|------|------|
| `uf_registered_routes.rs` | Route component imports / re-exports |
| `uf_app_route_table.rs` | Static table for shell page-view analytics |

When two packages share an `app_id`, the `*-app` package wins.

## Usage

In the host `build.rs`:

```rust
use std::path::PathBuf;
use uf_codegen::{generate_registered_routes, RoutesCodegenConfig};

fn main() -> anyhow::Result<()> {
    generate_registered_routes(&RoutesCodegenConfig {
        workspace_root: PathBuf::from(
            std::env::var("CARGO_WORKSPACE_DIR").unwrap_or_else(|_| {
                PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
                    .parent()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            }),
        ),
        out_dir: PathBuf::from(std::env::var("OUT_DIR").unwrap()),
        extra_packages: vec![],
        excluded_packages: vec!["setup-wizard-app".to_string()],
    })
}
```

Include both outputs:

```rust
mod uf_registered_routes {
    include!(concat!(env!("OUT_DIR"), "/uf_registered_routes.rs"));
}
mod uf_app_route_table {
    include!(concat!(env!("OUT_DIR"), "/uf_app_route_table.rs"));
}
```

Discovery uses `syn` on `src/lib.rs` and `src/main.rs` of each scanned package. Only packages with both `id` and `routes` in `uf_app!` contribute route imports.

## Verify

```bash
cargo check -p uf-codegen
cargo test -p uf-codegen
cargo run -p uf-codegen --example emit_routes_table
cargo doc -p uf-codegen --open
```

Runnable emit demo: [`examples/README.md`](examples/README.md).

## Related

- Macro that registers apps: [`uf-product-macros`](../uf-product-macros/)
- Runtime registry / guards: [`uf-product`](../uf-product/)
- Runnable discovery example: [`uf-product/examples`](../uf-product/examples/)
