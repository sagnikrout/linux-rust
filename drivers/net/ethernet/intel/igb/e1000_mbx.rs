//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/igb/e1000_mbx.h
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
// Copyright(c) 2007 - 2018 Intel Corporation.

pub const E1000_P2VMAILBOX_STS: c_uint = 0x00000001 /* Initiate message send to VF */;
pub const E1000_P2VMAILBOX_ACK: c_uint = 0x00000002 /* Ack message recv'd from VF */;
pub const E1000_P2VMAILBOX_VFU: c_uint = 0x00000004 /* VF owns the mailbox buffer */;
pub const E1000_P2VMAILBOX_PFU: c_uint = 0x00000008 /* PF owns the mailbox buffer */;
pub const E1000_P2VMAILBOX_RVFU: c_uint = 0x00000010 /* Reset VFU - used when VF stuck */;
pub const E1000_MBVFICR_VFREQ_MASK: c_uint = 0x000000FF /* bits for VF messages */;
pub const E1000_MBVFICR_VFREQ_VF1: c_uint = 0x00000001 /* bit for VF 1 message */;
pub const E1000_MBVFICR_VFACK_MASK: c_uint = 0x00FF0000 /* bits for VF acks */;
pub const E1000_MBVFICR_VFACK_VF1: c_uint = 0x00010000 /* bit for VF 1 ack */;

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
pub const E1000_VT_MSGINFO_SHIFT: c_int = 16;
// bits 23:16 are used for extra info for certain messages

pub const E1000_VF_RESET: c_uint = 0x01 /* VF requests reset */;
pub const E1000_VF_SET_MAC_ADDR: c_uint = 0x02 /* VF requests to set MAC addr */;
// VF requests to clear all unicast MAC filters

// VF requests to add unicast MAC filter

pub const E1000_VF_SET_MULTICAST: c_uint = 0x03 /* VF requests to set MC addr */;
pub const E1000_VF_SET_VLAN: c_uint = 0x04 /* VF requests to set VLAN */;
pub const E1000_VF_SET_LPE: c_uint = 0x05 /* VF requests to set VMOLR.LPE */;
pub const E1000_VF_SET_PROMISC: c_uint = 0x06 /*VF requests to clear VMOLR.ROPE/MPME*/;

pub const E1000_PF_CONTROL_MSG: c_uint = 0x0100 /* PF control message */;
extern "C" {
    pub fn igb_write_mbx(hw: *mut e1000_hw, msg: *mut u32, size: u16, mbx_id: u16) -> i32;
}
extern "C" {
    pub fn igb_check_for_msg(hw: *mut e1000_hw, mbx_id: u16) -> i32;
}
extern "C" {
    pub fn igb_check_for_ack(hw: *mut e1000_hw, mbx_id: u16) -> i32;
}
extern "C" {
    pub fn igb_check_for_rst(hw: *mut e1000_hw, mbx_id: u16) -> i32;
}
extern "C" {
    pub fn igb_unlock_mbx(hw: *mut e1000_hw, mbx_id: u16) -> i32;
}
extern "C" {
    pub fn igb_init_mbx_params_pf(hw: *mut e1000_hw) -> i32;
}
