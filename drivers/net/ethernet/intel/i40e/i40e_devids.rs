//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e_devids.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.
// Device IDs
pub const I40E_DEV_ID_X710_N3000: c_uint = 0x0CF8;
pub const I40E_DEV_ID_XXV710_N3000: c_uint = 0x0D58;
pub const I40E_DEV_ID_SFP_XL710: c_uint = 0x1572;
pub const I40E_DEV_ID_QEMU: c_uint = 0x1574;
pub const I40E_DEV_ID_KX_B: c_uint = 0x1580;
pub const I40E_DEV_ID_KX_C: c_uint = 0x1581;
pub const I40E_DEV_ID_QSFP_A: c_uint = 0x1583;
pub const I40E_DEV_ID_QSFP_B: c_uint = 0x1584;
pub const I40E_DEV_ID_QSFP_C: c_uint = 0x1585;
pub const I40E_DEV_ID_10G_BASE_T: c_uint = 0x1586;
pub const I40E_DEV_ID_20G_KR2: c_uint = 0x1587;
pub const I40E_DEV_ID_20G_KR2_A: c_uint = 0x1588;
pub const I40E_DEV_ID_10G_BASE_T4: c_uint = 0x1589;
pub const I40E_DEV_ID_25G_B: c_uint = 0x158A;
pub const I40E_DEV_ID_25G_SFP28: c_uint = 0x158B;
pub const I40E_DEV_ID_10G_BASE_T_BC: c_uint = 0x15FF;
pub const I40E_DEV_ID_10G_B: c_uint = 0x104F;
pub const I40E_DEV_ID_10G_SFP: c_uint = 0x104E;
pub const I40E_DEV_ID_5G_BASE_T_BC: c_uint = 0x101F;
pub const I40E_DEV_ID_1G_BASE_T_BC: c_uint = 0x0DD2;

pub const I40E_DEV_ID_KX_X722: c_uint = 0x37CE;
pub const I40E_DEV_ID_QSFP_X722: c_uint = 0x37CF;
pub const I40E_DEV_ID_SFP_X722: c_uint = 0x37D0;
pub const I40E_DEV_ID_1G_BASE_T_X722: c_uint = 0x37D1;
pub const I40E_DEV_ID_10G_BASE_T_X722: c_uint = 0x37D2;
pub const I40E_DEV_ID_SFP_I_X722: c_uint = 0x37D3;
pub const I40E_DEV_ID_SFP_X722_A: c_uint = 0x0DDA;
