use std::env;
use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use swayipc_types::{Error, Fallible};
use tokio::process::Command;

pub async fn get_socketpath() -> Fallible<PathBuf> {
    if let Some(socketpath) = env::var_os("I3SOCK") {
        Ok(socketpath.into())
    } else if let Some(socketpath) = env::var_os("SWAYSOCK") {
        Ok(socketpath.into())
    } else if let Ok(socketpath) = spawn("i3").await {
        Ok(socketpath)
    } else if let Ok(socketpath) = spawn("sway").await {
        Ok(socketpath)
    } else {
        Err(Error::SocketNotFound)
    }
}

async fn spawn(wm: &str) -> Fallible<PathBuf> {
    let mut buf = Command::new(wm)
        .arg("--get-socketpath")
        .output()
        .await?
        .stdout;
    buf.pop();
    Ok(OsString::from_vec(buf).into())
}
