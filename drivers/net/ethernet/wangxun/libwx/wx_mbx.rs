//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_mbx.h
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
// Copyright (c) 2015 - 2025 Beijing WangXun Technology Co., Ltd.
pub const WX_VXMAILBOX_SIZE: c_int = 15;
// PF Registers

// VF Registers
pub const WX_VXMAILBOX: c_uint = 0x600;

pub const WX_VXMBMEM: c_uint = 0x00C00 /* 16*4B */;

// SR-IOV specific macros

pub const WX_VT_MSGINFO_SHIFT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wx_pfvf_api_rev {
    wx_mbox_api_null,
    wx_mbox_api_13 = 4,      /* API version 1.3 */
    wx_mbox_api_unknown, /* indicates that API version is not known */
}

// mailbox API
pub const WX_VF_RESET: c_uint = 0x01 /* VF requests reset */;
pub const WX_VF_SET_MAC_ADDR: c_uint = 0x02 /* VF requests PF to set MAC addr */;
pub const WX_VF_SET_MULTICAST: c_uint = 0x03 /* VF requests PF to set MC addr */;
pub const WX_VF_SET_VLAN: c_uint = 0x04 /* VF requests PF to set VLAN */;
pub const WX_VF_SET_LPE: c_uint = 0x05 /* VF requests PF to set VMOLR.LPE */;
pub const WX_VF_SET_MACVLAN: c_uint = 0x06 /* VF requests PF unicast filter */;
pub const WX_VF_API_NEGOTIATE: c_uint = 0x08 /* negotiate API version */;
pub const WX_VF_GET_QUEUES: c_uint = 0x09 /* get queue configuration */;
pub const WX_VF_GET_RETA: c_uint = 0x0a /* VF request for RETA */;
pub const WX_VF_GET_RSS_KEY: c_uint = 0x0b /* get RSS key */;
pub const WX_VF_UPDATE_XCAST_MODE: c_uint = 0x0c;
pub const WX_VF_GET_LINK_STATE: c_uint = 0x10 /* get vf link state */;
pub const WX_VF_GET_FW_VERSION: c_uint = 0x11 /* get fw version */;
pub const WX_VF_BACKUP: c_uint = 0x8001 /* VF requests backup */;

pub const WX_PF_NOFITY_VF_LINK_STATUS: c_uint = 0x1;

pub const WX_VF_PERMADDR_MSG_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wxvf_xcast_modes {
    WXVF_XCAST_MODE_NONE = 0,
    WXVF_XCAST_MODE_MULTI,
    WXVF_XCAST_MODE_ALLMULTI,
    WXVF_XCAST_MODE_PROMISC,
}

extern "C" {
    pub fn wx_write_mbx_pf(wx: *mut wx, msg: *mut u32, size: u16, vf: u16) -> c_int;
}
extern "C" {
    pub fn wx_read_mbx_pf(wx: *mut wx, msg: *mut u32, size: u16, vf: u16) -> c_int;
}
extern "C" {
    pub fn wx_check_for_rst_pf(wx: *mut wx, mbx_id: u16) -> c_int;
}
extern "C" {
    pub fn wx_check_for_msg_pf(wx: *mut wx, mbx_id: u16) -> c_int;
}
extern "C" {
    pub fn wx_check_for_ack_pf(wx: *mut wx, mbx_id: u16) -> c_int;
}
extern "C" {
    pub fn wx_read_posted_mbx(wx: *mut wx, msg: *mut u32, size: u16) -> c_int;
}
extern "C" {
    pub fn wx_write_posted_mbx(wx: *mut wx, msg: *mut u32, size: u16) -> c_int;
}
extern "C" {
    pub fn wx_check_for_rst_vf(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_check_for_msg_vf(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_read_mbx_vf(wx: *mut wx, msg: *mut u32, size: u16) -> c_int;
}
extern "C" {
    pub fn wx_write_mbx_vf(wx: *mut wx, msg: *mut u32, size: u16) -> c_int;
}
extern "C" {
    pub fn wx_init_mbx_params_vf(wx: *mut wx) -> c_int;
}
