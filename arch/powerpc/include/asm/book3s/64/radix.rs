//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/radix.h
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


// SPDX-License-Identifier: GPL-2.0

// An empty PTE can still have a R or C writeback

// Bits to set in a RPMD/RPUD/RPGD

// Don't have anything in the reserved bits and leaf bits
pub const RADIX_PMD_BAD_BITS: c_uint = 0x60000000000000e0UL;
pub const RADIX_PUD_BAD_BITS: c_uint = 0x60000000000000e0UL;
pub const RADIX_P4D_BAD_BITS: c_uint = 0x60000000000000e0UL;

//
// Size of EA range mapped by our pagetables.
//

//
// We support 52 bit address space, Use top bit for kernel
// virtual mapping. Also make sure kernel fit in the top
// quadrant.
//
// +------------------+
// +------------------+  Kernel virtual map (0xc008000000000000)
// |                  |
// 0b11......+------------------+  Kernel linear map (0xc....)
// |                  |
// |     2 quadrant   |
// |                  |
// 0b10......+------------------+
// |                  |
// |    1 quadrant    |
// |                  |
// 0b01......+------------------+
// |                  |
// |    0 quadrant    |
// |                  |
// 0b00......+------------------+
//
// 3rd quadrant expanded:
// +------------------------------+  Highest address (0xc010000000000000)
// +------------------------------+  KASAN shadow end (0xc00fc00000000000)
// |                              |
// +------------------------------+  Kernel vmemmap end/shadow start (0xc00e000000000000)
// |                              |
// |           512TB		  |
// |                              |
// +------------------------------+  Kernel IO map end/vmemap start
// |                              |
// |           512TB		  |
// |                              |
// +------------------------------+  Kernel vmap end/ IO map start
// |                              |
// |           512TB		  |
// |                              |
// +------------------------------+  Kernel virt start (0xc008000000000000)
// |                              |
// +------------------------------+  Kernel linear (0xc.....)
//
// For the sizes of the shadow area, see kasan.h
//
// If we store section details in page->flags we can't increase the MAX_PHYSMEM_BITS
// if we increase SECTIONS_WIDTH we will not store node details in page->flags and
// page_to_nid does a page->section->node lookup
// Hence only increase for VMEMMAP. Further depending on SPARSEMEM_EXTREME reduce
// memory requirements with large number of sections.
// 51 bits is the max physical real address on POWER9
//

pub const R_MAX_PHYSMEM_BITS: c_int = 51;

pub const R_MAX_PHYSMEM_BITS: c_int = 46;

//
// 49 =  MAX_EA_BITS_PER_CONTEXT (hash specific). To make sure we pick
// the same value as hash.
//

extern "C" {
    pub fn radix__mark_rodata_ro();
}
extern "C" {
    pub fn radix__mark_initmem_nx();
}

extern "C" {
    pub fn be64_to_cpu(_arg: old_be) -> return;
}
// ptep = __pte(0);
extern "C" {
    pub fn __pte(_arg: old_pte) -> return;
}
// ptep = pte;
//
// The architecture suggests a ptesync after setting the pte, which
// orders the store that updates the pte with subsequent page table
// walk accesses which may load the pte. Without this it may be
// possible for a subsequent access to result in spurious fault.
//
// This is not necessary for correctness, because a spurious fault
// is tolerated by the page fault handler, and this store will
// eventually be seen. In testing, there was no noticable increase
// in user faults on POWER9. Avoiding ptesync here is a significant
// win for things like fork. If a future microarchitecture benefits
// from ptesync, it should probably go into update_mmu_cache, rather
// than set_pte_at (which is used to set ptes unrelated to faults).
//
// Spurious faults from the kernel memory are not tolerated, so there
// is a ptesync in flush_cache_vmap, and __map_kernel_page() follows
// the pte update sequence from ISA Book III 6.10 Translation Table
// Update Synchronization Requirements.
//

extern "C" {
    pub fn __pmd(_PAGE_PTE: pmd_val(pmd) |) -> return;
}
extern "C" {
    pub fn __pud(_PAGE_PTE: pud_val(pud) |) -> return;
}
extern "C" {
    pub fn radix__pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
}
// For radix 2M at PMD level means thp
// For radix 1G at PUD level means pud hugepage support

//
// We support 52 bits, hence:
// bits 52 - 31 = 21, 0b10101
// RTS encoding details
// bits 0 - 3 of rts -> bits 6 - 8 unsigned long
// bits 4 - 5 of rts -> bits 62 - 63 of unsigned long
//

extern "C" {
    pub fn radix__remove_section_mapping(start: c_ulong, end: c_ulong) -> c_int;
}

extern "C" {
    pub fn vmemmap_can_optimize(altmap: *mut vmem_altmap, pgmap: *mut dev_pagemap) -> bool;
}

