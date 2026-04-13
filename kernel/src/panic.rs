use crate::{hcf, println};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};

static PANICKED: AtomicBool = AtomicBool::new(false);

#[panic_handler]
#[cfg(not(test))]
fn rust_panic(info: &PanicInfo) -> ! {
    if !PANICKED.swap(true, Ordering::AcqRel) {
        println!("{info}");
    }
    // otherwise, we already panicked once, probably from the println invocation. So we just don't
    // do anything to avoid a double/triple fault
    hcf()
}
