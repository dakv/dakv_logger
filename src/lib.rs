#[macro_use(o)]
extern crate slog;

use crate::drain::DaKvFormatter;
use slog::{Drain, FilterLevel, Logger, Never};
use slog_async::Async;
use slog_envlogger::{EnvLogger, LogBuilder};
use slog_term::PlainDecorator;
use std::io;
use std::sync::Mutex;

mod drain;

/// ```
/// use slog::{crit, debug, error, info, trace, warn};
/// use dakv_logger::set_logger_level;
///
/// let logger = set_logger_level(std::io::stdout(), true, None);
/// info!(logger, "test");
/// ```
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

pub fn set_logger_level<W: io::Write + Send + Sync + 'static>(
    w: W,
    is_async: bool,
    chan_size: Option<usize>,
) -> Logger {
    let p = PlainDecorator::new(w);
    let format = DaKvFormatter::new(p).fuse();
    let env_drain = get_env_log(format, __slog_static_max_level());
    let logger = if is_async {
        let l = gen_async_log(env_drain, chan_size).fuse();
        Logger::root(l.fuse(), o!())
    } else {
        let l = Mutex::new(env_drain);
        Logger::root(l.fuse(), o!())
    };
    logger
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_async_log() {
        use super::set_logger_level;
        use slog::{crit, debug, error, info, trace, warn};
        let log = set_logger_level(std::io::stdout(), true, None);
        crit!(log, "test");
        info!(log, "test");
        warn!(log, "test");
        error!(log, "test");
        debug!(log, "test");
        trace!(log, "test");
    }

    #[test]
    fn test_log() {
        use super::set_logger_level;
        use slog::{crit, debug, error, info, trace, warn};
        let log = set_logger_level(std::io::stdout(), false, None);

        crit!(log, "test");
        info!(log, "test");
        warn!(log, "test");
        error!(log, "test");
        debug!(log, "test");
        trace!(log, "test");
    }
}
