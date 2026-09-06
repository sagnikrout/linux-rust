//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pgtable.h
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

// Make modules code happy. We don't set RO yet

// Advertise special mapping type for AGP

// Macro flag: #define HAVE_PAGE_AGP

// Keep this as a macro to avoid include dependency mess

//
// Select all bits except the pfn
//

extern "C" {
    pub fn __pgprot(_arg: pte_flags) -> return;
}

extern "C" {
    pub fn pte_pgprot(_arg: pmd_pte(pmd)) -> return;
}

extern "C" {
    pub fn pte_pgprot(_arg: pud_pte(pud)) -> return;
}

extern "C" {
    pub fn pte_pgprot(_arg: pte_exprotect(__pte(pgprot_val(prot)))) -> return;
}

extern "C" {
    pub fn __va(~PMD_MASKED_BITS: pmd_val(pmd) &) -> return;
}

extern "C" {
    pub fn paging_init();
}
extern "C" {
    pub fn poking_init();
}
// can we use this in kvm
extern "C" {
    pub fn vmalloc_to_phys(vmalloc_addr: *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn pgtable_cache_add(shift: c_uint);
}

extern "C" {
    pub fn mark_initmem_nx();
}

extern "C" {
    pub fn __phys_mem_access_prot(_arg: pfn, _arg: size, _arg: vma_prot) -> return;
}
extern "C" {
    pub fn __update_mmu_cache(vma: *mut vm_area_struct, address: c_ulong, ptep: *mut pte_t);
}
//
// This gets called at the end of handling a page fault, when
// the kernel has put a new PTE into the page table for the process.
// We use it to ensure coherency between the i-cache and d-cache
// for the page which has just been mapped in.
// On machines which use an MMU hash table, we use this to put a
// corresponding HPTE into the hash table ahead of time, instead of
// waiting for the inevitable extra hash-table miss exception.
//
// When used, PTE_FRAG_NR is defined in subarch pgtable.h
// so we are sure it is included when arriving here.
//

pub const PTE_FRAG_NR: c_int = 1;

extern "C" {
    pub fn vmemmap_populated(vmemmap_addr: c_ulong, vmemmap_map_size: c_int) -> int __meminit;
}
//
// mm/memory_hotplug.c:mhp_supports_memmap_on_memory goes into details
// some of the restrictions. We don't check for PMD_SIZE because our
// vmemmap allocation code can fallback correctly. The pageblock
// alignment requirement is met using altmap->reserve blocks.
//

//
// With 4K page size and 2M PMD_SIZE, we can align
// things better with memory block size value
// starting from 128MB. Hence align things with PMD_SIZE.
//
extern "C" {
    pub fn IS_ALIGNED(_arg: vmemmap_size, _arg: PMD_SIZE) -> return;
}

