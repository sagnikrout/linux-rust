//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/phy/qualcomm/phy-qcom-qmp-qserdes-ln-shrd-v6.h
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
// Copyright (c) 2023, Linaro Limited
//
pub const QSERDES_V6_LN_SHRD_RXCLK_DIV2_CTRL: c_uint = 0xa0;
pub const QSERDES_V6_LN_SHRD_RX_Q_EN_RATES: c_uint = 0xb0;
pub const QSERDES_V6_LN_SHRD_DFE_DAC_ENABLE1: c_uint = 0xb4;
pub const QSERDES_V6_LN_SHRD_TX_ADAPT_POST_THRESH1: c_uint = 0xc4;
pub const QSERDES_V6_LN_SHRD_TX_ADAPT_POST_THRESH2: c_uint = 0xc8;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B0: c_uint = 0xd4;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B1: c_uint = 0xd8;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B2: c_uint = 0xdc;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B3: c_uint = 0xe0;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B4: c_uint = 0xe4;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B5: c_uint = 0xe8;
pub const QSERDES_V6_LN_SHRD_RX_MODE_RATE_0_1_B6: c_uint = 0xec;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH1_RATE210: c_uint = 0xf0;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH1_RATE3: c_uint = 0xf4;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH2_RATE210: c_uint = 0xf8;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH2_RATE3: c_uint = 0xfc;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH3_RATE210: c_uint = 0x100;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH3_RATE3: c_uint = 0x104;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH4_RATE3: c_uint = 0x10c;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH5_RATE3: c_uint = 0x114;
pub const QSERDES_V6_LN_SHRD_RX_MARG_COARSE_THRESH6_RATE3: c_uint = 0x11c;
pub const QSERDES_V6_LN_SHRD_RX_SUMMER_CAL_SPD_MODE: c_uint = 0x128;
