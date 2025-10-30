#![forbid(unsafe_code)]

use flexi_logger::{AdaptiveFormat, DeferredNow};
use log::{Level, Record};
#[cfg(feature = "colors")]
use owo_colors::Style;
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "colors")]
pub const ADAPTIVE_PRETTY_FORMAT: AdaptiveFormat =
    AdaptiveFormat::Custom(pretty_env_logger_format, pretty_env_logger_color_format);

// Note: flexi_logger exports a "style" function, but
// that's not the greatest style for them to leak the abstracted type.
// Let's use our dependency instead.
// We should depend on flexi_logger's dependency if this was to be published independently.
#[cfg(feature = "colors")]
#[inline]
fn style(level: Level) -> Style {
    match level {
        Level::Error => Style::new().red().bold(),
        Level::Warn => Style::new().yellow(),
        Level::Info => Style::new().green(),
        Level::Debug => Style::new().blue(),
        Level::Trace => Style::new().magenta(),
    }
}

// Adapted from https://github.com/seanmonstar/pretty-env-logger/blob/master/src/lib.rs
#[cfg(feature = "colors")]
pub fn pretty_env_logger_color_format(
    f: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    let target = record.target();
    let max_width = max_target_width(target);
    let ls = style(level);
    let ts = Style::new().bold();

    write!(
        f,
        " {} {} {} {}",
        ls.style(format_args!("{level: <5}")),
        ts.style(format_args!("{target: <max_width$}")),
        ls.style(">"),
        record.args(),
    )
}

pub fn pretty_env_logger_format(
    f: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    // This copy-paste seems to be expected
    let level = record.level();
    let target = record.target();
    let max_width = max_target_width(target);

    write!(f, " {level: <5} {target: <max_width$} > {}", record.args(),)
}

const MAX_MODULE_WIDTH_CLAMP: usize = 64;
static MAX_MODULE_WIDTH: AtomicUsize = AtomicUsize::new(0);

fn max_target_width(target: &str) -> usize {
    let max_width = MAX_MODULE_WIDTH.load(Ordering::Relaxed);
    let target_len = target.len();
    if max_width < target_len && target_len < MAX_MODULE_WIDTH_CLAMP {
        MAX_MODULE_WIDTH.store(target_len, Ordering::Relaxed);
        target_len
    } else {
        std::cmp::max(max_width, target_len)
    }
}
