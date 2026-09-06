//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mrp_bridge.h
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

pub const MRP_MAX_FRAME_LENGTH: c_int = 200;
pub const MRP_DEFAULT_PRIO: c_uint = 0x8000;
pub const MRP_DOMAIN_UUID_LENGTH: c_int = 16;
pub const MRP_VERSION: c_int = 1;
pub const MRP_FRAME_PRIO: c_int = 7;
pub const MRP_OUI_LENGTH: c_int = 3;
pub const MRP_MANUFACTURE_DATA_LENGTH: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_ring_role_type {
    BR_MRP_RING_ROLE_DISABLED,
    BR_MRP_RING_ROLE_MRC,
    BR_MRP_RING_ROLE_MRM,
    BR_MRP_RING_ROLE_MRA,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_in_role_type {
    BR_MRP_IN_ROLE_DISABLED,
    BR_MRP_IN_ROLE_MIC,
    BR_MRP_IN_ROLE_MIM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_ring_state_type {
    BR_MRP_RING_STATE_OPEN,
    BR_MRP_RING_STATE_CLOSED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_in_state_type {
    BR_MRP_IN_STATE_OPEN,
    BR_MRP_IN_STATE_CLOSED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_port_state_type {
    BR_MRP_PORT_STATE_DISABLED,
    BR_MRP_PORT_STATE_BLOCKED,
    BR_MRP_PORT_STATE_FORWARDING,
    BR_MRP_PORT_STATE_NOT_CONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_port_role_type {
    BR_MRP_PORT_ROLE_PRIMARY,
    BR_MRP_PORT_ROLE_SECONDARY,
    BR_MRP_PORT_ROLE_INTER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_tlv_header_type {
    BR_MRP_TLV_HEADER_END = 0x0,
    BR_MRP_TLV_HEADER_COMMON = 0x1,
    BR_MRP_TLV_HEADER_RING_TEST = 0x2,
    BR_MRP_TLV_HEADER_RING_TOPO = 0x3,
    BR_MRP_TLV_HEADER_RING_LINK_DOWN = 0x4,
    BR_MRP_TLV_HEADER_RING_LINK_UP = 0x5,
    BR_MRP_TLV_HEADER_IN_TEST = 0x6,
    BR_MRP_TLV_HEADER_IN_TOPO = 0x7,
    BR_MRP_TLV_HEADER_IN_LINK_DOWN = 0x8,
    BR_MRP_TLV_HEADER_IN_LINK_UP = 0x9,
    BR_MRP_TLV_HEADER_IN_LINK_STATUS = 0xa,
    BR_MRP_TLV_HEADER_OPTION = 0x7f,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum br_mrp_sub_tlv_header_type {
    BR_MRP_SUB_TLV_HEADER_TEST_MGR_NACK = 0x1,
    BR_MRP_SUB_TLV_HEADER_TEST_PROPAGATE = 0x2,
    BR_MRP_SUB_TLV_HEADER_TEST_AUTO_MGR = 0x3,
}
