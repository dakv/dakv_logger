#[macro_use(o)]
extern crate slog;

use slog_term::PlainDecorator;
use slog::{Logger, Drain, Never, FilterLevel};
use slog_async::Async;
use slog_envlogger::{LogBuilder, EnvLogger};
use crate::drain::DaKVFormatter;
use std::sync::Mutex;
use slog_scope;

mod drain;

/// ```no_run
/// use dakv_logger::prelude::*;
/// use dakv_logger::set_logger_level;
///
/// fn main() {
///     let _logger = set_logger_level(true, None);
///     info!("test");
/// }
/// ```
// todo rotate
pub mod prelude {
    pub use slog_scope::{crit, debug, error, info, trace, warn};
}


pub fn set_logger_level(is_async: bool, chan_size: Option<usize>) -> slog_scope::GlobalLoggerGuard {
    let p = PlainDecorator::new(std::io::stdout());
    let format = DaKVFormatter::new(p).fuse();
    let env_drain = get_env_log(format, FilterLevel::Info);
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
    where D: Drain<Err=Never, Ok=()> + Send + 'static {
    let mut async_builder = Async::new(drain);
    if let Some(s) = chan_size {
        async_builder = async_builder.chan_size(s)
    }
    async_builder.build()
}

fn get_env_log<D>(drain: D, filter_level: FilterLevel) -> EnvLogger<D>
    where D: Drain<Err=Never, Ok=()> + Send + 'static {
    let mut env_log_builder = LogBuilder::new(drain);
    env_log_builder = env_log_builder.filter(None, filter_level);

    if let Ok(l) = std::env::var("RUST_LOG") {
        env_log_builder = env_log_builder.parse(&l);
    }
    env_log_builder.build()
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use super::set_logger_level;

    #[test]
    fn test_async_log() {
        let _l = set_logger_level(true, None);
        warn!("da");
    }

    #[test]
    fn test_log() {
        let _l = set_logger_level(false, Some(1 << 10));
        info!("test");
    }
}