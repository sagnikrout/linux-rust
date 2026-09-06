//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bcm281xx.h
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
// the clock control units (CCUs) on Broadcom BCM281XX family SoCs.
//
// These are the bcm281xx CCU device tree "compatible" strings.
// We're stuck with using "bcm11351" in the string because wild
// cards aren't allowed, and that name was the first one defined
// in this family of devices.
//

// root CCU clock ids
pub const BCM281XX_ROOT_CCU_FRAC_1M: c_int = 0;
pub const BCM281XX_ROOT_CCU_CLOCK_COUNT: c_int = 1;
// aon CCU clock ids
pub const BCM281XX_AON_CCU_HUB_TIMER: c_int = 0;
pub const BCM281XX_AON_CCU_PMU_BSC: c_int = 1;
pub const BCM281XX_AON_CCU_PMU_BSC_VAR: c_int = 2;
pub const BCM281XX_AON_CCU_CLOCK_COUNT: c_int = 3;
// hub CCU clock ids
pub const BCM281XX_HUB_CCU_TMON_1M: c_int = 0;
pub const BCM281XX_HUB_CCU_CLOCK_COUNT: c_int = 1;
// master CCU clock ids
pub const BCM281XX_MASTER_CCU_SDIO1: c_int = 0;
pub const BCM281XX_MASTER_CCU_SDIO2: c_int = 1;
pub const BCM281XX_MASTER_CCU_SDIO3: c_int = 2;
pub const BCM281XX_MASTER_CCU_SDIO4: c_int = 3;
pub const BCM281XX_MASTER_CCU_USB_IC: c_int = 4;
pub const BCM281XX_MASTER_CCU_HSIC2_48M: c_int = 5;
pub const BCM281XX_MASTER_CCU_HSIC2_12M: c_int = 6;
pub const BCM281XX_MASTER_CCU_CLOCK_COUNT: c_int = 7;
// slave CCU clock ids
pub const BCM281XX_SLAVE_CCU_UARTB: c_int = 0;
pub const BCM281XX_SLAVE_CCU_UARTB2: c_int = 1;
pub const BCM281XX_SLAVE_CCU_UARTB3: c_int = 2;
pub const BCM281XX_SLAVE_CCU_UARTB4: c_int = 3;
pub const BCM281XX_SLAVE_CCU_SSP0: c_int = 4;
pub const BCM281XX_SLAVE_CCU_SSP2: c_int = 5;
pub const BCM281XX_SLAVE_CCU_BSC1: c_int = 6;
pub const BCM281XX_SLAVE_CCU_BSC2: c_int = 7;
pub const BCM281XX_SLAVE_CCU_BSC3: c_int = 8;
pub const BCM281XX_SLAVE_CCU_PWM: c_int = 9;
pub const BCM281XX_SLAVE_CCU_CLOCK_COUNT: c_int = 10;
