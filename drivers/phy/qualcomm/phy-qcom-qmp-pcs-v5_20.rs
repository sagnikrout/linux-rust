//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-pcs-v5_20.h
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
//
// Copyright (c) 2022, Linaro Ltd.
//
pub const QPHY_V5_20_PCS_INSIG_SW_CTRL7: c_uint = 0x060;
pub const QPHY_V5_20_PCS_INSIG_MX_CTRL7: c_uint = 0x07c;
pub const QPHY_V5_20_PCS_LOCK_DETECT_CONFIG1: c_uint = 0x0c4;
pub const QPHY_V5_20_PCS_LOCK_DETECT_CONFIG2: c_uint = 0x0c8;
pub const QPHY_V5_20_PCS_G3S2_PRE_GAIN: c_uint = 0x170;
pub const QPHY_V5_20_PCS_RX_SIGDET_LVL: c_uint = 0x188;
pub const QPHY_V5_20_PCS_ALIGN_DETECT_CONFIG1: c_uint = 0x1b8;
pub const QPHY_V5_20_PCS_ALIGN_DETECT_CONFIG2: c_uint = 0x1bc;
pub const QPHY_V5_20_PCS_EQ_CONFIG2: c_uint = 0x1d8;
pub const QPHY_V5_20_PCS_EQ_CONFIG4: c_uint = 0x1e0;
pub const QPHY_V5_20_PCS_EQ_CONFIG5: c_uint = 0x1e4;
