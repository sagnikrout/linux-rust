//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icssg_stats.h
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
// Texas Instruments ICSSG Ethernet driver
//
// Copyright (C) 2018-2022 Texas Instruments Incorporated - https://www.ti.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct miig_stats_regs {
// Rx
    pub rx_packets: u32,
    pub rx_broadcast_frames: u32,
    pub rx_multicast_frames: u32,
    pub rx_crc_errors: u32,
    pub rx_mii_error_frames: u32,
    pub rx_odd_nibble_frames: u32,
    pub rx_frame_max_size: u32,
    pub rx_max_size_error_frames: u32,
    pub rx_frame_min_size: u32,
    pub rx_min_size_error_frames: u32,
    pub rx_over_errors: u32,
    pub rx_class0_hits: u32,
    pub rx_class1_hits: u32,
    pub rx_class2_hits: u32,
    pub rx_class3_hits: u32,
    pub rx_class4_hits: u32,
    pub rx_class5_hits: u32,
    pub rx_class6_hits: u32,
    pub rx_class7_hits: u32,
    pub rx_class8_hits: u32,
    pub rx_class9_hits: u32,
    pub rx_class10_hits: u32,
    pub rx_class11_hits: u32,
    pub rx_class12_hits: u32,
    pub rx_class13_hits: u32,
    pub rx_class14_hits: u32,
    pub rx_class15_hits: u32,
    pub rx_smd_frags: u32,
    pub rx_bucket1_size: u32,
    pub rx_bucket2_size: u32,
    pub rx_bucket3_size: u32,
    pub rx_bucket4_size: u32,
    pub rx_64B_frames: u32,
    pub rx_bucket1_frames: u32,
    pub rx_bucket2_frames: u32,
    pub rx_bucket3_frames: u32,
    pub rx_bucket4_frames: u32,
    pub rx_bucket5_frames: u32,
    pub rx_bytes: u32,
    pub rx_tx_total_bytes: u32,
// Tx
    pub tx_packets: u32,
    pub tx_broadcast_frames: u32,
    pub tx_multicast_frames: u32,
    pub tx_odd_nibble_frames: u32,
    pub tx_underflow_errors: u32,
    pub tx_frame_max_size: u32,
    pub tx_max_size_error_frames: u32,
    pub tx_frame_min_size: u32,
    pub tx_min_size_error_frames: u32,
    pub tx_bucket1_size: u32,
    pub tx_bucket2_size: u32,
    pub tx_bucket3_size: u32,
    pub tx_bucket4_size: u32,
    pub tx_64B_frames: u32,
    pub tx_bucket1_frames: u32,
    pub tx_bucket2_frames: u32,
    pub tx_bucket3_frames: u32,
    pub tx_bucket4_frames: u32,
    pub tx_bucket5_frames: u32,
    pub tx_bytes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_miig_stats {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub offset: u32,
    pub standard_stats: bool,
}

// Rx
// Tx

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_pa_stats {
    pub name: [c_char; ETH_GSTRING_LEN],
    pub offset: u32,
}
