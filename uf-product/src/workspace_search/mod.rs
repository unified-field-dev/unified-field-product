//! Per-user workspace **content index** (AppBar search) — not picker search sources.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | Index rows, writer, owner-scoped query (SSR) | Picker registry (`crate::search_sources` / `uf-search-core`) |
//! | Indexing policy + relative `link` validation | AppBar UI (`uf-integrations::WorkspaceSearch`) |
//! | Actor guard on [`query`] (User only) | Host Chronon / Boson scheduling for iters |
//! | Teaching stub [`demo`] (IndexedDemoItem SE/Iter) | Production L3 app source models |
//! | [`query_workspace_search`] server fn + [`WorkspaceSearchHit`] DTO | — |
//!
//! ## Integrating a source model
//!
//! Apps register a `SideEffect` / `iters` entry on the **source** schema and call
//! [`SearchDocumentWriter`] (see [`demo::IndexedDemoIndexer`] / [`demo::IndexedDemoBackfillIter`]).
//!
//! ## Privacy
//!
//! Index rows use `OWNER_BY_USER_FIELD` read and `SYSTEM_ONLY` CUD. [`query`] rejects
//! Anonymous and System actors so a mistaken `system_valence()` call cannot dump every
//! user's documents.
//!
//! ## Indexing policy
//!
//! Owner-scoped resource titles are allowed. Do **not** put emails, phone numbers,
//! secrets, session tokens, or auth material in `title` / `searchable_text` / `link`.
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Upsert / delete index rows | [`SearchDocumentWriter`], [`SearchDocumentDraft`] |
//! | Query my hits (library) | [`query`], [`WorkspaceSearchHit`] |
//! | Query from UI | [`query_workspace_search`] |
//! | Errors | [`WorkspaceSearchError`] |
//! | TTL window | [`SEARCH_DOCUMENT_TTL_SECS`] |
//! | Teaching SE / Iter | [`demo`] |

mod dto;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
mod server;

#[cfg(feature = "ssr")]
pub mod demo;
#[cfg(feature = "ssr")]
mod draft;
#[cfg(feature = "ssr")]
mod error;
#[cfg(feature = "ssr")]
mod link;
#[cfg(feature = "ssr")]
mod query;
#[cfg(feature = "ssr")]
mod writer;

pub use dto::WorkspaceSearchHit;
#[cfg(any(feature = "ssr", feature = "hydrate"))]
pub use server::query_workspace_search;

#[cfg(feature = "ssr")]
pub use draft::SearchDocumentDraft;
#[cfg(feature = "ssr")]
pub use error::WorkspaceSearchError;
#[cfg(feature = "ssr")]
pub use query::query;
#[cfg(feature = "ssr")]
pub use writer::{document_id, SearchDocumentWriter, SEARCH_DOCUMENT_TTL_SECS};

#[cfg(all(test, feature = "ssr"))]
mod tests;
