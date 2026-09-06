//! Automatically rewritten from C to Rust
//! Source: drivers/xen/biomerge.c
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

// check if @page can be merged with 'vec1'
    bool xen_biovec_phys_mergeable(const struct bio_vec *vec1,
    const struct page *page)
    {

    let mut bfn1: c_ulong = pfn_to_bfn(page_to_pfn(vec1.bv_page));
    let mut bfn2: c_ulong = pfn_to_bfn(page_to_pfn(page));
    return bfn1 + PFN_DOWN(vec1.bv_offset + vec1.bv_len) == bfn2;

//
// XXX: Add support for merging bio_vec when using different page
// size in Xen and Linux.
//
    return false;

    }
