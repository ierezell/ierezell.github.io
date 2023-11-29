use std::process::Command;
fn main() {
    Command::new("sh")
        .arg("-c")
        .arg("trunk")
        .arg("serve")
        .output()
        .expect("failed to launch server");
}
