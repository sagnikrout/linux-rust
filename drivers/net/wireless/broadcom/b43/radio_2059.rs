//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/radio_2059.h
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

pub const R2059_C1: c_uint = 0x000;
pub const R2059_C2: c_uint = 0x400;
pub const R2059_C3: c_uint = 0x800;
pub const R2059_ALL: c_uint = 0xC00;
pub const R2059_RCAL_CONFIG: c_uint = 0x004;
pub const R2059_RFPLL_MASTER: c_uint = 0x011;
pub const R2059_RFPLL_MISC_EN: c_uint = 0x02b;
pub const R2059_RFPLL_MISC_CAL_RESETN: c_uint = 0x02e;
pub const R2059_XTAL_CONFIG2: c_uint = 0x0c0;
pub const R2059_RCCAL_START_R1_Q1_P1: c_uint = 0x13c;
pub const R2059_RCCAL_X1: c_uint = 0x13d;
pub const R2059_RCCAL_TRC0: c_uint = 0x13e;
pub const R2059_RCCAL_DONE_OSCCAP: c_uint = 0x140;
pub const R2059_RCAL_STATUS: c_uint = 0x145;
pub const R2059_RCCAL_MASTER: c_uint = 0x17f;
// Values for various registers uploaded on channel switching
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_ht_channeltab_e_radio2059 {
// The channel frequency in MHz
    pub freq: u16,
// Values for radio registers
    pub radio_syn16: u8,
    pub radio_syn17: u8,
    pub radio_syn22: u8,
    pub radio_syn25: u8,
    pub radio_syn27: u8,
    pub radio_syn28: u8,
    pub radio_syn29: u8,
    pub radio_syn2c: u8,
    pub radio_syn2d: u8,
    pub radio_syn37: u8,
    pub radio_syn41: u8,
    pub radio_syn43: u8,
    pub radio_syn47: u8,
    pub radio_rxtx4a: u8,
    pub radio_rxtx58: u8,
    pub radio_rxtx5a: u8,
    pub radio_rxtx6a: u8,
    pub radio_rxtx6d: u8,
    pub radio_rxtx6e: u8,
    pub radio_rxtx92: u8,
    pub radio_rxtx98: u8,
// Values for PHY registers
    pub phy_regs: b43_phy_ht_channeltab_e_phy,
}

extern "C" {
    pub fn r2059_upload_inittabs(dev: *mut b43_wldev);
}
// b43_phy_ht_get_channeltab_e_r2059(struct b43_wldev *dev, u16 freq);
