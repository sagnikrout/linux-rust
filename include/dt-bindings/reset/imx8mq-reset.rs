//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/imx8mq-reset.h
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
// Copyright (C) 2018 Zodiac Inflight Innovations
//
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//
pub const IMX8MQ_RESET_A53_CORE_POR_RESET0: c_int = 0;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET1: c_int = 1;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET2: c_int = 2;
pub const IMX8MQ_RESET_A53_CORE_POR_RESET3: c_int = 3;
pub const IMX8MQ_RESET_A53_CORE_RESET0: c_int = 4;
pub const IMX8MQ_RESET_A53_CORE_RESET1: c_int = 5;
pub const IMX8MQ_RESET_A53_CORE_RESET2: c_int = 6;
pub const IMX8MQ_RESET_A53_CORE_RESET3: c_int = 7;
pub const IMX8MQ_RESET_A53_DBG_RESET0: c_int = 8;
pub const IMX8MQ_RESET_A53_DBG_RESET1: c_int = 9;
pub const IMX8MQ_RESET_A53_DBG_RESET2: c_int = 10;
pub const IMX8MQ_RESET_A53_DBG_RESET3: c_int = 11;
pub const IMX8MQ_RESET_A53_ETM_RESET0: c_int = 12;
pub const IMX8MQ_RESET_A53_ETM_RESET1: c_int = 13;
pub const IMX8MQ_RESET_A53_ETM_RESET2: c_int = 14;
pub const IMX8MQ_RESET_A53_ETM_RESET3: c_int = 15;
pub const IMX8MQ_RESET_A53_SOC_DBG_RESET: c_int = 16;
pub const IMX8MQ_RESET_A53_L2RESET: c_int = 17;
pub const IMX8MQ_RESET_SW_NON_SCLR_M4C_RST: c_int = 18;
pub const IMX8MQ_RESET_OTG1_PHY_RESET: c_int = 19;

pub const IMX8MQ_RESET_DISP_RESET: c_int = 31;
pub const IMX8MQ_RESET_GPU_RESET: c_int = 32;

pub const IMX8MQ_RESET_SW_M4C_RST: c_int = 50;
pub const IMX8MQ_RESET_SW_M4P_RST: c_int = 51;
pub const IMX8MQ_RESET_M4_ENABLE: c_int = 52;
pub const IMX8MQ_RESET_NUM: c_int = 53;
