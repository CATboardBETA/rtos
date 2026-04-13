#![cfg_attr(any(target_os = "none"), no_std)]
#[cfg(not(target_os = "none"))]
pub mod x {
    use std::process::Command;

    #[cfg(not(feature = "aarch"))]
    pub fn main() {
        Command::new("qemu-system-x86_64")
            .args([
                "-cdrom",
                env!("ISO"),
                "-m",
                "2G",
                "-accel",
                "tcg",
                "-drive",
                "if=pflash,unit=0,format=raw,file=runner/OVMF_X64.fd",
            ])
            .status()
            .unwrap();
    }

    #[cfg(feature = "aarch")]
    pub fn main() {
        Command::new("qemu-system-aarch64")
            .args([
                "-machine",
                "virt",
                "-cpu",
                "cortex-a57",
                "-m",
                "2G",
                "-bios",
                "runner/OVMF_AARCH64.fd",
                "-drive",
                concat!("if=virtio,format=raw,file=", env!("ISO")),
                "-monitor",
                "stdio",
            ])
            .status()
            .unwrap();
    }
}
#[cfg(not(target_os = "none"))]
pub use x::main;

#[cfg(target_os = "none")]
fn main() {
    // this shouldn't happen
}
