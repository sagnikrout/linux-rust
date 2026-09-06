//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/pgalloc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmemmap_backing {
    pub list: *mut vmemmap_backing,
    pub phys: c_ulong,
    pub virt_addr: c_ulong,
}

extern "C" {
    pub fn pmd_fragment_free(: *mut c_ulong);
}
extern "C" {
    pub fn pgtable_free_tlb(tlb: *mut mmu_gather, table: *mut c_void, shift: c_int);
}
extern "C" {
    pub fn __tlb_remove_table(_table: *mut c_void);
}
extern "C" {
    pub fn pte_frag_destroy(pte_frag: *mut c_void);
}

extern "C" {
    pub fn radix__pgd_alloc(_arg: mm) -> return;
}
//
// Don't scan the PGD for pointers, it contains references to PUDs but
// those references are not full pointers and so can't be recognised by
// kmemleak.
//
// With hugetlb, we don't clear the second half of the page table.
// If we share the same slab cache with the pmd or pud level table,
// we need to make sure we zero out the full table on alloc.
// With 4K we don't store slot in the second half. Hence we don't
// need to do this for 4k.
//

extern "C" {
    pub fn radix__pgd_free(_arg: mm, _arg: pgd) -> return;
}
// pgd =  __p4d(__pgtable_ptr_val(pud) | PGD_VAL_BITS);
//
// Tell kmemleak to ignore the PUD, that means don't scan it for
// pointers and don't consider it a leak. PUDs are typically only
// referred to by their PGD, but kmemleak is not able to recognise those
// as pointers, leading to false leak reports.
//
// Early pud pages allocated via memblock allocator
// can't be directly freed to slab. KFENCE pages have
// both reserved and slab flags set so need to be freed
// kmem_cache_free.
//
extern "C" {
    pub fn __pud_free(_arg: pud) -> return;
}
// pud = __pud(__pgtable_ptr_val(pmd) | PUD_VAL_BITS);
extern "C" {
    pub fn pmd_fragment_alloc(_arg: mm, _arg: addr) -> return;
}
extern "C" {
    pub fn pgtable_free_tlb(_arg: tlb, _arg: pmd, _arg: PMD_INDEX) -> return;
}
// pmd = __pmd(__pgtable_ptr_val(pte) | PMD_VAL_BITS);
// pmd = __pmd(__pgtable_ptr_val(pte_page) | PMD_VAL_BITS);
