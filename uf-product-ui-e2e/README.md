# uf-product-ui-e2e

Library-owned Leptos host + Playwright for product operator surfaces
(`uf-integrations`, `uf-apps`, `uf-welcome`, offering chrome).

Graduates [`examples/shell-chrome-host`](../examples/shell-chrome-host/) with a
tower-sessions e2e seed so [`RequireAuthenticated`](../uf-product/) runs on a
real `AuthContext` (harness session keys — not a duplicate of lepton auth funnels).

## Scenario catalog

| ID | Kind | Asserts |
|----|------|---------|
| `pw-shell-layout-default-closed-happy` | happy | Shell mounts; home visible |
| `pw-shell-sidebar-toggle-happy` | happy | Left nav reachable after toggle |
| `pw-app-bar-default-utilities-happy` | happy | Help / Apps / Appearance testids |
| `pw-app-bar-utilities-override-happy` | happy | Custom utilities; defaults absent |
| `pw-app-bar-slots-visible-happy` | happy | Search + trailing + user menu |
| `pw-app-bar-mobile-visible-at-top-happy` | happy | Mobile `/scroll-chrome`: bar visible at top |
| `pw-app-bar-mobile-hide-on-scroll-down-happy` | happy | Mobile: page-scroll down tucks bar (attr + geometry) |
| `pw-app-bar-mobile-show-on-scroll-up-happy` | happy | Mobile: scroll up restores bar |
| `pw-app-bar-mobile-hide-keeps-bar-mounted-happy` | happy | Mobile: bar/utilities stay mounted while tucked |
| `pw-app-bar-desktop-sticky-no-hide-happy` | happy | Desktop: same scroll never tucks bar |
| `pw-search-picker-select-happy` | happy | Select Beacon Alpha |
| `pw-search-picker-no-match-sad` | sad | No options for `zz-no-match` |
| `pw-workspace-search-desktop-select-happy` | happy | AppBar content search → hit → `/workspace-search-hit` |
| `pw-workspace-search-desktop-no-match-sad` | sad | No hit for `zz-no-workspace-match` |
| `pw-workspace-search-mobile-select-happy` | happy | Compact trigger → Dialog → navigate |
| `pw-workspace-search-mobile-no-match-sad` | sad | Compact Dialog no matches |
| `pw-workspace-search-unauth-sad` | sad | Anonymous: AppBar search disabled (`Sign in to search`) |
| `pw-coming-soon-page-happy` | happy | Coming soon page testid |
| `pw-not-found-page-happy` | happy | Not found page testid |
| `pw-auth-gate-unauthenticated-sad` | sad | Sign-in gate dialog on `/welcome` |
| `pw-auth-gate-signin-click-happy` | happy | Gate Sign In opens harness `auth-dialog-root` on `/welcome` (no route change; stub dialog, not lepton credentials) |
| `pw-welcome-authenticated-happy` | happy | Welcome cards + empty copy for Featured/Recent/My most used/Popular |
| `pw-welcome-usage-seeded-happy` | happy | **Fixture/integ-style UI:** seeded Spectra page views populate Recent / My most used / Popular; other viewers excluded from mine/recent |
| `pw-welcome-usage-emit-fetch-happy` | happy | **Emit→fetch E2E:** navigate `/apps` then `/welcome`; Recent + My most used include Apps via `PageViewTracker` |
| `pw-welcome-admin-unauthenticated-sad` | sad | Auth gate on `/welcome/admin` |
| `pw-welcome-admin-denied-sad` | sad | Signed-in without e2e admin / WelcomeAdmin sees deny MessageBar |
| `pw-welcome-admin-featured-crud-happy` | happy | Seed `welcome_admin`; add Apps+Welcome; Featured updates; reorder; remove clears |
| `pw-auth-gate-unverified-email-sad` | sad | Email verification gate dialog |
| `pw-auth-gate-permission-denied-sad` | sad | Permission gate dialog when `e2e.permission.deny` |
| `pw-auth-gate-permission-allow-happy` | happy | Seed `permission_allow`; `/gate/permission-allow` shows content |
| `pw-auth-gate-permission-request-redirect-sad` | sad | Request Permission → `/permission/permissions` |
| `pw-apps-index-list-filter-happy` | happy | Index lists apps; filter `Welcome` keeps welcome card |
| `pw-apps-index-filter-empty-sad` | sad | Unknown filter → empty state |
| `pw-apps-detail-happy` | happy | Index → Open → detail without not-found |
| `pw-apps-detail-overview-links-happy` | happy | Apps detail: GitHub + docs.rs open correct URLs |
| `pw-apps-detail-overview-links-absent-sad` | sad | Welcome detail: GitHub present; no docs.rs |
| `pw-apps-detail-unknown-sad` | sad | Unknown slug → `App not found.` |
| `pw-apps-launcher-open-empty-happy` | happy | Apps button → dialog; empty prompt; no results |
| `pw-apps-launcher-typeahead-filter-happy` | happy | Type `Welcome` → welcome result only |
| `pw-apps-launcher-filter-empty-sad` | sad | Unknown query → no-match empty |
| `pw-apps-launcher-select-navigate-happy` | happy | Select Welcome → `/welcome`; launcher closes |
| `pw-apps-launcher-dismiss-esc-happy` | happy | Escape closes dialog; stay on `/` |
| `pw-apps-launcher-mobile-open-empty-happy` | happy | Mobile viewport → dialog + empty prompt |
| `pw-apps-launcher-mobile-select-navigate-happy` | happy | Mobile select Welcome → `/welcome` |
| `help-spotlight-once-anon` | happy | Anon `/coming-soon` tour shows once; reload stays quiet |
| `help-spotlight-once-authed` | happy | Authed `/apps` tour shows once across reload |
| `help-spotlight-replay-current-route` | happy | Replay reopens tour on current route |
| `help-spotlight-replay-does-not-affect-other-route` | happy | Replay on `/apps` leaves `/welcome` quiet |
| `help-spotlight-mobile-viewport` | happy | Tour + Help AdaptiveMenu on narrow viewport |
| `help-report-bug-intro-and-form` | happy | Bug dialog intro → no-account form |
| `help-report-security-private-copy` | happy | Security intro forbids public issues |
| `help-report-feature-intro` | happy | Feature dialog opens from Help menu |

