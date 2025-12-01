mod util;

pub(crate) use colored::Colorize;
pub(crate) use util::*;

pub(crate) const VERBOSITY: Option<&str> = option_env!("QUINT_VERBOSE");

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
        if VERBOSITY.unwrap_or("0") >= stringify!($level) {
            eprintln!("{}", crate::logger::indent!(3, $fmt $(,$args)*).dimmed().bright_white());
        }
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use success;
pub(crate) use title;
pub(crate) use trace;
