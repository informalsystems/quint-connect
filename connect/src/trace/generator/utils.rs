use std::{ffi::OsStr, process::Command};

pub fn quint_command() -> Command {
    #[cfg(windows)]
    {
        // Wraps the quint invocation in cmd.exe /C on Windows 
        // (needed because npm installs quint as a .cmd batch file that Rust's Command can't execute directly)
        let mut cmd = Command::new("cmd");
        cmd.args(["/C", "quint"]);
        cmd
    }
    #[cfg(not(windows))]
    {
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
