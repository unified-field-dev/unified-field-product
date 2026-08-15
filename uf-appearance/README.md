# uf-appearance

Optional Appearance product offering for Unified Field shells.

Ships the desktop app-bar appearance popover and registers it into the default
utilities pack. Preference storage and Valence services remain in `uf-product`.

Types: `cargo doc -p uf-appearance --open`.

## Verify

```bash
cargo check -p uf-appearance --features ssr
cargo check -p shell-chrome-host --features ssr
```

## Related

- Shell offerings features: [`uf-integrations`](../uf-integrations/) (`offering-appearance` / `full`)
- Appearance preferences API: [`uf-product`](../uf-product/)
- Teaching mount: [`examples/shell-chrome-host`](../examples/shell-chrome-host/)
