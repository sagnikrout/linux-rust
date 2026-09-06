//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bcm21664.h
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
// Copyright (C) 2013 Broadcom Corporation
// Copyright 2013 Linaro Limited
//
// This file defines the values used to specify clocks provided by
// the clock control units (CCUs) on Broadcom BCM21664 family SoCs.
//
// bcm21664 CCU device tree "compatible" strings

// root CCU clock ids
pub const BCM21664_ROOT_CCU_FRAC_1M: c_int = 0;
pub const BCM21664_ROOT_CCU_CLOCK_COUNT: c_int = 1;
// aon CCU clock ids
pub const BCM21664_AON_CCU_HUB_TIMER: c_int = 0;
pub const BCM21664_AON_CCU_CLOCK_COUNT: c_int = 1;
// master CCU clock ids
pub const BCM21664_MASTER_CCU_SDIO1: c_int = 0;
pub const BCM21664_MASTER_CCU_SDIO2: c_int = 1;
pub const BCM21664_MASTER_CCU_SDIO3: c_int = 2;
pub const BCM21664_MASTER_CCU_SDIO4: c_int = 3;
pub const BCM21664_MASTER_CCU_SDIO1_SLEEP: c_int = 4;
pub const BCM21664_MASTER_CCU_SDIO2_SLEEP: c_int = 5;
pub const BCM21664_MASTER_CCU_SDIO3_SLEEP: c_int = 6;
pub const BCM21664_MASTER_CCU_SDIO4_SLEEP: c_int = 7;
pub const BCM21664_MASTER_CCU_CLOCK_COUNT: c_int = 8;
// slave CCU clock ids
pub const BCM21664_SLAVE_CCU_UARTB: c_int = 0;
pub const BCM21664_SLAVE_CCU_UARTB2: c_int = 1;
pub const BCM21664_SLAVE_CCU_UARTB3: c_int = 2;
pub const BCM21664_SLAVE_CCU_BSC1: c_int = 3;
pub const BCM21664_SLAVE_CCU_BSC2: c_int = 4;
pub const BCM21664_SLAVE_CCU_BSC3: c_int = 5;
pub const BCM21664_SLAVE_CCU_BSC4: c_int = 6;
pub const BCM21664_SLAVE_CCU_CLOCK_COUNT: c_int = 7;
