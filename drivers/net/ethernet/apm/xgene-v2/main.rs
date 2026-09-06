//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene-v2/main.h
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
// Applied Micro X-Gene SoC Ethernet v2 Driver
//
// Copyright (c) 2017, Applied Micro Circuits Corporation
// Author(s): Iyappan Subramanian <isubramanian@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//

pub const XGENE_ENET_STD_MTU: c_int = 1536;
pub const XGENE_ENET_MIN_FRAME: c_int = 60;
pub const IRQ_ID_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_resource {
    pub base_addr: *mut void __iomem,
    pub phy_mode: c_int,
    pub irq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_stats {
    pub tx_packets: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub rx_errors: u64,
}

// ethernet private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xge_pdata {
    pub resources: xge_resource,
    pub tx_ring: *mut xge_desc_ring,
    pub rx_ring: *mut xge_desc_ring,
    pub pdev: *mut platform_device,
    pub irq_name: [c_char; IRQ_ID_SIZE],
    pub mdio_bus: *mut mii_bus,
    pub ndev: *mut net_device,
    pub napi: napi_struct,
    pub stats: xge_stats,
    pub phy_speed: c_int,
    pub nbufs: u8,
}

extern "C" {
    pub fn xge_mdio_config(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn xge_mdio_remove(ndev: *mut net_device);
}
