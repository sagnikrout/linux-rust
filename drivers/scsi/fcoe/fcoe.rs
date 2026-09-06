//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/fcoe/fcoe.h
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
// Copyright(c) 2009 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

pub const FCOE_MAX_QUEUE_DEPTH: c_int = 256;
pub const FCOE_MIN_QUEUE_DEPTH: c_int = 32;
pub const FCOE_WORD_TO_BYTE: c_int = 4;

pub const FCOE_MAX_LUN: c_uint = 0xFFFF;
pub const FCOE_MAX_FCP_TARGET: c_int = 256;
pub const FCOE_MAX_OUTSTANDING_COMMANDS: c_int = 1024;
pub const FCOE_MIN_XID: c_uint = 0x0000	/* the min xid supported by fcoe_sw */;
pub const FCOE_MAX_XID: c_uint = 0x0FFF	/* the max xid supported by fcoe_sw */;
pub const FCOE_LOGGING: c_uint = 0x01 /* General logging, not categorized */;
pub const FCOE_NETDEV_LOGGING: c_uint = 0x02 /* Netdevice logging */;

//
// struct fcoe_interface - A FCoE interface
// @list:	      Handle for a list of FCoE interfaces
// @netdev:	      The associated net device
// @fcoe_packet_type: FCoE packet type
// @fip_packet_type:  FIP packet type
// @oem:	      The offload exchange manager for all local port
// instances associated with this port
// @removed:	      Indicates fcoe interface removed from net device
// @priority:	      Priority for the FCoE packet (DCB)
// This structure is 1:1 with a net device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcoe_interface {
    pub list: list_head,
    pub netdev: *mut net_device,
    pub realdev: *mut net_device,
    pub fcoe_packet_type: packet_type,
    pub fip_packet_type: packet_type,
    pub fip_vlan_packet_type: packet_type,
    pub oem: *mut fc_exch_mgr,
    pub removed: u8,
    pub priority: u8,
}

//
// fcoe_netdev() - Return the net device associated with a local port
// @lport: The local port to get the net device from
//
