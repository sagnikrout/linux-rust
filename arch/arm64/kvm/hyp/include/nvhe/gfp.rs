//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/gfp.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyp_pool {
//
// Spinlock protecting concurrent changes to the memory pool as well as
// the struct hyp_page of the pool's pages until we have a proper atomic
// API at EL2.
//
    pub lock: hyp_spinlock_t,
    pub free_area: [list_head; NR_PAGE_ORDERS],
    pub range_start: phys_addr_t,
    pub range_end: phys_addr_t,
    pub max_order: u8,
}

// Allocation
extern "C" {
    pub fn hyp_split_page(page: *mut hyp_page);
}
extern "C" {
    pub fn hyp_get_page(pool: *mut hyp_pool, addr: *mut c_void);
}
extern "C" {
    pub fn hyp_put_page(pool: *mut hyp_pool, addr: *mut c_void);
}
// Used pages cannot be freed
