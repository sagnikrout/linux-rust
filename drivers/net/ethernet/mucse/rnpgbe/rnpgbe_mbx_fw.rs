//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mucse/rnpgbe/rnpgbe_mbx_fw.h
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
// Copyright(c) 2020 - 2025 Mucse Corporation.

pub const MUCSE_MBX_REQ_HDR_LEN: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MUCSE_FW_CMD {
    GET_HW_INFO     = 0x0601,
    GET_MAC_ADDRESS = 0x0602,
    RESET_HW        = 0x0603,
    POWER_UP        = 0x0803,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mucse_hw_info {
    pub link_stat: u8,
    pub port_mask: u8,
    pub speed: __le32,
    pub phy_type: __le16,
    pub nic_mode: __le16,
    pub pfnum: __le16,
    pub fw_version: __le32,
    pub axi_mhz: __le32,
    pub port_id: [u8; 4],
    pub port_ids: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_fw_cmd_req {
    pub flags: __le16,
    pub opcode: __le16,
    pub datalen: __le16,
    pub ret_value: __le16,
    pub cookie_lo: __le32,
    pub cookie_hi: __le32,
    pub reply_lo: __le32,
    pub reply_hi: __le32,
    pub data: [u8; 32],
    pub version: __le32,
    pub status: __le32,
    pub powerup: },
    pub port_mask: __le32,
    pub pfvf_num: __le32,
    pub get_mac_addr: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbx_fw_cmd_reply {
    pub flags: __le16,
    pub opcode: __le16,
    pub error_code: __le16,
    pub datalen: __le16,
    pub cookie_lo: __le32,
    pub cookie_hi: __le32,
    pub data: [u8; 40],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mac_addr {
    pub ports: __le32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _addr {
// for macaddr:01:02:03:04:05:06
// mac-hi=0x01020304 mac-lo=0x05060000
//
    pub mac: [u8; 8],
    pub addrs: [}; 4],
    pub mac_addr: },
    pub hw_info: mucse_hw_info,
}

// Union wrappers to expose struct as __le32 dword array for mailbox
// transport, eliminating the need for pointer casts.  The __packed
// structs have no padding, so dwords[] overlays the fields exactly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mbx_fw_cmd_req_u {
    pub r: mbx_fw_cmd_req,
    pub sizeof(__le32)]: __le32 dwords[sizeof(struct mbx_fw_cmd_req) /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mbx_fw_cmd_reply_u {
    pub r: mbx_fw_cmd_reply,
    pub sizeof(__le32)]: __le32 dwords[sizeof(struct mbx_fw_cmd_reply) /,
}

extern "C" {
    pub fn mucse_mbx_sync_fw(hw: *mut mucse_hw) -> c_int;
}
extern "C" {
    pub fn mucse_mbx_powerup(hw: *mut mucse_hw, is_powerup: bool) -> c_int;
}
extern "C" {
    pub fn mucse_mbx_reset_hw(hw: *mut mucse_hw) -> c_int;
}
