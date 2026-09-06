//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ksm.h
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
//
// Memory merging support.
//
// This code enables dynamic sharing of identical pages found in different
// memory areas, even if they are not shared by fork().
//

extern "C" {
    pub fn ksm_enable_merge_any(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn ksm_disable_merge_any(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn ksm_disable(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn __ksm_enter(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn __ksm_exit(mm: *mut mm_struct);
}
//
// To identify zeropages that were mapped by KSM, we reuse the dirty bit
// in the PTE. If the PTE is dirty, the zeropage was mapped by KSM when
// deduplicating memory.
//

extern "C" {
    pub fn atomic_long_read(_arg: &mm->ksm_zero_pages) -> return;
}
// Adding mm to ksm is best effort on fork.
extern "C" {
    pub fn __ksm_enter(_arg: mm) -> return;
}
//
// When do_swap_page() first faults in from swap what used to be a KSM page,
// no problem, it will be assigned to this vma's anon_vma; but thereafter,
// it might be faulted into a different anon_vma (or perhaps to a different
// offset in the same anon_vma).  do_swap_page() cannot do all the locking
// needed to reconstitute a cross-anon_vma KSM page: for now it has to make
// a copy, and leave remerging the pages to a later pass of ksmd.
//
// We'd like to make this conditional on vma->vm_flags & VM_MERGEABLE,
// but what if the vma was unmerged while the page was swapped out?
//
extern "C" {
    pub fn rmap_walk_ksm(folio: *mut folio, rwc: *mut rmap_walk_control);
}
extern "C" {
    pub fn folio_migrate_ksm(newfolio: *mut folio, folio: *mut folio);
}
extern "C" {
    pub fn ksm_process_profit(: *mut mm_struct) -> c_long;
}
extern "C" {
    pub fn ksm_process_mergeable(mm: *mut mm_struct) -> bool;
}

