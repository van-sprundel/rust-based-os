// see https://wiki.osdev.org/Exceptions
// we need interrupts in case bad commands gets run
// e.g. writing to a read-only area
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;

pub fn init_idt() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler); // breakpoint handler, like an IDEs debug mode
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler) // similar to a catch statement e.g. writing to invalid access
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

// the handler in the idt looks like this
// extern "x86-interrupt" fn(_: &mut InterruptStackFrame);
// so we use this for out handlers
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame); // we don't need to return a double fault since the OS shouldn't continue on page fault
}