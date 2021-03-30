use slog::{Drain, Level, OwnedKVList, Record};
use slog_term::{Decorator, RecordDecorator};
use std::io;

pub struct DaKvFormatter<D: Decorator> {
    dec: D,
}

fn level_to_str(level: Level) -> &'static str {
    match level {
        Level::Info => "I",
        Level::Debug => "D",
        Level::Warning => "W",
        Level::Error => "E",
        Level::Trace => "T",
        Level::Critical => "C",
    }
}

impl<D: Decorator> DaKvFormatter<D> {
    pub fn new(dec: D) -> Self {
        DaKvFormatter { dec }
    }
}

impl<D: Decorator> Drain for DaKvFormatter<D> {
    type Ok = ();
    type Err = io::Error;

    /// Format sample:
    /// 2020-03-02T08:51:59.749075+08:00 I src/server.rs 12 hello
    fn log(&self, record: &Record<'_>, kv: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        self.dec.with_record(record, kv, |dec| {
            let t = chrono::Local::now();
            write!(dec, "{} ", t.to_rfc3339())?;
            write!(dec, "{} ", level_to_str(record.level()))?;
            write!(dec, "{} {}", record.file(), record.line())?;
            write!(dec, " {}", record.msg())?;
            finish_line(dec)?;
            Ok(())
        })
    }
}

fn finish_line(dec: &mut dyn RecordDecorator) -> io::Result<()> {
    dec.start_whitespace()?;
    writeln!(dec)?;
    dec.flush()
}
