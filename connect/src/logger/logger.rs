use colored::Colorize;

macro_rules! title {
    ($fmt:literal $(, $args:expr)*) => {
        eprint!("{}", "== ".bold());
        eprintln!("{}", format!($fmt $(,$args)*).bold());
    };
}

macro_rules! info {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::util::indent!(2, $fmt $(,$args)*));
    };
}

macro_rules! success {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::util::indent!(2, $fmt $(,$args)*).bold().green());
    };
}

macro_rules! error {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::util::indent!(2, $fmt $(,$args)*).bold().red());
    };
}

macro_rules! trace {
    ($fmt:literal $(, $args:expr)*) => {
        eprintln!("{}", crate::logger::util::indent!(2, $fmt $(,$args)*).dimmed().bright_white());
    };
}

pub(crate) use error;
pub(crate) use info;
pub(crate) use success;
pub(crate) use title;
pub(crate) use trace;
