//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene-v2/enet.h
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
pub const ENET_CLKEN: c_uint = 0xc008;
pub const ENET_SRST: c_uint = 0xc000;
pub const ENET_SHIM: c_uint = 0xc010;
pub const CFG_MEM_RAM_SHUTDOWN: c_uint = 0xd070;
pub const BLOCK_MEM_RDY: c_uint = 0xd074;
pub const MEM_RDY: c_uint = 0xffffffff;

pub const CFG_FORCE_LINK_STATUS_EN: c_uint = 0x229c;
pub const FORCE_LINK_STATUS: c_uint = 0x22a0;
pub const CFG_LINK_AGGR_RESUME: c_uint = 0x27c8;
pub const RX_DV_GATE_REG: c_uint = 0x2dfc;
extern "C" {
    pub fn xge_wr_csr(pdata: *mut xge_pdata, offset: u32, val: u32);
}
extern "C" {
    pub fn xge_rd_csr(pdata: *mut xge_pdata, offset: u32) -> u32;
}
extern "C" {
    pub fn xge_port_reset(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn xge_port_init(ndev: *mut net_device);
}
