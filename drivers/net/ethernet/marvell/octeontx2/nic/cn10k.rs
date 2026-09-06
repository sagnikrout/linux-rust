//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/cn10k.h
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
// Marvell RVU Ethernet driver
//
// Copyright (C) 2021 Marvell.
//

// On OTx2, since AF returns DWRR_MTU as '1', this logic
// will work on those silicons as well.
//
extern "C" {
    pub fn cn10k_refill_pool_ptrs(dev: *mut c_void, cq: *mut otx2_cq_queue) -> c_int;
}
extern "C" {
    pub fn cn10k_sqe_flush(dev: *mut c_void, sq: *mut otx2_snd_queue, size: c_int, qidx: c_int);
}
extern "C" {
    pub fn cn10k_sq_aq_init(dev: *mut c_void, qidx: u16, chan_offset: u8, sqb_aura: u16) -> c_int;
}
extern "C" {
    pub fn cn10k_lmtst_init(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn cn10k_free_all_ipolicers(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn cn10k_alloc_matchall_ipolicer(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn cn10k_free_matchall_ipolicer(pfvf: *mut otx2_nic) -> c_int;
}
extern "C" {
    pub fn cn10k_alloc_leaf_profile(pfvf: *mut otx2_nic, leaf: *mut u16) -> c_int;
}
extern "C" {
    pub fn cn10k_free_leaf_profile(pfvf: *mut otx2_nic, leaf: u16) -> c_int;
}
extern "C" {
    pub fn otx2_init_hw_ops(pfvf: *mut otx2_nic);
}
