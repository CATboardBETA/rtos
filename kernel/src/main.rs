#![cfg_attr(not(test), no_std)]
#![no_main]

extern crate alloc;
use crate::gfx::Display;
use core::fmt::Write;
#[cfg(not(test))]
use core::panic::PanicInfo;
use limine::{request::*, BaseRevision, RequestsEndMarker, RequestsStartMarker};

mod gfx;
mod crt;

#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();

#[unsafe(link_section = ".requests")]
pub static BASE_REVISION: BaseRevision = BaseRevision::with_revision(6);
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER: FramebufferRequest = FramebufferRequest::new();
#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();
#[unsafe(link_section = ".requests")]
pub static BOOTLOADER: BootloaderInfoRequest = BootloaderInfoRequest::new();
#[unsafe(link_section = ".requests")]
pub static FIRMWARE: FirmwareTypeRequest = FirmwareTypeRequest::new();
#[unsafe(link_section = ".requests")]
pub static DATE: DateAtBootRequest = DateAtBootRequest::new();
#[unsafe(link_section = ".requests")]
pub static BOOT_TIME: BootloaderPerformanceRequest = BootloaderPerformanceRequest::new();
#[unsafe(link_section = ".requests")]
pub static HHDM: HhdmRequest = HhdmRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_ADDR: ExecutableAddressRequest = ExecutableAddressRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_FILE: ExecutableFileRequest = ExecutableFileRequest::new();
#[unsafe(link_section = ".requests")]
pub static EXEC_CMDLINE: ExecutableCmdlineRequest = ExecutableCmdlineRequest::new();
#[unsafe(link_section = ".requests")]
pub static DTB: DtbRequest = DtbRequest::new();
#[unsafe(link_section = ".requests")]
pub static RSDP: RsdpRequest = RsdpRequest::new();
#[unsafe(link_section = ".requests")]
pub static MP: MpRequest = MpRequest::new(0);
#[cfg(target_arch = "riscv64")]
#[unsafe(link_section = ".requests")]
pub static BSP_HARTID: BspHartidRequest = BspHartidRequest::new();
#[unsafe(link_section = ".requests")]
pub static MODULES: ModulesRequest = ModulesRequest::new();
#[cfg(target_arch = "x86_64")]
#[unsafe(link_section = ".requests")]
pub static KEEP_IOMMU: KeepIommuRequest = KeepIommuRequest::new();
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(65536);
#[unsafe(link_section = ".requests")]
pub static PAGING: PagingModeRequest = PagingModeRequest::PREFER_MAXIMUM;
#[unsafe(link_section = ".requests")]
pub static ENTRY: EntryPointRequest = EntryPointRequest::new(kmain);
#[unsafe(link_section = ".requests")]
pub static SMBIOS: SmbiosRequest = SmbiosRequest::new();
#[unsafe(link_section = ".requests")]
pub static EFI: EfiRequest = EfiRequest::new();
#[unsafe(link_section = ".requests")]
pub static EFI_MEMMAP: EfiMemmapRequest = EfiMemmapRequest::new();
#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();

mod alloc_handler;

#[unsafe(no_mangle)]
/// # Safety
/// I mean, it's the entry point. What could go wrong?
pub unsafe extern "C" fn kmain() -> ! {

    assert!(BASE_REVISION.is_supported());

    alloc_handler::GLOBAL.init();

    if let Some(framebuffer_response) = FRAMEBUFFER.response()
        && let Some(fb) = framebuffer_response.framebuffers().first()
    {
        let mut disp = Display {
            inner: fb,
            text_info: Default::default(),
        };
        disp.write_str("\nhihihi").unwrap();
    }

    hcf();
}

#[panic_handler]
#[cfg(not(test))]
fn rust_panic(_info: &PanicInfo) -> ! {
    hcf()
}

fn hcf() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            core::arch::asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            core::arch::asm!("wfi");
        }
    }
}
