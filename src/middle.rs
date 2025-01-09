use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

/// 记录 API 到返回的详情
pub async fn trace(req: Request, next: Next) -> Response {
    // before all handler
    let start = std::time::Instant::now();

    let method = req.method().to_string();
    let uri = req.uri().to_string();

    let response = next.run(req).await;
    // after all handler

    let status = response.status();
    
    let past = humantime::format_duration(start.elapsed()).to_string();
    
    log::info!("|{}| {past:>16} | {method:>4} {uri}", colored_status_code(status));

    response
}

fn colored_status_code(status: StatusCode) -> colored::ColoredString {
    use colored::Colorize;

    let code = status.as_u16();
    let codes = code.to_string().bright_white();
    match code {
        100..=199 => codes.on_purple(),
        200..=299 => codes.on_green(),
        300..=399 => codes.on_cyan(),
        400..=499 => codes.on_yellow(),
        500..=599 => codes.on_red(),
        _ => codes.on_bright_black(),
    }
}