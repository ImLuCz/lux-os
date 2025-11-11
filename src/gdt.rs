/* gdt is Global Descriptor Table */
use lazy_static::lazy_static;
use x86_64::VirtAddr;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

/*
* basically double fault stack -> IST (has all the stacks for the different interrupts) -> TSS
* (needs to go through this, legacy structure) -> GDT (tells the cput that it should use that certain stack
* for that certain exception handler)
*/

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Assign the address of double fault stack to 0th entry in IST
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // TODO: storing stack this way sucks, needs proper memory management
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}

// Add code segment and tss in the gdt so the cpu knows to use it
// TSS uses gdt for historical reasons so there is no better way to do this
lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector); // reload code segment register
        load_tss(GDT.1.tss_selector);
    }
}
