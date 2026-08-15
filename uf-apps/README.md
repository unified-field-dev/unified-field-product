# uf-apps

Apps directory for Unified Field product hosts. Lists every app registered via `uf_app!` and shows ownership, goals, and tasks on a detail page.

Routes and types: `cargo doc -p uf-apps --open`.

## Features

- Index page: searchable grid of registered apps (`AppsIndexPage`)
- Detail page: overview card with product link and optional GitHub / docs.rs links
  (`AppDetailPage`)
- Help spotlights: search + first application card on `/apps`; overview description,
  source, docs, and product link on `/apps/:app_name` (`help_steps`)
- App-bar Apps launcher (`AppBarAppsButton` / `AppsLauncher`): centered dialog
  typeahead over registered apps; select navigates to each app's `route_path`
- Registers as the `"apps"` app via `uf_app!` at `/apps`
## Registration

```rust
// uf-apps/src/lib.rs
uf_app! {
    name: "Apps",
    id: "apps",
    description: "Apps directory and detail pages",
    icon: "📱",
    version: "0.1.0",
    routes: UfAppsRoutes,
    route_path: "/apps",
}
```

Routes:

- `/apps` — index
- `/apps/:app_name` — detail

Host mounts [`UfAppsRoutes`](src/lib.rs) inside `<Routes>`. Teaching mount:
[`examples/shell-chrome-host`](../examples/shell-chrome-host/).

## Verify

```bash
cargo check -p uf-apps --features ssr
cargo check -p shell-chrome-host --features ssr
cargo doc -p uf-apps --no-deps
```

## Related

- Product overlay / registry contracts: [`uf-product`](../uf-product/)
- Build-time `uf_app!` discovery: [`uf-codegen`](../uf-codegen/)
- Signed-in welcome landing: [`uf-welcome`](../uf-welcome/)
