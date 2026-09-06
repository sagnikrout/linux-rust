//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_ethtool.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_tx_stats {
    pub syncp: u64_stats_sync,
    pub descs: u64_stats_t,
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub drops: u64_stats_t,
    pub kicks: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_ctx_stats {
    pub descs: u64,
    pub packets: u64,
    pub bytes: u64,
    pub drops: u64,
    pub split_hdr_bytes: u64,
    pub split_hdr_packets: u64,
    pub kicks: u64,
    pub length_errors: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_rx_stats {
    pub syncp: u64_stats_sync,
    pub descs: u64_stats_t,
    pub packets: u64_stats_t,
    pub bytes: u64_stats_t,
    pub drops: u64_stats_t,
    pub kicks: u64_stats_t,
    pub split_hdr_bytes: u64_stats_t,
    pub split_hdr_packets: u64_stats_t,
    pub length_errors: u64_stats_t,
}
