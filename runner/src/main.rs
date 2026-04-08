use std::process::Command;

#[cfg(not(feature="aarch"))]
fn main() {
    Command::new("qemu-system-x86_64").args([
        "-cdrom",
        env!("ISO"),
        "-m",
        "4G",
        "-accel","tcg"
    ]).status().unwrap();
}