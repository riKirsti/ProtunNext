use std::sync::OnceLock;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
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
        metadata.level() <= self.level && !metadata.target().starts_with("muon")
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
