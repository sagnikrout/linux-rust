//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/marvell/octeontx2/nic/rep.h
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
// Marvell RVU REPRESENTOR driver
//
// Copyright (C) 2024 Marvell.
//

pub const PCI_DEVID_RVU_REP: c_uint = 0xA0E0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rep_stats {
    pub rx_bytes: u64,
    pub rx_frames: u64,
    pub rx_drops: u64,
    pub rx_mcast_frames: u64,
    pub tx_bytes: u64,
    pub tx_frames: u64,
    pub tx_drops: u64,
    pub tx_discards: atomic_long_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rep_dev {
    pub mdev: *mut otx2_nic,
    pub netdev: *mut net_device,
    pub stats: rep_stats,
    pub stats_wrk: delayed_work,
    pub dl_port: devlink_port,
    pub flow_cfg: *mut otx2_flow_config,

    pub flags: u64,
    pub rep_id: u16,
    pub pcifunc: u16,
    pub mac: [u8; ETH_ALEN],
}

extern "C" {
    pub fn rvu_rep_create(priv: *mut otx2_nic, extack: *mut netlink_ext_ack) -> c_int;
}
extern "C" {
    pub fn rvu_rep_destroy(priv: *mut otx2_nic);
}
extern "C" {
    pub fn rvu_event_up_notify(pf: *mut otx2_nic, info: *mut rep_event) -> c_int;
}
