//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/tlbflush.h
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
// Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
//

//
// In UML, we need to sync the TLB over by using mmap/munmap syscalls from
// the process handling the MM (which can be the kernel itself).
//
// To track updates, we can hook into set_ptes and flush_tlb_*. With set_ptes
// we catch all PTE transitions where memory that was unusable becomes usable.
// While with flush_tlb_* we can track any memory that becomes unusable and
// even if a higher layer of the page table was modified.
//
// So, we simply track updates using both methods and mark the memory area to
// be synced later on. The only special case is that flush_tlb_kern_* needs to
// be executed immediately as there is no good synchronization point in that
// case. In contrast, in the set_ptes case we can wait for the next kernel
// segfault before we do the synchornization.
//
// - flush_tlb_all() flushes all processes TLBs
// - flush_tlb_mm(mm) flushes the specified mm context TLB's
// - flush_tlb_page(vma, vmaddr) flushes one page
// - flush_tlb_range(vma, start, end) flushes a range of pages
// - flush_tlb_kernel_range(start, end) flushes a range of kernel pages
//
extern "C" {
    pub fn um_tlb_sync(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn flush_tlb_all();
}
extern "C" {
    pub fn flush_tlb_mm(mm: *mut mm_struct);
}
// Kernel needs to be synced immediately
