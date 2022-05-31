use std::io::Write;

use env::{Builder, fmt};
pub use env_logger as env;
pub use log as log;


pub fn setup_default_logger() {
    Builder::new()
    .format(|buf, record| {
        let level = record.level();
        let target = record.target();
        let args = record.args();
        let file = record.file().unwrap_or("unknown");
        let line = record.line().unwrap_or(0);

        (|buf : &mut fmt::Formatter, args : std::fmt::Arguments<'_>| {
            let written_chars = args.to_string().len();
            buf.write_fmt(args)?;

            for _ in 0..(144 - written_chars) {
                write!(buf, " ")?;
            }
            writeln!(buf, "{}:{}", file, line)
        })(buf, format_args!("[{}][{}] - {}", level, target, args))


    })
    .filter_level(log::LevelFilter::Info)
    .init();
}

#[macro_export]
macro_rules! trace {
    ($( $args:expr ),*) => {
        $crate::log::log::trace!($($args), *);
    };
}

#[macro_export]
macro_rules! info {
    ($( $args:expr ),*) => {
        $crate::log::log::info!($($args), *);
    };
}

#[macro_export]
macro_rules! error {
    ($( $args:expr ),*) => {
        $crate::log::log::error!($($args), *);
    };
}

#[macro_export]
macro_rules! warn {
    ($( $args:expr ),*) => {
        $crate::log::log::warn!($($args), *);
    };
}

#[macro_export]
macro_rules! fatal {
    ($( $args:expr ),*) => {
        error!($($args), *);
        panic!($($args), *);
    };
}

#[macro_export]
macro_rules! t_trace {
    ($( $args:expr ),*) => {
        
        $crate::log::log::trace!(target: { use crate::IDENTIFIER; crate::IDENTIFIER }, $($args), *);
    };
}

#[macro_export]
macro_rules! t_info {
    ($( $args:expr ),*) => {
        $crate::log::log::info!(target: { use crate::IDENTIFIER; crate::IDENTIFIER },$($args), *);
    };
}

#[macro_export]
macro_rules! t_error {
    ($( $args:expr ),*) => {
        $crate::log::log::error!(target: { use crate::IDENTIFIER; crate::IDENTIFIER },$($args), *);
    };
}

#[macro_export]
macro_rules! t_warn {
    ($( $args:expr ),*) => {
        $crate::log::log::warn!(target: { use crate::IDENTIFIER; crate::IDENTIFIER },$($args), *);
    };
}

#[macro_export]
macro_rules! t_fatal {
    ($( $args:expr ),*) => {{
        $crate::log::log::error!(target: { use crate::IDENTIFIER; crate::IDENTIFIER }, $($args), *);
        panic!($($args), *)
    };
}}