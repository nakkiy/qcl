use tracing_subscriber::fmt::format::FmtSpan;

pub fn init_logger(log_level: &str, log_file: Option<String>) {
    if let Some(file_path) = log_file {
        let file_appender = tracing_appender::rolling::never(".", &file_path);
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::fmt()
            .with_writer(non_blocking)
            .with_max_level(log_level.parse().unwrap_or(tracing::Level::INFO))
            .with_span_events(FmtSpan::CLOSE)
            .init();
    }
}
