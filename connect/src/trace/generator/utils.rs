use std::{ffi::OsStr, process::Command};

pub fn quint_command() -> Command {
    if cfg!(windows) {
        Command::new("quint.cmd")
    } else {
        Command::new("quint")
    }
}

pub fn opt_arg<A>(cmd: &mut Command, name: &str, arg: Option<A>)
where
    A: AsRef<OsStr>,
{
    if let Some(arg) = arg {
        cmd.arg(name).arg(arg);
    }
}
