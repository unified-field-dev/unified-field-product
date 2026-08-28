//! Product UI e2e host library.
#![allow(missing_docs)]

mod app;
#[cfg(feature = "ssr")]
mod e2e_spectra;
#[cfg(feature = "ssr")]
mod e2e_valence;
mod gate_demos;
mod harness_auth_menu;
mod help_steps;
mod pages;
#[cfg(feature = "ssr")]
pub mod seed;

pub use app::{shell, App};
#[cfg(feature = "ssr")]
pub use e2e_spectra::e2e_spectra;
#[cfg(feature = "ssr")]
pub use e2e_valence::{e2e_valence, init_e2e_valence};
