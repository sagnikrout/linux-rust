//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/pgtable.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 Regents of the University of California
//

// Leave 2GB for kernel and BPF at the end of the address space

// Number of entries in the page global directory

// Number of entries in the page table

//
// Half of the kernel address space (1/4 of the entries of the page global
// directory) is for the direct mapping.
//

// Modules always live before the kernel

// This is used to define the end of the KASAN shadow region

//
// Roughly size the vmemmap space to be large enough to fit enough
// struct pages to map half the virtual address space. Then
// position vmemmap directly below the VMALLOC region.
//
pub const VA_BITS_SV32: c_int = 32;

pub const VA_BITS_SV39: c_int = 39;
pub const VA_BITS_SV48: c_int = 48;
pub const VA_BITS_SV57: c_int = 57;

//
// Define vmemmap for pfn_to_page & page_to_pfn calls. Needed if kernel
// is configured with CONFIG_SPARSEMEM_VMEMMAP enabled.
//

// Needed to limit get_free_mem_region()

// DIRECT_MAP_PHYSMEM_END is not limited by VA space assignment in this case

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_alloc_ops {
    pub pa): *mut *mut *mut pte_t (get_pte_virt)(phys_addr_t,
    pub va): *mut *mut phys_addr_t (alloc_pte)(uintptr_t,
    pub pa): *mut *mut *mut pmd_t (get_pmd_virt)(phys_addr_t,
    pub va): *mut *mut phys_addr_t (alloc_pmd)(uintptr_t,
    pub pa): *mut *mut *mut pud_t (get_pud_virt)(phys_addr_t,
    pub va): *mut *mut phys_addr_t (alloc_pud)(uintptr_t,
    pub pa): *mut *mut *mut p4d_t (get_p4d_virt)(phys_addr_t,
    pub va): *mut *mut phys_addr_t (alloc_p4d)(uintptr_t,

}

// Number of PGD entries that a user-mode program can use

// Page protection bits

//
// Checking for _PAGE_LEAF is needed too because:
// When splitting a THP, split_huge_page() will temporarily clear
// the present bit, in this situation, pmd_present() and
// pmd_trans_huge() still needs to return true.
//

extern "C" {
    pub fn pmd_present(_PAGE_LEAF: pmd) && (pmd_val(pmd) &) -> return;
}
extern "C" {
    pub fn __pgd(prot_val: (pfn << _PAGE_PFN_SHIFT) |) -> return;
}
extern "C" {
    pub fn __page_val_to_pfn(_arg: pgd_val(pgd)) -> return;
}
extern "C" {
    pub fn pfn_to_page(_arg: __page_val_to_pfn(pmd_val(pmd))) -> return;
}
extern "C" {
    pub fn __pte(_arg: pmd_val(pmd)) -> return;
}
extern "C" {
    pub fn __pte(_arg: pud_val(pud)) -> return;
}

extern "C" {
    pub fn riscv_has_extension_likely(_arg: RISCV_ISA_EXT_SVNAPOT) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_NAPOT: (pte_val(pte) & napot_mask) | napot_bit |) -> return;
}

// Yields the page frame number (PFN) of a page table entry

// Constructs a page table entry
extern "C" {
    pub fn __pte(prot_val: (pfn << _PAGE_PFN_SHIFT) |) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: pte_val(pfn_pte(pfn, pte_val(pte): __pgprot(0))) ^) -> return;
}

extern "C" {
    pub fn pte_present(_PAGE_LEAF: pte) && (pte_val(pte) &) -> return;
}
// static inline pte_t pte_rdprotect(pte_t pte)
extern "C" {
    pub fn __pte((_PAGE_READ): (pte_val(pte) & ~(_PAGE_WRITE)) |) -> return;
}

extern "C" {
    pub fn pte_wrprotect(_PAGE_UFFD): __pte(pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_UFFD): pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SWP_UFFD: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_SWP_UFFD): pte_val(pte) &) -> return;
}

// static inline pte_t pte_mkread(pte_t pte)
extern "C" {
    pub fn pte_mkwrite(pte: pte_t, vma: *mut vm_area_struct) -> pte_t;
}

extern "C" {
    pub fn __pte(_PAGE_WRITE: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_WRITE: (pte_val(pte) & ~(_PAGE_LEAF)) |) -> return;
}
// static inline pte_t pte_mkexec(pte_t pte)
extern "C" {
    pub fn __pte(_PAGE_SOFT_DIRTY: pte_val(pte) | _PAGE_DIRTY |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_DIRTY): pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_ACCESSED: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_ACCESSED): pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SPECIAL: pte_val(pte) |) -> return;
}

