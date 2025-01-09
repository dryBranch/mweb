use std::convert::Infallible;

use axum::{handler::Handler, routing::{get, MethodRouter}};

/// allow get and post
pub fn pget<H, T, S>(handler: H) -> MethodRouter<S, Infallible>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + 'static,
{
    get(handler.clone()).post(handler)
}