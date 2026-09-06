//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/e1000e/manage.h
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
extern "C" {
    pub fn e1000e_check_mng_mode_generic(hw: *mut e1000_hw) -> bool;
}
extern "C" {
    pub fn e1000e_enable_tx_pkt_filtering(hw: *mut e1000_hw) -> bool;
}
extern "C" {
    pub fn e1000e_mng_write_dhcp_info(hw: *mut e1000_hw, buffer: *mut u8, length: u16) -> i32;
}
extern "C" {
    pub fn e1000e_enable_mng_pass_thru(hw: *mut e1000_hw) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum e1000_mng_mode {
    e1000_mng_mode_none = 0,
    e1000_mng_mode_asf,
    e1000_mng_mode_pt,
    e1000_mng_mode_ipmi,
    e1000_mng_mode_host_if_only
}

pub const E1000_FACTPS_MNGCG: c_uint = 0x20000000;
pub const E1000_FWSM_MODE_MASK: c_uint = 0xE;
pub const E1000_FWSM_MODE_SHIFT: c_int = 1;
pub const E1000_MNG_IAMT_MODE: c_uint = 0x3;
pub const E1000_MNG_DHCP_COOKIE_LENGTH: c_uint = 0x10;
pub const E1000_MNG_DHCP_COOKIE_OFFSET: c_uint = 0x6F0;
pub const E1000_MNG_DHCP_COMMAND_TIMEOUT: c_int = 10;
pub const E1000_MNG_DHCP_TX_PAYLOAD_CMD: c_int = 64;
pub const E1000_MNG_DHCP_COOKIE_STATUS_PARSING: c_uint = 0x1;
pub const E1000_MNG_DHCP_COOKIE_STATUS_VLAN: c_uint = 0x2;
pub const E1000_VFTA_ENTRY_SHIFT: c_int = 5;
pub const E1000_VFTA_ENTRY_MASK: c_uint = 0x7F;
pub const E1000_VFTA_ENTRY_BIT_SHIFT_MASK: c_uint = 0x1F;
pub const E1000_HICR_EN: c_uint = 0x01	/* Enable bit - RO */;
// Driver sets this bit when done to put command in RAM
pub const E1000_HICR_C: c_uint = 0x02;
pub const E1000_HICR_SV: c_uint = 0x04	/* Status Validity */;
pub const E1000_HICR_FW_RESET_ENABLE: c_uint = 0x40;
pub const E1000_HICR_FW_RESET: c_uint = 0x80;
// Intel(R) Active Management Technology signature
pub const E1000_IAMT_SIGNATURE: c_uint = 0x544D4149;
