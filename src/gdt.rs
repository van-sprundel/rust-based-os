/// Global Descriptor Table
// see https://www.amd.com/system/files/TechDocs/24593.pdf
// table 8-3 shows all double fault conditions
// also https://wiki.osdev.org/Exceptions

use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use lazy_static::lazy_static;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};

pub fn init() {
    use x86_64::instructions::segmentation::set_cs; // code selector
    use x86_64::instructions::tables::load_tss; // tss
    GDT.0.load();
    unsafe {
        set_cs(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
    }
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable,Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors { code_selector, tss_selector })
    };
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0; // any ist would work

// our tss handler
// lazy_static is required since the const is too much work
// for rust to init at compile time
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}
