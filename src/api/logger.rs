use std::sync::OnceLock;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
#[cfg_attr(feature = "uniffi", uniffi::export(callback_interface))]
pub trait ClientLogger: Send + Sync {
    fn log(&self, level: LogLevel, message: String);
}
struct CallbackLogger {
    callback: Box<dyn ClientLogger>,
    level: log::LevelFilter,
}
static LOGGER: OnceLock<CallbackLogger> = OnceLock::new();
thread_local! { static IN_LOG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
impl log::Log for CallbackLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        // Muon 3 logs raw fork selectors in its auth tracing events. Account
        // failures are delivered as structured, credential-free API events.
        metadata.level() <= self.level && metadata.target().starts_with("protun_next")
    }
    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        IN_LOG.with(|in_log| {
            if in_log.replace(true) {
                return;
            }
            struct Reset<'a>(&'a std::cell::Cell<bool>);
            impl Drop for Reset<'_> {
                fn drop(&mut self) {
                    self.0.set(false);
                }
            }
            let _reset = Reset(in_log);
            let level = match record.level() {
                log::Level::Trace => LogLevel::Trace,
                log::Level::Debug => LogLevel::Debug,
                log::Level::Info => LogLevel::Info,
                log::Level::Warn => LogLevel::Warn,
                log::Level::Error => LogLevel::Error,
            };
            self.callback
                .log(level, format!("{}: {}", record.target(), record.args()));
        });
    }
    fn flush(&self) {}
}
/// First initialization wins. No environment mutation or process-wide panic-hook
/// replacement is needed to deliver the v3.0.5 logging callback contract.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn init_logger(level: LogLevel, logger: Box<dyn ClientLogger>) {
    let level = match level {
        LogLevel::Trace => log::LevelFilter::Trace,
        LogLevel::Debug => log::LevelFilter::Debug,
        LogLevel::Info => log::LevelFilter::Info,
        LogLevel::Warn => log::LevelFilter::Warn,
        LogLevel::Error => log::LevelFilter::Error,
    };
    if LOGGER
        .set(CallbackLogger {
            callback: logger,
            level,
        })
        .is_ok()
        && log::set_logger(LOGGER.get().expect("initialized")).is_ok()
    {
        log::set_max_level(level);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use log::Log;
    use std::sync::{Arc, Mutex};
    struct Collect(Arc<Mutex<Vec<String>>>);
    impl ClientLogger for Collect {
        fn log(&self, _: LogLevel, message: String) {
            self.0.lock().unwrap().push(message);
        }
    }
    #[test]
    fn sdk_auth_and_http_traces_cannot_forward_selectors_to_client_logger() {
        let messages = Arc::new(Mutex::new(Vec::new()));
        let logger = CallbackLogger {
            callback: Box::new(Collect(messages.clone())),
            level: log::LevelFilter::Trace,
        };
        for target in ["muon::auth::from_fork", "hyper::client", "h2::client"] {
            logger.log(
                &log::Record::builder()
                    .level(log::Level::Info)
                    .target(target)
                    .args(format_args!("selector=PUBLIC-TEST-ONLY"))
                    .build(),
            );
        }
        logger.log(
            &log::Record::builder()
                .level(log::Level::Info)
                .target("protun_next::api")
                .args(format_args!("diagnostic"))
                .build(),
        );
        assert_eq!(
            *messages.lock().unwrap(),
            vec!["protun_next::api: diagnostic"]
        );
    }
}
