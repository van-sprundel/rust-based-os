// see https://wiki.osdev.org/Exceptions
// we need interrupts in case an illegal commands gets run
// e.g. writing to a read-only area
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// the handler in the idt looks like this
// extern "x86-interrupt" fn(_: &mut InterruptStackFrame);
// so we use this in our handler
extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame)
}