//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/otx2_xsk.h
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
// Marvell RVU PF/VF Netdev Devlink
//
// Copyright (C) 2024 Marvell.
//
extern "C" {
    pub fn otx2_xsk_pool_setup(pf: *mut otx2_nic, pool: *mut xsk_buff_pool, qid: u16) -> c_int;
}
extern "C" {
    pub fn otx2_xsk_pool_enable(pf: *mut otx2_nic, pool: *mut xsk_buff_pool, qid: u16) -> c_int;
}
extern "C" {
    pub fn otx2_xsk_pool_disable(pf: *mut otx2_nic, qid: u16) -> c_int;
}
extern "C" {
    pub fn otx2_xsk_wakeup(dev: *mut net_device, queue_id: u32, flags: u32) -> c_int;
}
extern "C" {
    pub fn otx2_attach_xsk_buff(pfvf: *mut otx2_nic, sq: *mut otx2_snd_queue, qidx: c_int);
}
