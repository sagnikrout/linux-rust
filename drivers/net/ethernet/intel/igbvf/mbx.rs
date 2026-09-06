//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igbvf/mbx.h
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
// Copyright(c) 1999 - 2018 Intel Corporation.

pub const E1000_V2PMAILBOX_REQ: c_uint = 0x00000001 /* Request for PF Ready bit */;
pub const E1000_V2PMAILBOX_ACK: c_uint = 0x00000002 /* Ack PF message received */;
pub const E1000_V2PMAILBOX_VFU: c_uint = 0x00000004 /* VF owns the mailbox buffer */;
pub const E1000_V2PMAILBOX_PFU: c_uint = 0x00000008 /* PF owns the mailbox buffer */;
pub const E1000_V2PMAILBOX_PFSTS: c_uint = 0x00000010 /* PF wrote a message in the MB */;
pub const E1000_V2PMAILBOX_PFACK: c_uint = 0x00000020 /* PF ack the previous VF msg */;
pub const E1000_V2PMAILBOX_RSTI: c_uint = 0x00000040 /* PF has reset indication */;
pub const E1000_V2PMAILBOX_RSTD: c_uint = 0x00000080 /* PF has indicated reset done */;
pub const E1000_V2PMAILBOX_R2C_BITS: c_uint = 0x000000B0 /* All read to clear bits */;

// If it's a E1000_VF_* msg then it originates in the VF and is sent to the
// PF.  The reverse is true if it is E1000_PF_*.
// Message ACK's are the value or'd with 0xF0000000
//
// Messages below or'd with this are the ACK
pub const E1000_VT_MSGTYPE_ACK: c_uint = 0x80000000;
// Messages below or'd with this are the NACK
pub const E1000_VT_MSGTYPE_NACK: c_uint = 0x40000000;
// Indicates that VF is still clear to send requests
pub const E1000_VT_MSGTYPE_CTS: c_uint = 0x20000000;
// We have a total wait time of 1s for vf mailbox posted messages

pub const E1000_VT_MSGINFO_SHIFT: c_int = 16;
// bits 23:16 are used for exra info for certain messages

pub const E1000_VF_RESET: c_uint = 0x01 /* VF requests reset */;
pub const E1000_VF_SET_MAC_ADDR: c_uint = 0x02 /* VF requests PF to set MAC addr */;
// VF requests PF to clear all unicast MAC filters

// VF requests PF to add unicast MAC filter

pub const E1000_VF_SET_MULTICAST: c_uint = 0x03 /* VF requests PF to set MC addr */;
pub const E1000_VF_SET_VLAN: c_uint = 0x04 /* VF requests PF to set VLAN */;
pub const E1000_VF_SET_LPE: c_uint = 0x05 /* VF requests PF to set VMOLR.LPE */;
pub const E1000_PF_CONTROL_MSG: c_uint = 0x0100 /* PF control message */;
extern "C" {
    pub fn e1000_init_mbx_params_vf(: *mut e1000_hw) -> i32;
}
