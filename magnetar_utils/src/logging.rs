use std::io::Write;
use termcolor::*;

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_log {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Information);
        println!($($args), *)}
    };
}

#[macro_export]
macro_rules! log {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Information);
        println!($($args), *)}
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_log {
    ($( $args:expr ),*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_warn {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Warning);
        println!($($args), *)}
    };
}

#[macro_export]
macro_rules! warn {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Warning);
        println!($($args), *)}
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_warn {
    ($( $args:expr ),*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_error {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Error);
        println!($($args), *)}
    };
}

#[macro_export]
macro_rules! error {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Error);
        println!($($args), *)}
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_error {
    ($( $args:expr ),*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_success {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Success);
        println!($($args), *)}
    };
}

#[macro_export]
macro_rules! success {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Success);
        println!($($args), *)}
    };
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_success {
    ($( $args:expr ),*) => {};
}

#[macro_export]
#[cfg(debug_assertions)]
macro_rules! debug_failure {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Failure);
        panic!($($args), *)}
    }
}

#[macro_export]
macro_rules! failure {
    ($( $args:expr ),*) => {{
        $crate::logging::write_tag($crate::logging::LoggingLevel::Failure);
        panic!($($args), *)}
    }
}

#[macro_export]
#[cfg(not(debug_assertions))]
macro_rules! debug_failure {
    ($( $args:expr ),*) => {};
}

#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
pub enum LoggingLevel {
    Information = 0,
    Success = 1,
    Warning = 2,
    Error = 3,
    Failure = 4,
}

pub fn write_tag(log_level: LoggingLevel) {
    let color;
    let level;

    match log_level {
        LoggingLevel::Success => {
            level = " SUCCESS ";
            color = Color::Green;
        }
        LoggingLevel::Information => {
            level = "   INFO  ";
            color = Color::Cyan;
        }
        LoggingLevel::Warning => {
            level = " WARNING ";
            color = Color::Yellow;
        }
        LoggingLevel::Error => {
            level = "  ERROR  ";
            color = Color::Red;
        }
        LoggingLevel::Failure => {
            level = " FAILURE ";
            color = Color::Blue;
        }
    }

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))
        .unwrap();
    write!(&mut stdout, "[").unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
        .unwrap();
    write!(&mut stdout, "{}", level).unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(true))
        .unwrap();
    write!(&mut stdout, "] ").unwrap();
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)).set_bold(false))
        .unwrap();
}
