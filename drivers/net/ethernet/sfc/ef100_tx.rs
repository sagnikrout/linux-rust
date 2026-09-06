//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef100_tx.h
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
//
// Driver for Solarflare network controllers and boards
// Copyright 2019 Solarflare Communications Inc.
// Copyright 2019-2020 Xilinx Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//

extern "C" {
    pub fn ef100_tx_probe(tx_queue: *mut efx_tx_queue) -> c_int;
}
extern "C" {
    pub fn ef100_tx_init(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn ef100_tx_write(tx_queue: *mut efx_tx_queue);
}
extern "C" {
    pub fn ef100_tx_max_skb_descs(efx: *mut efx_nic) -> c_uint;
}
extern "C" {
    pub fn ef100_ev_tx(channel: *mut efx_channel, p_event: *const efx_qword_t) -> c_int;
}
extern "C" {
    pub fn ef100_enqueue_skb(tx_queue: *mut efx_tx_queue, skb: *mut sk_buff) -> netdev_tx_t;
}
