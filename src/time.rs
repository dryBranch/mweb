/// 获取当前时间的字符串
/// 
/// 格式 `%Y-%m-%d %H:%M:%S`
pub fn now() -> String {
    chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}