## Run

```bash
# From workspace root (builds SSR + hydrate, then Playwright):
cd uf-product-ui-e2e/end2end && npm ci && npx playwright install chromium
cd ../..
# This host keeps uf-apps / uf-welcome `lazy-routes` off (single WASM).
# Product hosts that use `cargo leptos --split` leave `lazy-routes` on (crate default).
cargo leptos end-to-end --project uf-product-ui-e2e
```

Harness seed: `POST /api/test/seed-data` with `{ "auth": "authenticated_verified" }`
(also `anonymous`, `authenticated_unverified`). Optional `page_views` and `usage_viewer`
seed mem Spectra for usage-card fixtures (one process-global Spectra; isolate via
viewer keys). Optional `welcome_admin: true` sets `uf_e2e_welcome_admin` so featured
mutations use the harness Valence seam (session flag; not Gauge `PermissionBackend`).
Optional `permission_allow: true` sets `uf_e2e_permission_allow` and a process-global
flag so the harness `PermissionBackend` allows `e2e.permission.allow` (still not live
Gauge `actor_can`).

## Runtime scope

Product crates use **Valence** and **Spectra**. Production hosts also supply **Higgs**
(session Valence / `from_request`) and, when wired, a host **Gauge**
`PermissionBackend`. This e2e host stubs Higgs with tower-sessions and provides a
harness `PermissionBackend` for gate allow/deny demos (not Gauge evaluation).

**Chronon, Boson, and Photon are not product dependencies** — no crate in this
workspace calls them. Host composition for those runtimes is covered by L5
IsolatedLab (`uf-embedded-e2e`, `uf-site-e2e`, …). Real Gauge allow evaluation is
covered where hosts call `wire_gauge_permissions()`, not here.
