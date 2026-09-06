//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/b43/phy_ac.h
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

pub const B43_PHY_AC_BBCFG: c_uint = 0x001;
pub const B43_PHY_AC_BBCFG_RSTCCA: c_uint = 0x4000	/* Reset CCA */;
pub const B43_PHY_AC_BANDCTL: c_uint = 0x003	/* Band control */;
pub const B43_PHY_AC_BANDCTL_5GHZ: c_uint = 0x0001;
pub const B43_PHY_AC_TABLE_ID: c_uint = 0x00d;
pub const B43_PHY_AC_TABLE_OFFSET: c_uint = 0x00e;
pub const B43_PHY_AC_TABLE_DATA1: c_uint = 0x00f;
pub const B43_PHY_AC_TABLE_DATA2: c_uint = 0x010;
pub const B43_PHY_AC_TABLE_DATA3: c_uint = 0x011;
pub const B43_PHY_AC_CLASSCTL: c_uint = 0x140	/* Classifier control */;
pub const B43_PHY_AC_CLASSCTL_CCKEN: c_uint = 0x0001	/* CCK enable */;
pub const B43_PHY_AC_CLASSCTL_OFDMEN: c_uint = 0x0002	/* OFDM enable */;
pub const B43_PHY_AC_CLASSCTL_WAITEDEN: c_uint = 0x0004	/* Waited enable */;
pub const B43_PHY_AC_BW1A: c_uint = 0x371;
pub const B43_PHY_AC_BW2: c_uint = 0x372;
pub const B43_PHY_AC_BW3: c_uint = 0x373;
pub const B43_PHY_AC_BW4: c_uint = 0x374;
pub const B43_PHY_AC_BW5: c_uint = 0x375;
pub const B43_PHY_AC_BW6: c_uint = 0x376;
pub const B43_PHY_AC_RFCTL_CMD: c_uint = 0x408;
pub const B43_PHY_AC_C1_CLIP: c_uint = 0x6d4;
pub const B43_PHY_AC_C1_CLIP_DIS: c_uint = 0x4000;
pub const B43_PHY_AC_C2_CLIP: c_uint = 0x8d4;
pub const B43_PHY_AC_C2_CLIP_DIS: c_uint = 0x4000;
pub const B43_PHY_AC_C3_CLIP: c_uint = 0xad4;
pub const B43_PHY_AC_C3_CLIP_DIS: c_uint = 0x4000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct b43_phy_ac {
}
