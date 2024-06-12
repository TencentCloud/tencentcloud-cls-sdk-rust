#[allow(dead_code)]
pub mod headers {
    pub const LOG_COMPRESS_TYPE: &str = "x-cls-compress-type";
    pub const USER_AGENT_VALUE: &str =
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    pub const DEFAULT_CONTENT_TYPE: &str = "application/x-protobuf";
}
