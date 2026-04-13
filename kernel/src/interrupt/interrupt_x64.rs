use crate::interrupt::interrupt_x64::acpi::AcpiHandlerImpl;
use crate::interrupt::interrupt_x64::apic::init_io_apic;
use crate::println;
use crate::reqs::{HHDM, RSDP};
use acpi::{AcpiTables, InterruptModel};
use core::ops::IndexMut;
use spin::Lazy;
use x86_64::structures::idt::{
    InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
};

mod acpi;
mod apic;
mod irq;

static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(handle_breakpoint);
    idt.double_fault.set_handler_fn(handle_double_fault);
    idt.page_fault.set_handler_fn(handle_page_fault);
    idt.alignment_check.set_handler_fn(handle_alignment_check);
    idt.general_protection_fault.set_handler_fn(handle_gpf);
    idt
});

pub fn init_interrupt_table() {
    disable_pic();
    let phys_mem_offset = HHDM.response().unwrap().offset as usize;
    let rsdp = RSDP.response().unwrap().address as usize - phys_mem_offset;
    let handler = AcpiHandlerImpl::new();
    let tables = unsafe { AcpiTables::from_rsdp(handler, rsdp).unwrap() };
    let platform = tables.platform_info().unwrap();
    let page_table = 
    if let InterruptModel::Apic(apic) = platform.interrupt_model {
        let io_apic_addr = apic.io_apics[0].address;
        unsafe {
            init_io_apic(io_apic_addr as _);
        }
    }

    IDT.load();
}

extern "x86-interrupt" fn handle_page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("EXCEPTION: PAGE FAULT: {error_code:?}\n{stack_frame:#?}");
}

extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT ENCOUNTERED\n{stack_frame:#?}");
}

extern "x86-interrupt" fn handle_double_fault(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{stack_frame:#?}");
}
extern "x86-interrupt" fn handle_alignment_check(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    println!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn handle_gpf(stack_frame: InterruptStackFrame, _error_code: u64) {
    println!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

fn disable_pic() {
    // Disable any unneeded PIC features, such as timer or keyboard to prevent it from firing interrupts
    use x86_64::instructions::port::Port;

    unsafe {
        Port::<u8>::new(0xA1).write(0xFF); // PIC2 (Slave PIC)
    }
}