extern "C" {
    pub fn __pte(_PAGE_SOFT_DIRTY: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_SOFT_DIRTY): pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SWP_SOFT_DIRTY: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~(_PAGE_SWP_SOFT_DIRTY): pte_val(pte) &) -> return;
}

extern "C" {
    pub fn pte_protnone(_arg: pmd_pte(pmd)) -> return;
}

// Modify page protection bits
extern "C" {
    pub fn __pte(newprot_val: (pte_val(pte) & _PAGE_CHG_MASK) |) -> return;
}

// Commit new configuration to MMU hardware
//
// Svvptc guarantees that the new valid pte will be visible within
// a bounded timeframe, so when the uarch does not cache invalid
// entries, we don't have to do anything.
//
// The kernel assumes that TLBs don't cache invalid entries, but
// in RISC-V, SFENCE.VMA specifies an ordering constraint, not a
// cache flush; it is necessary even after writing invalid entries.
// Relying on flush_tlb_fix_spurious_fault would suffice, but
// the extra traps reduce performance.  So, eagerly SFENCE.VMA.
//

extern "C" {
    pub fn pte_val(pte_val(pte_b: pte_a) ==) -> return;
}
//
// Certain architectures need to do special things when PTEs within
// a page table are directly modified.  Thus, the following hook is
// made available.
//
extern "C" {
    pub fn flush_icache_pte(mm: *mut mm_struct, pte: pte_t);
}

//
// ptep_set_wrprotect can be called for shadow stack ranges too.
// shadow stack memory is XWR = 010 and thus clearing _PAGE_WRITE will lead to
// encoding 000b which is wrong encoding with V = 1. This should lead to page fault
// but we dont want this wrong configuration to be set in page tables.
//
// This comment is borrowed from x86, but applies equally to RISC-V:
//
// Clearing the accessed bit without a TLB flush
// doesn't cause data corruption. [ It could cause incorrect
// page aging and the (mistaken) reclaim of hot pages, but the
// chance of that should be relatively low. ]
//
// So as a performance optimization don't flush the TLB when
// clearing the accessed bit, it will eventually be flushed by
// a context switch or a VM operation anyway. [ In the rare
// event of it not getting flushed for a long time the delay
// shouldn't really matter because there's no real memory
// pressure for swapout to react to. ]
//
extern "C" {
    pub fn ptep_test_and_clear_young(_arg: vma, _arg: address, _arg: ptep) -> return;
}

extern "C" {
    pub fn __pgprot(~_PAGE_EXEC: pgprot_val(_prot) &) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: prot) -> return;
}

extern "C" {
    pub fn __pgprot(_arg: prot) -> return;
}

//
// Both Svade and Svadu control the hardware behavior when the PTE A/D bits need to be set. By
// default the M-mode firmware enables the hardware updating scheme when only Svadu is present in
// DT.
//

extern "C" {
    pub fn riscv_has_extension_unlikely(_arg: RISCV_ISA_EXT_SVADU) -> return;
}
//
// THP functions
//
extern "C" {
    pub fn __pmd(_arg: pte_val(pte)) -> return;
}
extern "C" {
    pub fn __pud(_arg: pte_val(pte)) -> return;
}
extern "C" {
    pub fn __pmd(~(_PAGE_PRESENT|_PAGE_PROT_NONE): pmd_val(pmd) &) -> return;
}

extern "C" {
    pub fn pte_pgprot(_arg: pmd_pte(pmd)) -> return;
}

extern "C" {
    pub fn pte_pgprot(_arg: pud_pte(pud)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_modify(pmd_pte(pmd), _arg: newprot)) -> return;
}

extern "C" {
    pub fn pte_write(_arg: pmd_pte(pmd)) -> return;
}

extern "C" {
    pub fn pte_write(_arg: pud_pte(pud)) -> return;
}

extern "C" {
    pub fn pte_dirty(_arg: pmd_pte(pmd)) -> return;
}

