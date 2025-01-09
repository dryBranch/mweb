//! 简易的基于 Axum 的 http 服务器 API 封装 (自用)

pub mod server;
pub mod route;
pub mod logger;
#[cfg(feature = "middle")]
pub mod middle;
#[cfg(feature = "err")]
pub mod err;
pub mod prelude;

pub use server::*;
pub use route::*;
pub use logger::*;
