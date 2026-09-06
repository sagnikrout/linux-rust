//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_vf.h
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
// Control registers
pub const WX_VF_MAX_RING_NUMS: c_int = 8;
pub const WX_VX_PF_BME: c_uint = 0x4B8;

pub const WX_VXSTATUS: c_uint = 0x4;
pub const WX_VXCTRL: c_uint = 0x8;

pub const WX_VXMRQC: c_uint = 0x78;

// Interrupt registers
pub const WX_VXICR: c_uint = 0x100;
pub const WX_VXIMS: c_uint = 0x108;
pub const WX_VXIMC: c_uint = 0x10C;
pub const WX_VF_IRQ_CLEAR_MASK: c_int = 7;
pub const WX_VF_MAX_TX_QUEUES: c_int = 4;
pub const WX_VF_MAX_RX_QUEUES: c_int = 4;

pub const WX_VXIVAR_MISC: c_uint = 0x260;

pub const WX_RX_HDR_SIZE: c_int = 256;
pub const WX_RX_BUF_SIZE: c_int = 2048;

pub const WX_RXBUFFER_3072: c_int = 3072;
// Receive Path

// Transimit Path

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wx_link_reg_fields {
    pub mac_type: u32,
    pub bit0_f: u32,
    pub bit1_f: u32,
    pub bit2_f: u32,
    pub bit3_f: u32,
    pub bit4_f: u32,
}

extern "C" {
    pub fn wx_init_hw_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_reset_hw_vf(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_get_mac_addr_vf(wx: *mut wx, mac_addr: *mut u8);
}
extern "C" {
    pub fn wx_stop_adapter_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_get_fw_version_vf(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_set_rar_vf(wx: *mut wx, index: u32, addr: *mut u8, enable_addr: u32) -> c_int;
}
extern "C" {
    pub fn wx_update_mc_addr_list_vf(wx: *mut wx, netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn wx_set_uc_addr_vf(wx: *mut wx, index: u32, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn wx_rlpml_set_vf(wx: *mut wx, max_size: u16) -> c_int;
}
extern "C" {
    pub fn wx_negotiate_api_version(wx: *mut wx, api: c_int) -> c_int;
}
extern "C" {
    pub fn wx_get_queues_vf(wx: *mut wx, num_tcs: *mut u32, default_tc: *mut u32) -> c_int;
}
extern "C" {
    pub fn wx_update_xcast_mode_vf(wx: *mut wx, xcast_mode: c_int) -> c_int;
}
extern "C" {
    pub fn wx_get_link_state_vf(wx: *mut wx, link_state: *mut u16) -> c_int;
}
extern "C" {
    pub fn wx_check_mac_link_vf(wx: *mut wx) -> c_int;
}
