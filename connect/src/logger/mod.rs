mod util;

pub(crate) use colored::Colorize;
pub(crate) use util::*;

use std::sync::OnceLock;

static VERBOSITY: OnceLock<u8> = OnceLock::new();

pub(crate) fn verbosity() -> u8 {
    *VERBOSITY.get_or_init(|| match option_env!("QUINT_VERBOSE") {
        Some("0") | None => 0,
        Some("1") => 1,
        Some("2") => 2,
        Some(n) => panic!("Invalid verbosity level: {}", n),
    })
}

macro_rules! title {
    ($fmt:literal $(, $args:expr)*) => {
        eprint!("{}", "== ".bold());
        eprintln!("{}", format!($fmt $(,$args)*).bold());
    };
}

macro_rules! info {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::indent!(3, $fmt $(,$args)*));
    };
}

macro_rules! success {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::indent!(3, $fmt $(,$args)*).bold().green());
    };
}

macro_rules! error {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::indent!(3, $fmt $(,$args)*).bold().red());
    };
}

macro_rules! trace {
    ($level:literal, $fmt:literal $(, $args:expr)*) => {
        if crate::logger::verbosity() >= $level {
            eprintln!("{}", crate::logger::indent!(3, $fmt $(,$args)*).dimmed().bright_white());
        }
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use success;
pub(crate) use title;
pub(crate) use trace;
