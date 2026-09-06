//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/realtek/r8169.h
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


// SPDX-License-Identifier: GPL-2.0-only
// r8169.h: RealTek 8169/8168/8101 ethernet driver.
//
// Copyright (c) 2002 ShuChen <shuchen@realtek.com.tw>
// Copyright (c) 2003 - 2007 Francois Romieu <romieu@fr.zoreil.com>
// Copyright (c) a lot of people too. Please respect their work.
//
// See MAINTAINERS file for support contact information.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mac_version {
// support for ancient RTL_GIGA_MAC_VER_01 has been removed
    RTL_GIGA_MAC_VER_02,
    RTL_GIGA_MAC_VER_03,
    RTL_GIGA_MAC_VER_04,
    RTL_GIGA_MAC_VER_05,
    RTL_GIGA_MAC_VER_06,
    RTL_GIGA_MAC_VER_07,
    RTL_GIGA_MAC_VER_08,
    RTL_GIGA_MAC_VER_09,
    RTL_GIGA_MAC_VER_10,
// support for RTL_GIGA_MAC_VER_11 has been removed
// RTL_GIGA_MAC_VER_12 was handled the same as VER_17
// RTL_GIGA_MAC_VER_13 was merged with VER_10
    RTL_GIGA_MAC_VER_14,
// RTL_GIGA_MAC_VER_16 was merged with VER_10
    RTL_GIGA_MAC_VER_17,
    RTL_GIGA_MAC_VER_18,
    RTL_GIGA_MAC_VER_19,
    RTL_GIGA_MAC_VER_20,
    RTL_GIGA_MAC_VER_21,
    RTL_GIGA_MAC_VER_22,
    RTL_GIGA_MAC_VER_23,
    RTL_GIGA_MAC_VER_24,
    RTL_GIGA_MAC_VER_25,
    RTL_GIGA_MAC_VER_26,
// support for RTL_GIGA_MAC_VER_27 has been removed
    RTL_GIGA_MAC_VER_28,
    RTL_GIGA_MAC_VER_29,
    RTL_GIGA_MAC_VER_30,
    RTL_GIGA_MAC_VER_31,
    RTL_GIGA_MAC_VER_32,
    RTL_GIGA_MAC_VER_33,
    RTL_GIGA_MAC_VER_34,
    RTL_GIGA_MAC_VER_35,
    RTL_GIGA_MAC_VER_36,
    RTL_GIGA_MAC_VER_37,
    RTL_GIGA_MAC_VER_38,
    RTL_GIGA_MAC_VER_39,
    RTL_GIGA_MAC_VER_40,
// support for RTL_GIGA_MAC_VER_41 has been removed
    RTL_GIGA_MAC_VER_42,
    RTL_GIGA_MAC_VER_43,
    RTL_GIGA_MAC_VER_44,
// support for RTL_GIGA_MAC_VER_45 has been removed
    RTL_GIGA_MAC_VER_46,
// support for RTL_GIGA_MAC_VER_47 has been removed
    RTL_GIGA_MAC_VER_48,
// support for RTL_GIGA_MAC_VER_49 has been removed
// support for RTL_GIGA_MAC_VER_50 has been removed
    RTL_GIGA_MAC_VER_51,
    RTL_GIGA_MAC_VER_52,
// support for RTL_GIGA_MAC_VER_60 has been removed
    RTL_GIGA_MAC_VER_61,
    RTL_GIGA_MAC_VER_63,
    RTL_GIGA_MAC_VER_64,
    RTL_GIGA_MAC_VER_65,
    RTL_GIGA_MAC_VER_66,
    RTL_GIGA_MAC_VER_70,
    RTL_GIGA_MAC_VER_80,
    RTL_GIGA_MAC_NONE,
    RTL_GIGA_MAC_VER_LAST = RTL_GIGA_MAC_NONE - 1,
    RTL_GIGA_MAC_VER_EXTENDED = RTL_GIGA_MAC_NONE + 1
}

extern "C" {
    pub fn r8169_apply_firmware(tp: *mut rtl8169_private);
}
extern "C" {
    pub fn rtl8168h_2_get_adc_bias_ioffset(tp: *mut rtl8169_private) -> u16;
}
extern "C" {
    pub fn rtl8168d_efuse_read(tp: *mut rtl8169_private, reg_addr: c_int) -> u8;
}
extern "C" {
    pub fn rtl8168_get_led_mode(tp: *mut rtl8169_private) -> c_int;
}
extern "C" {
    pub fn rtl8168_led_mod_ctrl(tp: *mut rtl8169_private, mask: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn rtl8125_get_led_mode(tp: *mut rtl8169_private, index: c_int) -> c_int;
}
extern "C" {
    pub fn rtl8125_set_led_mode(tp: *mut rtl8169_private, index: c_int, mode: u16) -> c_int;
}
extern "C" {
    pub fn r8169_remove_leds(leds: *mut r8169_led_classdev);
}
