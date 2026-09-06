//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/snic/vnic_stats.h
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
// Copyright 2014 Cisco Systems, Inc.  All rights reserved.
// Tx statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_tx_stats {
    pub tx_frames_ok: u64,
    pub tx_unicast_frames_ok: u64,
    pub tx_multicast_frames_ok: u64,
    pub tx_broadcast_frames_ok: u64,
    pub tx_bytes_ok: u64,
    pub tx_unicast_bytes_ok: u64,
    pub tx_multicast_bytes_ok: u64,
    pub tx_broadcast_bytes_ok: u64,
    pub tx_drops: u64,
    pub tx_errors: u64,
    pub tx_tso: u64,
    pub rsvd: [u64; 16],
}

// Rx statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_rx_stats {
    pub rx_frames_ok: u64,
    pub rx_frames_total: u64,
    pub rx_unicast_frames_ok: u64,
    pub rx_multicast_frames_ok: u64,
    pub rx_broadcast_frames_ok: u64,
    pub rx_bytes_ok: u64,
    pub rx_unicast_bytes_ok: u64,
    pub rx_multicast_bytes_ok: u64,
    pub rx_broadcast_bytes_ok: u64,
    pub rx_drop: u64,
    pub rx_no_bufs: u64,
    pub rx_errors: u64,
    pub rx_rss: u64,
    pub rx_crc_errors: u64,
    pub rx_frames_64: u64,
    pub rx_frames_127: u64,
    pub rx_frames_255: u64,
    pub rx_frames_511: u64,
    pub rx_frames_1023: u64,
    pub rx_frames_1518: u64,
    pub rx_frames_to_max: u64,
    pub rsvd: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vnic_stats {
    pub tx: vnic_tx_stats,
    pub rx: vnic_rx_stats,
}
