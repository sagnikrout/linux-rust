//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/page_pool/memory_provider.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct memory_provider_ops {
    pub gfp): *mut *mut *mut netmem_ref (alloc_netmems)(struct page_pool pool, gfp_t,
    pub netmem): *mut *mut *mut bool (release_netmem)(struct page_pool pool, netmem_ref,
    pub pool): *mut *mut int (init)(struct page_pool,
    pub pool): *mut *mut void (destroy)(struct page_pool,
    pub rxq): *mut netdev_rx_queue,
    pub rxq): *mut *mut *mut void (uninstall)(void mp_priv, struct netdev_rx_queue,
}

extern "C" {
    pub fn net_mp_niov_set_dma_addr(niov: *mut net_iov, addr: dma_addr_t) -> bool;
}
extern "C" {
    pub fn net_mp_niov_set_page_pool(pool: *mut page_pool, niov: *mut net_iov);
}
extern "C" {
    pub fn net_mp_niov_clear_page_pool(niov: *mut net_iov);
}
//
// net_mp_netmem_place_in_cache() - give a netmem to a page pool
// @pool:      the page pool to place the netmem into
// @netmem:    netmem to give
//
// Push an accounted netmem into the page pool's allocation cache. The caller
// must ensure that there is space in the cache. It should only be called off
// the mp_ops->alloc_netmems() path.
//
