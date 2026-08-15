# uf-help

Optional Help product offering for Unified Field shells.

Ships spotlight tours, the app-bar Help menu, and no-account GitHub filing against
each app's `uf_app!` `repository`.

Types: `cargo doc -p uf-help --open`.

## Features

- **Incremental highlights** — tour progress is keyed by `feature_highlight`, not
  by route alone. When you ship a new UI feature, add a `#[help_spotlight_step]`
  with a **new** key. Users who already finished older steps on that route still
  get a tour, but only for the unseen (and replay-flagged) steps. Already-seen
  steps stay quiet until Help → Replay spotlight tour, which resets the **current
  route only**.
- **AdaptiveMenu Help** — Report a bug, Request a feature, Report a security
  issue, Replay spotlight tour. Wide viewports use a popover; narrow viewports
  open an overlay drawer (`Breakpoint::Md`).
- **Repository from `uf_app!`** — Bug / feature / security deep links and the
  GitHub bot target come from `AppRegistration.repository` for the active route.
  When `repository` is missing, reporting is disabled for that app (Replay still
  works).
- **Anon local mirror** — signed-out progress lives in `localStorage`
  (`uf.help.tour_steps`) and merges into Valence on the first authenticated write.
- **Default shell mount** — `HelpTourPlayer` mounts from `UnifiedFieldShellLayout`
  when the `offering-help` feature is enabled.
- **GitHub bot** — bug/feature create public issues; security uses the private
  vulnerability report API. Hosts seal `help.github_feedback` with Neutrino and
  call `set_github_token_resolver` (or export `UF_HELP_GITHUB_TOKEN` in lab).

## Authoring steps

```rust,ignore
use leptos::prelude::*;
use uf_help_macros::help_spotlight_step;

#[help_spotlight_step(
    route = "/apps",
    feature_highlight = "apps-search",
    title = "Search apps",
    spotlight = "apps-search-input",
    order = 10,
)]
#[component]
pub fn AppsSearchHelp() -> impl IntoView {
    view! { <p>"Use search to find installed apps by name."</p> }
}
```

Keep `feature_highlight` keys stable. Renaming a key is treated as a new highlight.
Omit `spotlight` to center the panel in the viewport (no cutout target).
Use `title` for the panel header; it defaults to `feature_highlight` when omitted.

## Verify

```bash
cargo check -p uf-help --features ssr
cargo test -p uf-help --features ssr
cargo check -p shell-chrome-host --features ssr
```

## Related

- Shell offerings features: [`uf-integrations`](../uf-integrations/) (`offering-help` / `full`)
- Teaching mount: [`examples/shell-chrome-host`](../examples/shell-chrome-host/)
- Macro crate: [`uf-help-macros`](../uf-help-macros/)
