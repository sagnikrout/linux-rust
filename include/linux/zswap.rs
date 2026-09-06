//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/zswap.h
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
pub struct zswap_lruvec_state {
//
// Number of swapped in pages from disk, i.e not found in the zswap pool.
//
// This is consumed and subtracted from the lru size in
// zswap_shrinker_count() to penalize past overshrinking that led to disk
// swapins. The idea is that had we considered this many more pages in the
// LRU active/protected and not written them back, we would not have had to
// swapped them in.
//
    pub nr_disk_swapins: atomic_long_t,
}

extern "C" {
    pub fn zswap_total_pages() -> c_ulong;
}
extern "C" {
    pub fn zswap_store(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn zswap_load(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn zswap_invalidate(swp: swp_entry_t);
}
extern "C" {
    pub fn zswap_swapon(type: c_int, nr_pages: c_ulong) -> c_int;
}
extern "C" {
    pub fn zswap_swapoff(type: c_int);
}
extern "C" {
    pub fn zswap_memcg_offline_cleanup(memcg: *mut mem_cgroup);
}
extern "C" {
    pub fn zswap_lruvec_state_init(lruvec: *mut lruvec);
}
extern "C" {
    pub fn zswap_folio_swapin(folio: *mut folio);
}
extern "C" {
    pub fn zswap_is_enabled() -> bool;
}
extern "C" {
    pub fn zswap_never_enabled() -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zswap_lruvec_state {
    pub false: return,
    pub -ENOENT: return,
    pub 0: return,
    pub false: return,
    pub true: return,

