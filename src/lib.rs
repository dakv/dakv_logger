#[macro_use(o)]
extern crate slog;

use crate::drain::DaKVFormatter;
use once_cell::sync::Lazy;
use slog::{Discard, Drain, FilterLevel, Logger, Never};
use slog_async::Async;
use slog_envlogger::{EnvLogger, LogBuilder};
use slog_term::PlainDecorator;
use std::sync::Mutex;

mod drain;

/// ```
/// use dakv_logger::prelude::*;
/// use dakv_logger::set_logger_level;
///
/// let _logger = set_logger_level(true, None);
/// info!("test");
/// ```
// todo rotate
pub mod prelude {
    pub use slog_scope::{crit, debug, error, info, trace, warn};
}

#[allow(unknown_lints)]
#[allow(clippy::inline_always)]
#[inline(always)]
pub fn __slog_static_max_level() -> FilterLevel {
    if !cfg!(debug_assertions) {
        if cfg!(feature = "release_max_level_off") {
            return FilterLevel::Off;
        } else if cfg!(feature = "release_max_level_error") {
            return FilterLevel::Error;
        } else if cfg!(feature = "release_max_level_warn") {
            return FilterLevel::Warning;
        } else if cfg!(feature = "release_max_level_info") {
            return FilterLevel::Info;
        } else if cfg!(feature = "release_max_level_debug") {
            return FilterLevel::Debug;
        } else if cfg!(feature = "release_max_level_trace") {
            return FilterLevel::Trace;
        }
    }
    if cfg!(feature = "max_level_off") {
        FilterLevel::Off
    } else if cfg!(feature = "max_level_error") {
        FilterLevel::Error
    } else if cfg!(feature = "max_level_warn") {
        FilterLevel::Warning
    } else if cfg!(feature = "max_level_info") {
        FilterLevel::Info
    } else if cfg!(feature = "max_level_debug") {
        FilterLevel::Debug
    } else if cfg!(feature = "max_level_trace") {
        FilterLevel::Trace
    } else if !cfg!(debug_assertions) {
        FilterLevel::Info
    } else {
        FilterLevel::Debug
    }
}

pub fn set_logger_level(
    is_async: bool,
    chan_size: Option<usize>,
) -> slog_scope::GlobalLoggerGuard {
    let p = PlainDecorator::new(std::io::stdout());
    let format = DaKVFormatter::new(p).fuse();
    let env_drain = get_env_log(format, __slog_static_max_level());
    let logger = if is_async {
        let l = gen_async_log(env_drain, chan_size).fuse();
        Logger::root(l.fuse(), o!())
    } else {
        let l = Mutex::new(env_drain);
        Logger::root(l.fuse(), o!())
    };
    slog_scope::set_global_logger(logger)
}

fn gen_async_log<D>(drain: D, chan_size: Option<usize>) -> Async
where
    D: Drain<Err = Never, Ok = ()> + Send + 'static,
{
    let mut async_builder = Async::new(drain);
    if let Some(s) = chan_size {
        async_builder = async_builder.chan_size(s)
    }
    async_builder.build()
}

fn get_env_log<D>(drain: D, filter_level: FilterLevel) -> EnvLogger<D>
where
    D: Drain<Err = Never, Ok = ()> + Send + 'static,
{
    let mut env_log_builder = LogBuilder::new(drain);
    env_log_builder = env_log_builder.filter(None, filter_level);

    if let Ok(l) = std::env::var("RUST_LOG") {
        env_log_builder = env_log_builder.parse(&l);
    }
    env_log_builder.build()
}

pub fn make_logger_static_for_testing() {
    Lazy::force(&TESTING_GUARD);
}

static TESTING_GUARD: Lazy<slog_scope::GlobalLoggerGuard> = Lazy::new(|| {
    let logger = Logger::root(Discard, o!());
    slog_scope::set_global_logger(logger)
});

#[cfg(test)]
mod tests {
    use crate::make_logger_static_for_testing;

    #[test]
    fn test_async_log() {
        use super::prelude::{crit, debug, error, info, trace, warn};
        use super::set_logger_level;
        let _l = set_logger_level(true, None);
        crit!("test");
        info!("test");
        warn!("test");
        error!("test");
        debug!("test");
        trace!("test");
    }

    #[test]
    fn test_log() {
        use super::prelude::{crit, debug, error, info, trace, warn};
        make_logger_static_for_testing();

        crit!("test");
        info!("test");
        warn!("test");
        error!("test");
        debug!("test");
        trace!("test");
    }
}
