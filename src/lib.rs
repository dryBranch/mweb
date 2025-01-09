//! 简易的基于 Axum 的 http 服务器 API 封装 (自用)

pub mod server;
pub mod route;
pub mod logger;
#[cfg(feature = "middle")]
pub mod middle;
#[cfg(feature = "err")]
pub mod err;

pub use server::*;
pub use route::*;
pub use logger::*;


pub use log;
pub use env_logger;
pub use axum;
pub use tokio;
pub use thiserror;
pub use anyhow;
pub use serde;
pub use serde_json;

#[cfg(feature = "sqlx")]
pub use sqlx;
#[cfg(feature = "sqlx_macro_ex")]
pub use sqlx_macro_ex;

#[cfg(feature = "clap")]
pub use clap;
#[cfg(feature = "sha256")]
pub use sha256;
#[cfg(feature = "regex")]
pub use regex;
#[cfg(feature = "once_cell")]
pub use once_cell;