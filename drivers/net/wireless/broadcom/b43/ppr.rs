//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/ppr.h
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

pub const B43_PPR_CCK_RATES_NUM: c_int = 4;
pub const B43_PPR_OFDM_RATES_NUM: c_int = 8;
pub const B43_PPR_MCS_RATES_NUM: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_ppr_rates {
    pub cck: [u8; B43_PPR_CCK_RATES_NUM],
    pub ofdm: [u8; B43_PPR_OFDM_RATES_NUM],
    pub ofdm_20_cdd: [u8; B43_PPR_OFDM_RATES_NUM],
    pub /: *mut *mut u8 mcs_20[B43_PPR_MCS_RATES_NUM]; / SISO,
    pub mcs_20_cdd: [u8; B43_PPR_MCS_RATES_NUM],
    pub mcs_20_stbc: [u8; B43_PPR_MCS_RATES_NUM],
    pub mcs_20_sdm: [u8; B43_PPR_MCS_RATES_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_ppr {
// All powers are in qdbm (Q5.2)
    pub __all_rates: [u8; B43_PPR_RATES_NUM],
    pub rates: b43_ppr_rates,
}

extern "C" {
    pub fn b43_ppr_clear(dev: *mut b43_wldev, ppr: *mut b43_ppr);
}
extern "C" {
    pub fn b43_ppr_add(dev: *mut b43_wldev, ppr: *mut b43_ppr, diff: c_int);
}
extern "C" {
    pub fn b43_ppr_apply_max(dev: *mut b43_wldev, ppr: *mut b43_ppr, max: u8);
}
extern "C" {
    pub fn b43_ppr_apply_min(dev: *mut b43_wldev, ppr: *mut b43_ppr, min: u8);
}
extern "C" {
    pub fn b43_ppr_get_max(dev: *mut b43_wldev, ppr: *mut b43_ppr) -> u8;
}
