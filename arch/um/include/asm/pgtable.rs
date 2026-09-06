//! Automatically rewritten from C Header to Rust Module
//! Source: arch/um/include/asm/pgtable.h
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
// Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
// Copyright 2003 PathScale, Inc.
// Derived from include/asm-i386/pgtable.h
//

pub const _PAGE_PRESENT: c_uint = 0x001;
pub const _PAGE_NEEDSYNC: c_uint = 0x002;
pub const _PAGE_RW: c_uint = 0x020;
pub const _PAGE_USER: c_uint = 0x040;
pub const _PAGE_ACCESSED: c_uint = 0x080;
pub const _PAGE_DIRTY: c_uint = 0x100;
// If _PAGE_PRESENT is clear, we use these:
pub const _PAGE_PROTNONE: c_uint = 0x010	/* if the user mapped it with PROT_NONE;;
// We borrow bit 10 to store the exclusive marker in swap PTEs.
pub const _PAGE_SWP_EXCLUSIVE: c_uint = 0x400;

// Just any arbitrary offset to the start of the vmalloc VM area: the
// current 8MB value just means that there will be a 8MB "hole" after the
// physical memory until the kernel virtual memory starts.  That means that
// any out-of-bounds memory accesses will hopefully be caught.
// The vmalloc() routines leaves a hole of 4kB between each vmalloced
// area for the same reason. ;)
//

//
// The i386 can't do page protection for execute, and considers that the same
// are read.
// Also, write permissions imply read permissions. This is the closest we can
// get..
//

//
// =================================
// Flags checking section.
// =================================
//
extern "C" {
    pub fn pte_is_zero(_arg: pte) -> return;
}
//
// The following only work if pte_present() is true.
// Undefined behaviour if not..
//
extern "C" {
    pub fn pte_get_bits(_arg: pte, _arg: _PAGE_DIRTY) -> return;
}
extern "C" {
    pub fn pte_get_bits(_arg: pte, _arg: _PAGE_ACCESSED) -> return;
}
extern "C" {
    pub fn pte_get_bits(_arg: pte, _arg: _PAGE_NEEDSYNC) -> return;
}
//
// =================================
// Flags setting section.
// =================================
//
// If it's a swap entry, it needs to be marked _PAGE_NEEDSYNC so
// update_pte_range knows to unmap it.
//
// pteptr = pte_mkneedsync(*pteptr);

// Basically the default implementation

//
// the pmd page can be thought of an array like this: pmd_t[PTRS_PER_PMD]
//
// this macro returns the index of the entry in the pmd page which would
// control the given virtual address
//

//
// Encode/decode swap entries and swap PTEs. Swap PTEs are all PTEs that
// are !pte_none() && !pte_present().
//
// Format of swap PTEs:
//
// 3 3 2 2 2 2 2 2 2 2 2 2 1 1 1 1 1 1 1 1 1 1
// 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0 9 8 7 6 5 4 3 2 1 0
// <--------------- offset ----------------> E < type -> 0 0 0 1 0
//
// E is the exclusive marker that is not stored in swap entries.
// _PAGE_NEEDSYNC (bit 1) is always set to 1 in set_pte().
//

extern "C" {
    pub fn pte_get_bits(_arg: pte, _arg: _PAGE_SWP_EXCLUSIVE) -> return;
}
