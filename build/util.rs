use std::{
    borrow::Borrow,
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{bail, Context};

pub fn is_cross_compiling() -> anyhow::Result<bool> {
    Ok(env::var("TARGET")? != env::var("HOST")?)
}

pub fn get_gcc_command_result(command: &mut Command) -> anyhow::Result<String> {
    let output = command
        .output()
        .context("Couldn't find target GCC executable.")?;
    if !output.status.success() {
        bail!("GCC failed with status: {:?}.", output.status)
    }
    Ok(String::from_utf8(output.stdout).context("Invalid output from GCC.")?)
}

pub fn run_command_or_fail<P, S>(dir: &str, cmd: P, args: &[S])
where
    P: AsRef<Path>,
    S: Borrow<str> + AsRef<OsStr>,
{
    let cmd = cmd.as_ref();
    let cmd = if cmd.components().count() > 1 && cmd.is_relative() {
        // If `cmd` is a relative path (and not a bare command that should be
        // looked up in PATH), absolutize it relative to `dir`, as otherwise the
        // behavior of std::process::Command is undefined.
        // https://github.com/rust-lang/rust/issues/37868
        PathBuf::from(dir)
            .join(cmd)
            .canonicalize()
            .expect("canonicalization failed")
    } else {
        PathBuf::from(cmd)
    };
    eprintln!(
        "Running command: \"{} {}\" in dir: {}",
        cmd.display(),
        args.join(" "),
        dir
    );
    let ret = Command::new(cmd).current_dir(dir).args(args).status();
    match ret.map(|status| (status.success(), status.code())) {
        Ok((true, _)) => {}
        Ok((false, Some(c))) => panic!("Command failed with error code {}", c),
        Ok((false, None)) => panic!("Command got killed"),
        Err(e) => panic!("Command failed with error: {}", e),
    }
}
