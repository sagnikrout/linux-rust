//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ixgbe/ixgbe_mbx.h
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

pub const IXGBE_VFMAILBOX: c_uint = 0x002FC;
pub const IXGBE_VFMBMEM: c_uint = 0x00200;
pub const IXGBE_PFMAILBOX_STS: c_uint = 0x00000001 /* Initiate message send to VF */;
pub const IXGBE_PFMAILBOX_ACK: c_uint = 0x00000002 /* Ack message recv'd from VF */;
pub const IXGBE_PFMAILBOX_VFU: c_uint = 0x00000004 /* VF owns the mailbox buffer */;
pub const IXGBE_PFMAILBOX_PFU: c_uint = 0x00000008 /* PF owns the mailbox buffer */;
pub const IXGBE_PFMAILBOX_RVFU: c_uint = 0x00000010 /* Reset VFU - used when VF stuck */;
pub const IXGBE_MBVFICR_VFREQ_MASK: c_uint = 0x0000FFFF /* bits for VF messages */;
pub const IXGBE_MBVFICR_VFREQ_VF1: c_uint = 0x00000001 /* bit for VF 1 message */;
pub const IXGBE_MBVFICR_VFACK_MASK: c_uint = 0xFFFF0000 /* bits for VF acks */;
pub const IXGBE_MBVFICR_VFACK_VF1: c_uint = 0x00010000 /* bit for VF 1 ack */;
// If it's a IXGBE_VF_* msg then it originates in the VF and is sent to the
// PF.  The reverse is true if it is IXGBE_PF_*.
// Message ACK's are the value or'd with 0xF0000000
//
pub const IXGBE_VT_MSGTYPE_ACK: c_uint = 0x80000000  /* Messages below or'd with;
// this are the ACK
pub const IXGBE_VT_MSGTYPE_NACK: c_uint = 0x40000000  /* Messages below or'd with;
// this are the NACK
pub const IXGBE_VT_MSGTYPE_CTS: c_uint = 0x20000000  /* Indicates that VF is still;
pub const IXGBE_VT_MSGINFO_SHIFT: c_int = 16;
// bits 23:16 are used for extra info for certain messages

// definitions to support mailbox API version negotiation
//
// Each element denotes a version of the API; existing numbers may not
// change; any additions must go at the end
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ixgbe_pfvf_api_rev {
    ixgbe_mbox_api_10,	/* API version 1.0, linux/freebsd VF driver */
    ixgbe_mbox_api_20,	/* API version 2.0, solaris Phase1 VF driver */
    ixgbe_mbox_api_11,	/* API version 1.1, linux/freebsd VF driver */
    ixgbe_mbox_api_12,	/* API version 1.2, linux/freebsd VF driver */
    ixgbe_mbox_api_13,	/* API version 1.3, linux/freebsd VF driver */
    ixgbe_mbox_api_14,	/* API version 1.4, linux/freebsd VF driver */
    ixgbe_mbox_api_15,	/* API version 1.5, linux/freebsd VF driver */
    ixgbe_mbox_api_16,	/* API version 1.6, linux/freebsd VF driver */
    ixgbe_mbox_api_17,	/* API version 1.7, linux/freebsd VF driver */
// This value should always be last
    ixgbe_mbox_api_unknown,	/* indicates that API version is not known */
}

// mailbox API, legacy requests
pub const IXGBE_VF_RESET: c_uint = 0x01 /* VF requests reset */;
pub const IXGBE_VF_SET_MAC_ADDR: c_uint = 0x02 /* VF requests PF to set MAC addr */;
pub const IXGBE_VF_SET_MULTICAST: c_uint = 0x03 /* VF requests PF to set MC addr */;
pub const IXGBE_VF_SET_VLAN: c_uint = 0x04 /* VF requests PF to set VLAN */;
// mailbox API, version 1.0 VF requests
pub const IXGBE_VF_SET_LPE: c_uint = 0x05 /* VF requests PF to set VMOLR.LPE */;
pub const IXGBE_VF_SET_MACVLAN: c_uint = 0x06 /* VF requests PF for unicast filter */;
pub const IXGBE_VF_API_NEGOTIATE: c_uint = 0x08 /* negotiate API version */;
// mailbox API, version 1.1 VF requests
pub const IXGBE_VF_GET_QUEUES: c_uint = 0x09 /* get queue configuration */;
// GET_QUEUES return data indices within the mailbox

// mailbox API, version 1.2 VF requests
pub const IXGBE_VF_GET_RETA: c_uint = 0x0a	/* VF request for RETA */;
pub const IXGBE_VF_GET_RSS_KEY: c_uint = 0x0b	/* get RSS key */;
pub const IXGBE_VF_UPDATE_XCAST_MODE: c_uint = 0x0c;
// mailbox API, version 1.4 VF requests
pub const IXGBE_VF_IPSEC_ADD: c_uint = 0x0d;
pub const IXGBE_VF_IPSEC_DEL: c_uint = 0x0e;
pub const IXGBE_VF_GET_LINK_STATE: c_uint = 0x10 /* get vf link state */;
// mailbox API, version 1.6 VF requests
pub const IXGBE_VF_GET_PF_LINK_STATE: c_uint = 0x11 /* request PF to send link info */;
// mailbox API, version 1.7 VF requests
pub const IXGBE_VF_FEATURES_NEGOTIATE: c_uint = 0x12 /* get features supported by PF */;
// length of permanent address message returned from PF
pub const IXGBE_VF_PERMADDR_MSG_LEN: c_int = 4;
// word in permanent address message with the current multicast type
pub const IXGBE_VF_MC_TYPE_WORD: c_int = 3;
pub const IXGBE_PF_CONTROL_MSG: c_uint = 0x0100 /* PF control message */;

// features negotiated between PF/VF

extern "C" {
    pub fn ixgbe_read_mbx(: *mut ixgbe_hw, : *mut u32, _arg: u16, _arg: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_write_mbx(: *mut ixgbe_hw, : *mut u32, _arg: u16, _arg: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_check_for_msg(: *mut ixgbe_hw, _arg: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_check_for_ack(: *mut ixgbe_hw, _arg: u16) -> c_int;
}
extern "C" {
    pub fn ixgbe_check_for_rst(: *mut ixgbe_hw, _arg: u16) -> c_int;
}

extern "C" {
    pub fn ixgbe_init_mbx_params_pf(: *mut ixgbe_hw);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixgbe_mbx_operations {
    pub hw): *mut *mut int (init_params)(struct ixgbe_hw,
    pub vf_number): *mut *mut *mut *mut int (read)(struct ixgbe_hw hw, u32 msg, u16 size, u16,
    pub vf_number): *mut *mut *mut *mut int (write)(struct ixgbe_hw hw, u32 msg, u16 size, u16,
    pub mbx_id): *mut *mut *mut *mut int (read_posted)(struct ixgbe_hw hw, u32 msg, u16 size, u16,
    pub mbx_id): u16,
    pub vf_number): *mut *mut *mut int (check_for_msg)(struct ixgbe_hw hw, u16,
    pub vf_number): *mut *mut *mut int (check_for_ack)(struct ixgbe_hw hw, u16,
    pub vf_number): *mut *mut *mut int (check_for_rst)(struct ixgbe_hw hw, u16,
}
