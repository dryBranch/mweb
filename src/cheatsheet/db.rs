//! 数据库示例
//! 
//! ```no_run
//! use std::sync::Arc;
//!
//! use sqlx::{Pool, Sqlite};
//! use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
//!
//! use crate::err::{MError, MResult};
//!
//! pub type RDB = Pool<Sqlite>;
//! pub type MDB = Arc<RDB>;
//!
//! /// 打开 sqlite3 数据库
//! pub async fn open_db(fpath: &str) -> MResult<Pool<Sqlite>> {
//!     let file_exists = std::path::Path::new(fpath).exists();
//!
//!     // 确保文件可读写
//!     if !file_exists {
//!         // 数据库文件不存在
//!         log::warn!("Database is not exists: {fpath}, creates file.");
//!         let _f = match tokio::fs::File::create(fpath).await {
//!             Ok(file) => file,
//!             Err(e) => {
//!                 let msg = format!("create file `{fpath}` failed: {e}");
//!                 log::error!("{msg}");
//!                 return Err(MError::Message(msg));
//!             },
//!         };
//!     }
//!
//!     // 打开文件
//!     log::debug!("connect database: {fpath}");
//!     let pool = sqlx::sqlite::SqlitePoolOptions::new()
//!         .connect(&format!("sqlite:{fpath}")).await?;
//!
//!     // 确保数据库表完整
//!     if !file_exists {
//!         create_tables(&pool).await?;
//!     }
//!
//!     Ok(pool)
//! }
//!
//! /// 创建预设的表
//! async fn create_tables(pool: &Pool<Sqlite>) -> sqlx::Result<()> {
//!     execute(pool, include_str!("./db.sql")).await
//! }
//!
//! /// 简单执行 sql
//! async fn execute(db: &Pool<Sqlite>, sql: &str) -> sqlx::Result<()> {
//!     sqlx::query(sql)
//!         .execute(db).await?;
//!     Ok(())
//! }
//!
//! #[allow(unused)]
//! pub(crate) async fn open_test_db() -> Pool<Sqlite> {
//!     open_db("test.db").await.unwrap()
//! } 
//! ```