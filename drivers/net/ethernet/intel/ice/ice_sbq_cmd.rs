//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_sbq_cmd.h
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
// Copyright (C) 2021, Intel Corporation.
// This header file defines the Sideband Queue commands, error codes and
// descriptor format. It is shared between Firmware and Software.
//
// Sideband Queue command structure and opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sbq_opc {
// Sideband Queue commands
    ice_sbq_opc_neigh_dev_req			= 0x0C00,
    ice_sbq_opc_neigh_dev_ev			= 0x0C01
}

// Sideband Queue descriptor. Indirect command
// and non posted
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sbq_cmd_desc {
    pub flags: __le16,
    pub opcode: __le16,
    pub datalen: __le16,
    pub cmd_retval: __le16,
// Opaque message data
    pub cookie_high: __le32,
    pub cookie_low: __le32,
    pub cmd_len: __le16,
    pub cmpl_len: __le16,
    pub param0: },
    pub reserved: [u8; 6],
    pub addr_high: __le32,
    pub addr_low: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sbq_evt_desc {
    pub flags: __le16,
    pub opcode: __le16,
    pub datalen: __le16,
    pub cmd_retval: __le16,
    pub data: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sbq_dev_id {
    ice_sbq_dev_phy_0	= 0x02,
    ice_sbq_dev_cgu		= 0x06,
    ice_sbq_dev_phy_0_peer	= 0x0D,
    ice_sbq_dev_cgu_peer	= 0x0F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_sbq_msg_opcode {
    ice_sbq_msg_rd		= 0x00,
    ice_sbq_msg_wr		= 0x01,
    ice_sbq_msg_wr_np	= 0x02
}

pub const ICE_SBQ_MSG_FLAGS: c_uint = 0x40;
pub const ICE_SBQ_MSG_SBE_FBE: c_uint = 0x0F;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sbq_msg_req {
    pub dest_dev: u8,
    pub src_dev: u8,
    pub opcode: u8,
    pub flags: u8,
    pub sbe_fbe: u8,
    pub func_id: u8,
    pub msg_addr_low: __le16,
    pub msg_addr_high: __le32,
    pub data: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sbq_msg_cmpl {
    pub dest_dev: u8,
    pub src_dev: u8,
    pub opcode: u8,
    pub flags: u8,
    pub data: __le32,
}

// Internal struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sbq_msg_input {
    pub dest_dev: u8,
    pub opcode: u8,
    pub msg_addr_low: u16,
    pub msg_addr_high: u32,
    pub data: u32,
}
