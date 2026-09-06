//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_ethtool.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2014-2016 Broadcom Corporation
// Copyright (c) 2016-2017 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_led_cfg {
    pub led_id: u8,
    pub led_state: u8,
    pub led_color: u8,
    pub unused: u8,
    pub led_blink_on: __le16,
    pub led_blink_off: __le16,
    pub led_group_id: u8,
    pub rsvd: u8,
}

pub const BNXT_LED_DFLT_ENA_SHIFT: c_int = 6;

pub const BNXT_PXP_REG_LEN: c_uint = 0x3110;
pub const BNXT_IP_PROTO_FULL_MASK: c_uint = 0xFF;
pub const BNXT_IP_PROTO_WILDCARD: c_uint = 0x0;
extern "C" {
    pub fn bnxt_get_rxfh_indir_size(dev: *mut net_device) -> u32;
}
extern "C" {
    pub fn _bnxt_fw_to_linkmode(mode: *mut c_ulong, fw_speeds: u16);
}
extern "C" {
    pub fn bnxt_fw_to_ethtool_speed(_arg: u16) -> u32;
}
extern "C" {
    pub fn bnxt_get_fw_auto_link_speeds(mode: *const c_ulong) -> u16;
}
extern "C" {
    pub fn bnxt_get_pkginfo(dev: *mut net_device, ver: *mut c_char, size: c_int) -> c_int;
}
extern "C" {
    pub fn bnxt_ethtool_init(bp: *mut bnxt);
}
extern "C" {
    pub fn bnxt_ethtool_free(bp: *mut bnxt);
}
