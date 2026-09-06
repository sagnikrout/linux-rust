//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/cxgb4_tc_flower.h
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


//
// This file is part of the Chelsio T4/T5/T6 Ethernet driver for Linux.
//
// Copyright (c) 2017 Chelsio Communications, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tc_flower_stats {
    pub prev_packet_count: u64,
    pub packet_count: u64,
    pub byte_count: u64,
    pub last_used: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tc_flower_entry {
    pub fs: ch_filter_specification,
    pub stats: ch_tc_flower_stats,
    pub tc_flower_cookie: c_ulong,
    pub node: rhash_head,
    pub rcu: rcu_head,
    pub /: *mut *mut spinlock_t lock; / lock for stats,
    pub filter_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch_tc_pedit_fields {
    pub field: u8,
    pub size: u8,
    pub offset: u32,
}

pub const PEDIT_ETH_DMAC_MASK: c_uint = 0xffff;
pub const PEDIT_TCP_UDP_SPORT_MASK: c_uint = 0xffff;
pub const PEDIT_ETH_DMAC_31_0: c_uint = 0x0;
pub const PEDIT_ETH_DMAC_47_32_SMAC_15_0: c_uint = 0x4;
pub const PEDIT_ETH_SMAC_47_16: c_uint = 0x8;
pub const PEDIT_IP4_SRC: c_uint = 0xC;
pub const PEDIT_IP4_DST: c_uint = 0x10;
pub const PEDIT_IP6_SRC_31_0: c_uint = 0x8;
pub const PEDIT_IP6_SRC_63_32: c_uint = 0xC;
pub const PEDIT_IP6_SRC_95_64: c_uint = 0x10;
pub const PEDIT_IP6_SRC_127_96: c_uint = 0x14;
pub const PEDIT_IP6_DST_31_0: c_uint = 0x18;
pub const PEDIT_IP6_DST_63_32: c_uint = 0x1C;
pub const PEDIT_IP6_DST_95_64: c_uint = 0x20;
pub const PEDIT_IP6_DST_127_96: c_uint = 0x24;
pub const PEDIT_TCP_SPORT_DPORT: c_uint = 0x0;
pub const PEDIT_UDP_SPORT_DPORT: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxgb4_action_natmode_flags {
    CXGB4_ACTION_NATMODE_NONE = 0,
    CXGB4_ACTION_NATMODE_DIP = (1 << 0),
    CXGB4_ACTION_NATMODE_SIP = (1 << 1),
    CXGB4_ACTION_NATMODE_DPORT = (1 << 2),
    CXGB4_ACTION_NATMODE_SPORT = (1 << 3),
}

// TC PEDIT action to NATMODE translation entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxgb4_natmode_config {
    pub chip: chip_type,
    pub flags: u8,
    pub natmode: u8,
}

extern "C" {
    pub fn cxgb4_init_tc_flower(adap: *mut adapter) -> c_int;
}
extern "C" {
    pub fn cxgb4_cleanup_tc_flower(adap: *mut adapter);
}
