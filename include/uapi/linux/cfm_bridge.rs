//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cfm_bridge.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note

pub const CFM_MAID_LENGTH: c_int = 48;
pub const CFM_CCM_PDU_LENGTH: c_int = 75;
pub const CFM_PORT_STATUS_TLV_LENGTH: c_int = 4;
pub const CFM_IF_STATUS_TLV_LENGTH: c_int = 4;
pub const CFM_IF_STATUS_TLV_TYPE: c_int = 4;
pub const CFM_PORT_STATUS_TLV_TYPE: c_int = 2;
pub const CFM_ENDE_TLV_TYPE: c_int = 0;

pub const CFM_FRAME_PRIO: c_int = 7;
pub const CFM_CCM_TLV_OFFSET: c_int = 70;
pub const CFM_CCM_PDU_MAID_OFFSET: c_int = 10;
pub const CFM_CCM_PDU_MEPID_OFFSET: c_int = 8;
pub const CFM_CCM_PDU_SEQNR_OFFSET: c_int = 4;
pub const CFM_CCM_PDU_TLV_OFFSET: c_int = 74;
pub const CFM_CCM_ITU_RESERVED_SIZE: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct br_cfm_common_hdr {
    pub mdlevel_version: __u8,
    pub opcode: __u8,
    pub flags: __u8,
    pub tlv_offset: __u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_cfm_opcodes {
    BR_CFM_OPCODE_CCM = 0x1,
}

// MEP domain
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_cfm_domain {
    BR_CFM_PORT,
    BR_CFM_VLAN,
}

// MEP direction
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_cfm_mep_direction {
    BR_CFM_MEP_DIRECTION_DOWN,
    BR_CFM_MEP_DIRECTION_UP,
}

// CCM interval supported.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_cfm_ccm_interval {
    BR_CFM_CCM_INTERVAL_NONE,
    BR_CFM_CCM_INTERVAL_3_3_MS,
    BR_CFM_CCM_INTERVAL_10_MS,
    BR_CFM_CCM_INTERVAL_100_MS,
    BR_CFM_CCM_INTERVAL_1_SEC,
    BR_CFM_CCM_INTERVAL_10_SEC,
    BR_CFM_CCM_INTERVAL_1_MIN,
    BR_CFM_CCM_INTERVAL_10_MIN,
}
