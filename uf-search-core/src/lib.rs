//! Leptos-free types for the `/search` command palette: source keys/items shared with the
//! client, plus (behind `ssr`) the backend provider trait and inventory-based registry.
//!
//! ## Owns / Does not own
//!
//! | Owns | Does not own |
//! |------|----------------|
//! | Client DTOs ([`SearchSourceKey`], [`SearchSourceItem`]) | Leptos / UI combobox (`uf-integrations::SearchSourcePicker`) |
//! | SSR provider trait + inventory registry | `define_search_sources!` macro expansion (`uf-product-macros`) |
//! | Fan-out query helpers on `SearchSourceRegistry` (`ssr`) | Product re-export surface (`uf-product::search_sources`) |
//!
//! ## Concern → API
//!
//! | Concern | API |
//! |---------|-----|
//! | Client source selection | [`SearchSourceKey`] |
//! | Result rows for the picker | [`SearchSourceItem`] |
//! | Implement one backend source (`ssr`) | `SearchSourceProvider`, `SearchSourceDescriptor` |
//! | Provider / fan-out failures (`ssr`) | [`SearchSourceError`] |
//! | Query many sources (`ssr`) | `SearchSourceRegistry` |
//!
//! ## Features
//!
//! - **Client-safe DTOs** — [`SearchSourceKey`] and [`SearchSourceItem`] have no SSR-only
//!   dependencies, so they can be sent to and rendered by the browser.
//! - **Backend registry (`ssr` feature)** — `SearchSourceProvider` is implemented once per
//!   source (e.g. "apps", "notifications"); `SearchSourceDescriptor` registers an instance via
//!   `inventory::submit!`; `SearchSourceRegistry` fans a query out across selected sources.
//!
//! ## Getting started
//!
//! Registering a search source (`ssr`-only):
//!
//! ```rust,ignore
//! use uf_search_core::{SearchSourceDescriptor, SearchSourceFuture, SearchSourceItem, SearchSourceProvider};
//!
//! struct MyProvider;
//! impl SearchSourceProvider for MyProvider {
//!     fn query<'a>(
//!         &'a self,
//!         valence: &'a valence::Valence,
//!         query_text: &'a str,
//!         max_results: u32,
//!     ) -> SearchSourceFuture<'a> {
//!         Box::pin(async move { Ok(Vec::<SearchSourceItem>::new()) })
//!     }
//! }
//!
//! static PROVIDER: MyProvider = MyProvider;
//! quark::inventory::submit! {
//!     SearchSourceDescriptor {
//!         id: "my-source",
//!         label: "My Source",
//!         description: "Searches my stuff",
//!         provider: &PROVIDER,
//!     }
//! }
//! ```
//!
//! ## Examples
//!
//! | Level | Where | What |
//! |-------|-------|------|
//! | Highlight | Getting started above | `SearchSourceProvider` + `inventory::submit!` |
//! | Mid / detailed | workspace `uf-search-core/examples/query_many_stub.rs` | Fan-out `SearchSourceRegistry::query_many` |
//! | UI | `uf-integrations::SearchSourcePicker` | Combobox over these DTOs |
//!
//! ```bash
//! cargo run -p uf-search-core --example query_many_stub --features ssr
//! ```
//!
//! ## Where to look next
//!
//! - [`SearchSourceKey`] / [`SearchSourceItem`] — client-visible DTOs.
//! - `SearchSourceRegistry` (`ssr`) — the fan-out query registry.
//! - `uf-product-macros` — `define_search_sources!` to emit descriptors.
//! - `uf-integrations` — `SearchSourcePicker` UI over these DTOs.
//! - `uf-product::search_sources` — product re-export for app crates.

mod dto;
#[cfg(feature = "ssr")]
mod ssr;

pub use dto::{SearchSourceItem, SearchSourceKey};

#[cfg(feature = "ssr")]
pub use ssr::{
    SearchSourceDescriptor, SearchSourceError, SearchSourceFuture, SearchSourceProvider,
    SearchSourceRegistry, SearchSourceResult,
};
