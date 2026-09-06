//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/devfreq/event/exynos-nocp.h
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
// exynos-nocp.h - Exynos NoC (Network on Chip) Probe header file
//
// Copyright (c) 2016 Samsung Electronics Co., Ltd.
// Author : Chanwoo Choi <cw00.choi@samsung.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nocp_reg {
    NOCP_ID_REVISION_ID		= 0x04,
    NOCP_MAIN_CTL			= 0x08,
    NOCP_CFG_CTL			= 0x0C,

    NOCP_STAT_PERIOD		= 0x24,
    NOCP_STAT_GO			= 0x28,
    NOCP_STAT_ALARM_MIN		= 0x2C,
    NOCP_STAT_ALARM_MAX		= 0x30,
    NOCP_STAT_ALARM_STATUS		= 0x34,
    NOCP_STAT_ALARM_CLR		= 0x38,

    NOCP_COUNTERS_0_SRC		= 0x138,
    NOCP_COUNTERS_0_ALARM_MODE	= 0x13C,
    NOCP_COUNTERS_0_VAL		= 0x140,

    NOCP_COUNTERS_1_SRC		= 0x14C,
    NOCP_COUNTERS_1_ALARM_MODE	= 0x150,
    NOCP_COUNTERS_1_VAL		= 0x154,

    NOCP_COUNTERS_2_SRC		= 0x160,
    NOCP_COUNTERS_2_ALARM_MODE	= 0x164,
    NOCP_COUNTERS_2_VAL		= 0x168,

    NOCP_COUNTERS_3_SRC		= 0x174,
    NOCP_COUNTERS_3_ALARM_MODE	= 0x178,
    NOCP_COUNTERS_3_VAL		= 0x17C,
}

// NOCP_MAIN_CTL register

// NOCP_CFG_CTL register

// NOCP_COUNTERS_x_SRC register
pub const NOCP_CNT_SRC_INTEVENT_SHIFT: c_int = 0;

// NOCP_COUNTERS_x_ALARM_MODE register
pub const NOCP_CNT_ALARM_MODE_SHIFT: c_int = 0;

