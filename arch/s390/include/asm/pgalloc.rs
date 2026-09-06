//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pgalloc.h
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Hartmut Penner (hp@de.ibm.com)
// Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/pgalloc.h"
// Copyright (C) 1994  Linus Torvalds
//

pub const CRST_ALLOC_ORDER: c_int = 2;

extern "C" {
    pub fn crst_table_free(: *mut mm_struct, : *mut c_ulong);
}

extern "C" {
    pub fn page_table_free(: *mut mm_struct, : *mut c_ulong);
}
extern "C" {
    pub fn crst_table_upgrade(mm: *mut mm_struct, limit: c_ulong) -> c_int;
}

//
// page table entry allocation/free routines.
//

// arch use pte_free_defer() implementation in arch/s390/mm/pgalloc.c

extern "C" {
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);
}
extern "C" {
    pub fn vmem_map_init();
}
extern "C" {
    pub fn base_asce_alloc(addr: c_ulong, num_pages: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn base_asce_free(asce: c_ulong);
}
