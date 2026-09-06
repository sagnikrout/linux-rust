//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/drxd_firm.h
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
// drxd_firm.h
//
// Copyright (C) 2006-2007 Micronas
//

pub const VERSION_MAJOR: c_int = 1;
pub const VERSION_MINOR: c_int = 4;
pub const VERSION_PATCH: c_int = 23;

pub const HI_I2C_DELAY: c_int = 84;
pub const HI_I2C_BRIDGE_DELAY: c_int = 750;
pub const EQ_TD_TPS_PWR_UNKNOWN: c_uint = 0x00C0	/* Unknown configurations */;
pub const EQ_TD_TPS_PWR_QPSK: c_uint = 0x016a;
pub const EQ_TD_TPS_PWR_QAM16_ALPHAN: c_uint = 0x0195;
pub const EQ_TD_TPS_PWR_QAM16_ALPHA1: c_uint = 0x0195;
pub const EQ_TD_TPS_PWR_QAM16_ALPHA2: c_uint = 0x011E;
pub const EQ_TD_TPS_PWR_QAM16_ALPHA4: c_uint = 0x01CE;
pub const EQ_TD_TPS_PWR_QAM64_ALPHAN: c_uint = 0x019F;
pub const EQ_TD_TPS_PWR_QAM64_ALPHA1: c_uint = 0x019F;
pub const EQ_TD_TPS_PWR_QAM64_ALPHA2: c_uint = 0x00F8;
pub const EQ_TD_TPS_PWR_QAM64_ALPHA4: c_uint = 0x014D;
pub const DRXD_DEF_AG_PWD_CONSUMER: c_uint = 0x000E;
pub const DRXD_DEF_AG_PWD_PRO: c_uint = 0x0000;
pub const DRXD_DEF_AG_AGC_SIO: c_uint = 0x0000;
pub const DRXD_FE_CTRL_MAX: c_int = 1023;

