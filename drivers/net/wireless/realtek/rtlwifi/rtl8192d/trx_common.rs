//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/realtek/rtlwifi/rtl8192d/trx_common.h
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
// Copyright(c) 2009-2012  Realtek Corporation.
pub const RX_DRV_INFO_SIZE_UNIT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl92d_rx_desc_enc {
    RX_DESC_ENC_NONE	= 0,
    RX_DESC_ENC_WEP40	= 1,
    RX_DESC_ENC_TKIP_WO_MIC	= 2,
    RX_DESC_ENC_TKIP_MIC	= 3,
    RX_DESC_ENC_AES		= 4,
    RX_DESC_ENC_WEP104	= 5,
}

// macros to read/write various fields in RX or TX descriptors
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(31)) -> return;
}
// (__pdesc + 8) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_to_cpu(8): *mut *mut (__pdesc +) -> return;
}
// (__pdesc + 10) = cpu_to_le32(__val);
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(13, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(19, _arg: 16)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(22, _arg: 20)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: GENMASK(25, _arg: 24)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(26)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(27)) -> return;
}
extern "C" {
    pub fn le32_get_bits(_arg: *mut __pdesc, _arg: BIT(31)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: BIT(14)) -> return;
}
extern "C" {
    pub fn le32_get_bits(1): *mut *mut (__pdesc +, _arg: BIT(15)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: GENMASK(5, _arg: 0)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(6)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(8)) -> return;
}
extern "C" {
    pub fn le32_get_bits(3): *mut *mut (__pdesc +, _arg: BIT(9)) -> return;
}
extern "C" {
    pub fn le32_to_cpu(5): *mut *mut (__pdesc +) -> return;
}
extern "C" {
    pub fn le32_to_cpu(6): *mut *mut (__pdesc +) -> return;
}
// (__pdesc + 6) = cpu_to_le32(__val);
// For 92D early mode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_fwinfo_92d {
    pub gain_trsw: [u8; 4],
    pub pwdb_all: u8,
    pub cfosho: [u8; 4],
    pub cfotail: [u8; 4],
    pub rxevm: [i8; 2],
    pub rxsnr: [i8; 4],
    pub pdsnr: [u8; 2],
    pub csi_current: [u8; 2],
    pub csi_target: [u8; 2],
    pub sigevm: u8,
    pub max_ex_pwr: u8,

    pub ex_intf_flag:1: u8,
    pub sgi_en:1: u8,
    pub rxsc:2: u8,
    pub reserve:4: u8,

    pub reserve:4: u8,
    pub rxsc:2: u8,
    pub sgi_en:1: u8,
    pub ex_intf_flag:1: u8,

    pub __packed: },
    pub skb): *mut *mut u8 pdesc, struct sk_buff,
    pub val): *mut u8 desc_name, u8,
    pub desc_name): *mut *mut u8 p_desc, bool istx, u8,
