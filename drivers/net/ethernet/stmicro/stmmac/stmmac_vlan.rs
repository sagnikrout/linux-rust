//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/stmmac_vlan.h
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
//
// Copyright (C) 2025, Altera Corporation
// stmmac VLAN(802.1Q) handling
//

pub const VLAN_TAG: c_uint = 0x00000050;
pub const VLAN_TAG_DATA: c_uint = 0x00000054;
pub const VLAN_HASH_TABLE: c_uint = 0x00000058;
pub const VLAN_INCL: c_uint = 0x00000060;
// MAC VLAN

pub const VLAN_VLC_SHIFT: c_int = 16;

// MAC VLAN Tag

// MAC VLAN Tag Control

pub const VLAN_TAG_CTRL_OFS_SHIFT: c_int = 2;

pub const VLAN_TAG_CTRL_EVLS_SHIFT: c_int = 21;

// MAC VLAN Tag Data/Filter

// MAC VLAN HW FEAT
pub const HW_FEATURE3: c_uint = 0x00000128;

extern "C" {
    pub fn stmmac_get_num_vlan(ioaddr: *mut void __iomem) -> u32;
}
