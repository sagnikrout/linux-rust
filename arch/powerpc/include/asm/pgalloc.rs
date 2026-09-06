//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pgalloc.h
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

extern "C" {
    pub fn pte_frag_destroy(pte_frag: *mut c_void);
}
extern "C" {
    pub fn pte_fragment_free(table: *mut c_ulong, kernel: c_int);
}
// arch use pte_free_defer() implementation in arch/powerpc/mm/pgtable-frag.c

extern "C" {
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);
}
//
// Functions that deal with pagetables that could be at any level of
// the table need to be passed an "index_size" so they know how to
// handle allocation.  For PTE pages, the allocation size will be
// (2^index_size * sizeof(pointer)) and allocations are drawn from
// the kmem_cache in PGT_CACHE(index_size).
//
// The maximum index size needs to be big enough to allow any
// pagetable sizes we need, but small enough to fit in the low bits of
// any page table pointer.  In other words all pagetables, even tiny
// ones, must be aligned to allow at least enough low 0 bits to
// contain this value.  This value is also used as a mask, so it must
// be one less than a power of two.
//
pub const MAX_PGTABLE_INDEX_SIZE: c_uint = 0xf;

