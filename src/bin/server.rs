use std::process::{Command, Stdio};

/// Launches the server in a child process and waits for it.
/// StdIO is inherited from the current process so all the output and errors will be printed to the current terminal.
/// This is only a small util to spawn the server from cargo to uniformize commands but still rely on trunk.
/// Trunk as to be installed.
fn main() {
    Command::new("sh")
        .arg("-c")
        .arg("trunk serve")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to launch server");
}
