# uf-notifications (product UI)

Bell, dropdown, and `/notifications` inbox for Unified Field hosts. Persistence and
`#[server]` ops live in the domain repo
[uf-notifications](https://github.com/deathbreakfast/uf-notifications)
(`uf-notifications-core`, `uf-notifications-api`).

## Mount

1. Depend on this crate plus domain core/api with matching `ssr` / `hydrate`.
2. Enable `uf-integrations` `offering-notifications` (or `full`) and call
   `ensure_notification_bell_linked()` once at `App` root.
3. Nest `<NotificationsRoutes />` under your host `<Routes>`.
4. Mount Photon `/ws/notifications` when you want live badge refresh.

Override the inventory bell only with
`uf_integrations::provide_shell_notification_bell`.

Crate-root rustdoc has the full get-started ladder (Features vs Feature flags).

## Verify

See [docs/VERIFICATION.md](../docs/VERIFICATION.md) (`cargo check -p uf-notifications --features ssr`
and Playwright `notifications.spec.ts` under `uf-product-ui-e2e`).
