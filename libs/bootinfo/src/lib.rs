#![no_std]

use arrayvec::ArrayVec;
use cpu::paging::{self, PML4Entry, PDPEntry, PDEntry, PTEntry};
use cpu::{PhysAddr, PhysSlice, paging::Megapage};
use uefi::table::{Runtime, boot::MemoryDescriptor};
use uefi::prelude::*;
use uart_16550::SerialPort;

#[repr(C, align(4096))]
pub struct Bootinfo {
    pub paging_root: paging::Table<PML4Entry>,
    pub pdp: paging::Table<PDPEntry>,
    pub pd: paging::Table<PDEntry>,
    pub page_table: paging::Table<PTEntry>,

    pub this: PhysAddr<Bootinfo>,
    pub kernel_pslice: PhysSlice<u8>,

    pub buf: [u8; 8192],
    pub uefi_meminfo: ArrayVec<[MemoryDescriptor; 192]>,
    pub uefi_systable: Option<SystemTable<Runtime>>,
    pub serial: Option<SerialPort>,
}

impl Bootinfo {
    pub const fn new() -> Self {
        Self {
            paging_root:    paging::Table::new(),
            pdp:            paging::Table::new(),
            pd:             paging::Table::new(),
            page_table:     paging::Table::new(),

            this:           PhysAddr::null(),
            kernel_pslice:  PhysSlice::null(),

            buf:            [0u8; 8192],
            uefi_meminfo:   ArrayVec::new(),
            uefi_systable:  None,
            serial:         None,
        }
    }

    /// # Safety
    /// * Technically this struct is self-referential, 
    /// so we should use Pin, but for simplicity sake we don't.
    /// * Memory must be identity-mapped.
    /// * Kernel base must be 0xffff_ffff_c000_0000.
    pub unsafe fn map_kernel(
        &mut self,
        text: PhysSlice<Megapage>,
        rodata: PhysSlice<Megapage>, 
        data: PhysSlice<Megapage>,
    ) {
        let base = 0xffff_ffff_c000_0000u64;
        /* What we want to do here is to map kernel with 2M pages and bootinfo
         * with normal 4K pages.
         * It is assumed that by this time memory is identity mapped (so that
         * remapping `self` is possible */
    }

    /// # Safety
    /// * `entry` must be a valid function pointer, that can be
    /// called as page fault handler.
    /// * Absolutely no safety otherwise
    pub unsafe fn page_fault_jump_trick(entry: u64) -> ! {
        let page_fault_vector_number = 14;
        unreachable!();
    }
}
