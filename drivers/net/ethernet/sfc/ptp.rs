//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ptp.h
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
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
// Copyright 2019-2020 Xilinx Inc.
//

extern "C" {
    pub fn efx_ptp_probe(efx: *mut efx_nic, channel: *mut efx_channel) -> c_int;
}
extern "C" {
    pub fn efx_ptp_defer_probe_with_channel(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ptp_update_channel(efx: *mut efx_nic, channel: *mut efx_channel);
}
extern "C" {
    pub fn efx_ptp_remove(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ptp_is_ptp_tx(efx: *mut efx_nic, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn efx_ptp_tx(efx: *mut efx_nic, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn efx_ptp_event(efx: *mut efx_nic, ev: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_ptp_describe_stats(efx: *mut efx_nic, strings: *mut u8) -> usize;
}
extern "C" {
    pub fn efx_ptp_update_stats(efx: *mut efx_nic, stats: *mut u64) -> usize;
}
extern "C" {
    pub fn efx_time_sync_event(channel: *mut efx_channel, ev: *mut efx_qword_t);
}
extern "C" {
    pub fn efx_ptp_start_datapath(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ptp_stop_datapath(efx: *mut efx_nic);
}
extern "C" {
    pub fn efx_ptp_use_mac_tx_timestamps(efx: *mut efx_nic) -> bool;
}
extern "C" {
    pub fn efx_ptp_nic_to_kernel_time(tx_queue: *mut efx_tx_queue) -> ktime_t;
}
