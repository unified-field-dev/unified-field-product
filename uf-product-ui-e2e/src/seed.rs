//! Harness-only seed endpoint for Playwright.

use axum::http::StatusCode;
use axum::Json;
use chrono::{TimeZone, Utc};
use serde::Deserialize;
use serde_json::json;
use spectra::try_log_event_at;
use tower_sessions::Session;
use uf_product::telemetry::usage::{E2E_USAGE_VIEWER_SESSION_KEY, PAGE_VIEW_LOG_TABLE};

use crate::e2e_spectra::e2e_spectra;
use crate::e2e_valence::e2e_valence;
use crate::gate_demos::{write_e2e_auth_kind, E2eAuthKind};
use uf_welcome::E2E_WELCOME_ADMIN_SESSION_KEY;

#[derive(Debug, Deserialize)]
pub struct SeedPageView {
    /// [`uf_product::AppRegistration::id`].
    pub app_id: String,
    /// Spectra `viewer_key` (defaults to e2e usage viewer).
    #[serde(default)]
    pub viewer_key: Option<String>,
    /// Seconds before now (for ordering).
    #[serde(default)]
    pub age_secs: i64,
}

#[derive(Debug, Deserialize)]
pub struct SeedRequest {
    /// E2e auth kind: `anonymous`, `authenticated_verified`, `authenticated_unverified`.
    #[serde(default = "default_auth")]
    pub auth: String,
    /// Override Spectra / welcome usage viewer key (defaults from auth kind).
    #[serde(default)]
    pub usage_viewer: Option<String>,
    /// When true, set e2e welcome-admin session key for featured mutations.
    #[serde(default)]
    pub welcome_admin: bool,
    /// Optional page-view events into mem Spectra (usage cards).
    #[serde(default)]
    pub page_views: Vec<SeedPageView>,
}

fn default_auth() -> String {
    E2eAuthKind::Anonymous.as_str().to_string()
}

fn default_viewer_for(kind: E2eAuthKind) -> Option<&'static str> {
    match kind {
        E2eAuthKind::AuthenticatedVerified => Some("e2e-user"),
        E2eAuthKind::AuthenticatedUnverified => Some("e2e-unverified"),
        E2eAuthKind::Anonymous => None,
    }
}

pub async fn seed_data(
    session: Session,
    Json(body): Json<SeedRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let kind = E2eAuthKind::parse(&body.auth);
    write_e2e_auth_kind(&session, kind)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let viewer = body
        .usage_viewer
        .clone()
        .or_else(|| default_viewer_for(kind).map(str::to_string));

    if let Some(ref viewer) = viewer {
        session
            .insert(E2E_USAGE_VIEWER_SESSION_KEY, viewer.clone())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        session
            .remove::<String>(E2E_USAGE_VIEWER_SESSION_KEY)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if body.welcome_admin {
        session
            .insert(E2E_WELCOME_ADMIN_SESSION_KEY, true)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        session
            .remove::<bool>(E2E_WELCOME_ADMIN_SESSION_KEY)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Process-global Valence: clear featured catalog so scenarios do not leak rows.
    {
        use uf_welcome::featured::clear_all;
        use valence::Actor;
        let valence = e2e_valence().with_actor(Actor::System {
            operation: "e2e_seed_clear_featured".into(),
        });
        let _ = clear_all(&valence).await;
    }

    // Seed IndexedDemoItem under the verified e2e user (SE writes workspace index).
    let mut workspace_search_seeded = false;
    if matches!(kind, E2eAuthKind::AuthenticatedVerified) {
        use uf_product::generated::IndexedDemoItem;
        use valence::{Actor, Model, RecordId};
        let valence = e2e_valence().with_actor(Actor::System {
            operation: "e2e_seed_workspace_search".into(),
        });
        let row = IndexedDemoItem::new(
            RecordId::new("user", "e2e-user"),
            "WorkspaceBeaconAlpha".into(),
            "/workspace-search-hit".into(),
        )
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        IndexedDemoItem::upsert("e2e-ws-1", row, &valence)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        workspace_search_seeded = true;
    }

    // Ensure global Spectra is installed before try_log_event_at.
    let spectra = e2e_spectra();
    let now = Utc::now().timestamp();
    for pv in &body.page_views {
        let event_viewer = pv
            .viewer_key
            .clone()
            .or_else(|| viewer.clone())
            .unwrap_or_else(|| "anonymous".into());
        let ts = Utc
            .timestamp_opt(now - pv.age_secs.max(0), 0)
            .single()
            .ok_or(StatusCode::BAD_REQUEST)?;
        try_log_event_at(
            PAGE_VIEW_LOG_TABLE,
            &json!({
                "path": format!("/{}", pv.app_id),
                "app_id": pv.app_id,
                "app_name": pv.app_id,
                "route_prefix": format!("/{}", pv.app_id),
                "surface": "main_shell",
                "auth": "authenticated",
                "email_verified": "unknown",
                "viewer_key": event_viewer,
                "nav_kind": "e2e_seed",
                "referrer_path": "",
                "outcome": "ok",
                "permission_name": "",
                "role_count": 0,
            }),
            ts,
        );
    }
    if !body.page_views.is_empty() {
        spectra
            .flush_persist()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let mut recent_preview = Vec::<String>::new();
    if let Some(ref viewer) = viewer {
        use uf_product::telemetry::usage::{recent_apps_for_viewer, UsageQueryOptions};
        if let Ok(apps) =
            recent_apps_for_viewer(spectra.as_ref(), viewer, &UsageQueryOptions::default()).await
        {
            recent_preview = apps.into_iter().map(|a| a.app_id).collect();
        }
    }

    Ok(Json(serde_json::json!({
        "ok": true,
        "auth": kind.as_str(),
        "usage_viewer": viewer,
        "welcome_admin": body.welcome_admin,
        "page_views": body.page_views.len(),
        "recent_preview": recent_preview,
        "workspace_search_seeded": workspace_search_seeded,
    })))
}
