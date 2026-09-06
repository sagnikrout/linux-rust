//! Automatically rewritten from C Header to Rust Module
//! Source: net/core/page_pool_priv.h
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
    pub fn page_pool_inflight(pool: *const page_pool, strict: bool) -> i32;
}
extern "C" {
    pub fn page_pool_list(pool: *mut page_pool) -> c_int;
}
extern "C" {
    pub fn page_pool_detached(pool: *mut page_pool);
}
extern "C" {
    pub fn page_pool_unlist(pool: *mut page_pool);
}
// We assume page alignment to shave off bottom bits,
// if this "compression" doesn't work we need to drop.
//
extern "C" {
    pub fn page_pool_set_dma_addr_netmem(_arg: page_to_netmem(page), _arg: addr) -> return;
}

extern "C" {
    pub fn page_pool_set_pp_info(pool: *mut page_pool, netmem: netmem_ref);
}
extern "C" {
    pub fn page_pool_clear_pp_info(netmem: netmem_ref);
}