extern "C" {
    pub fn pte_young(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_user(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkold(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkyoung(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t;
}

extern "C" {
    pub fn pte_pmd(_arg: pte_mkwrite_novma(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn __pmd(_PAGE_WRITE: (pmd_val(pte) & ~(_PAGE_LEAF)) |) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_wrprotect(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkclean(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkdirty(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn pte_special(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkspecial(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn pte_special(_arg: pud_pte(pud)) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkspecial(pud_pte(pud))) -> return;
}

extern "C" {
    pub fn pte_uffd(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mkuffd(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_clear_uffd(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_swp_uffd(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_swp_mkuffd(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_swp_clear_uffd(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn pte_soft_dirty(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_mksoft_dirty(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_clear_soft_dirty(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn pte_swp_soft_dirty(_arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_swp_mksoft_dirty(pmd_pte(pmd))) -> return;
}
extern "C" {
    pub fn pte_pmd(_arg: pte_swp_clear_soft_dirty(pmd_pte(pmd))) -> return;
}

extern "C" {
    pub fn __set_pte_at(_arg: mm, )pmdp: *mut (pte_t, _arg: pmd_pte(pmd)) -> return;
}
extern "C" {
    pub fn __set_pte_at(_arg: mm, )pudp: *mut (pte_t, _arg: pud_pte(pud)) -> return;
}

extern "C" {
    pub fn pte_present(pte_user(pte: pte) &&) -> return;
}
extern "C" {
    pub fn pmd_leaf(pmd_user(pmd: pmd) &&) -> return;
}
extern "C" {
    pub fn pud_leaf(pud_user(pud: pud) &&) -> return;
}

extern "C" {
    pub fn pmd_leaf(_arg: pmd) -> return;
}
extern "C" {
    pub fn ptep_set_access_flags(_arg: vma, _arg: address, )pmdp: *mut (pte_t, _arg: pmd_pte(entry), _arg: dirty) -> return;
}
extern "C" {
    pub fn ptep_test_and_clear_young(_arg: vma, _arg: address, )pmdp: *mut (pte_t) -> return;
}

extern "C" {
    pub fn __pmd()pmdp: *mut atomic_long_xchg((atomic_long_t, _arg: pmd_val(pmd))) -> return;
}

extern "C" {
    pub fn pte_pud(_arg: pte_wrprotect(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn pud_leaf(_arg: pud) -> return;
}
extern "C" {
    pub fn pte_dirty(_arg: pud_pte(pud)) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkyoung(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkold(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkdirty(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkclean(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_mkwrite_novma(pud_pte(pud))) -> return;
}
extern "C" {
    pub fn ptep_set_access_flags(_arg: vma, _arg: address, )pudp: *mut (pte_t, _arg: pud_pte(entry), _arg: dirty) -> return;
}
extern "C" {
    pub fn ptep_test_and_clear_young(_arg: vma, _arg: address, )pudp: *mut (pte_t) -> return;
}

extern "C" {
    pub fn pte_young(_arg: pud_pte(pud)) -> return;
}
extern "C" {
    pub fn __pud()pudp: *mut atomic_long_xchg((atomic_long_t, _arg: pud_val(pud))) -> return;
}
extern "C" {
    pub fn __pud(_PAGE_PROT_NONE): pud_val(pud) & ~(_PAGE_PRESENT |) -> return;
}
extern "C" {
    pub fn pte_pud(_arg: pte_modify(pud_pte(pud), _arg: newprot)) -> return;
}

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTE:
// bit            0:	_PAGE_PRESENT (zero)
// bit       1 to 2:	(zero)
// bit            3:	_PAGE_SWP_SOFT_DIRTY
// bit            4:	_PAGE_SWP_UFFD
// bit            5:	_PAGE_PROT_NONE (zero)
// bit            6:	exclusive marker
// bits      7 to 11:	swap type
// bits 12 to XLEN-1:	swap offset
//
pub const __SWP_TYPE_SHIFT: c_int = 7;
pub const __SWP_TYPE_BITS: c_int = 5;

extern "C" {
    pub fn __pte(_PAGE_SWP_EXCLUSIVE: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_SWP_EXCLUSIVE: pte_val(pte) &) -> return;
}

//
// In the RV64 Linux scheme, we give the user half of the virtual-address space
// and give the kernel the other (upper) half.
//

//
// Task size is 0x4000000000 for RV64 or 0x9fc00000 for RV32.
// Note that PGDIR_SIZE must evenly divide TASK_SIZE.
// Task size is:
// -        0x9fc00000	(~2.5GB) for RV32.
// -      0x4000000000	( 256GB) for RV64 using SV39 mmu
// -    0x800000000000	( 128TB) for RV64 using SV48 mmu
// - 0x100000000000000	(  64PB) for RV64 using SV57 mmu
//
// Note that PGDIR_SIZE must evenly divide TASK_SIZE since "RISC-V
// Instruction Set Manual Volume II: Privileged Architecture" states that
// "load and store effective addresses, which are 64bits, must have bits
// 63–48 all equal to bit 47, or else a page-fault exception will occur."
// Similarly for SV57, bits 63–57 must be equal to bit 56.
//

extern "C" {
    pub fn paging_init();
}
extern "C" {
    pub fn misc_mem_init();
}
//
// Use set_p*_safe(), and elide TLB flushing, when confident that *no
// TLB flush will be required as a result of the "set". For example, use
// in scenarios where it is known ahead of time that the routine is
// setting non-present entries, or re-setting an existing entry to the
// same value. Otherwise, use the typical "set" helpers and flush the
// TLB.
//

