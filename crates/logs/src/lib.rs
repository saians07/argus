#[doc(hidden)]
pub use tracing;

use tracing_subscriber::{
    EnvFilter,
    fmt::{format, layer, time},
    layer::SubscriberExt,
    registry,
    util::SubscriberInitExt,
};

struct ChronoLocalTimer;

impl time::FormatTime for ChronoLocalTimer {
    fn format_time(&self, w: &mut format::Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%H:%M:%S%.3f"))
    }
}

fn create_query(query: &str) -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| query.into())
}

pub fn start_tracing(query: &str) {
    let _ = registry()
        .with(create_query(query))
        .with(
            layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_timer(ChronoLocalTimer)
                .with_test_writer()
                .compact(),
        )
        .try_init();
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::tracing::info!("{}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        tracing::warn!("{}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!("{}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!("{}", format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_info_with_ids {
    ($request_id:expr, $($arg:tt)*) => {
        let short_id = &$request_id.as_str()[..8];
        $crate::tracing::info!("[{}] {}", short_id, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_warning_with_ids {
    ($request_id:expr, $($arg:tt)*) => {
        let short_id = &$request_id.as_str()[..8];
        $crate::tracing::warn!("[{}] {}", short_id, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_debug_with_ids {
    ($request_id:expr, $($arg:tt)*) => {
        let short_id = &$request_id.as_str()[..8];
        $crate::tracing::debug!("[{}] {}", short_id, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! log_error_with_ids {
    ($request_id:expr, $($arg:tt)*) => {
        let short_id = &$request_id.as_str()[..8];
        $crate::tracing::error!("[{}] {}", short_id, format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use crate::create_query;

    #[test]
    fn check_query() {
        let query = "exter_app=info";
        let res = create_query(query).to_string();
        assert_eq!(res, "exter_app=info".to_string())
    }
}
