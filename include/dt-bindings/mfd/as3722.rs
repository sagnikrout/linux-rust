//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mfd/as3722.h
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
// This header provides macros for ams AS3722 device bindings.
//
// Copyright (c) 2013, NVIDIA Corporation.
//
// Author: Laxman Dewangan <ldewangan@nvidia.com>
//
// External control pins
pub const AS3722_EXT_CONTROL_PIN_ENABLE1: c_int = 1;
pub const AS3722_EXT_CONTROL_PIN_ENABLE2: c_int = 2;
pub const AS3722_EXT_CONTROL_PIN_ENABLE3: c_int = 3;
// Interrupt numbers for AS3722
pub const AS3722_IRQ_LID: c_int = 0;
pub const AS3722_IRQ_ACOK: c_int = 1;
pub const AS3722_IRQ_ENABLE1: c_int = 2;
pub const AS3722_IRQ_OCCUR_ALARM_SD0: c_int = 3;
pub const AS3722_IRQ_ONKEY_LONG_PRESS: c_int = 4;
pub const AS3722_IRQ_ONKEY: c_int = 5;
pub const AS3722_IRQ_OVTMP: c_int = 6;
pub const AS3722_IRQ_LOWBAT: c_int = 7;
pub const AS3722_IRQ_SD0_LV: c_int = 8;
pub const AS3722_IRQ_SD1_LV: c_int = 9;
pub const AS3722_IRQ_SD2_LV: c_int = 10;
pub const AS3722_IRQ_PWM1_OV_PROT: c_int = 11;
pub const AS3722_IRQ_PWM2_OV_PROT: c_int = 12;
pub const AS3722_IRQ_ENABLE2: c_int = 13;
pub const AS3722_IRQ_SD6_LV: c_int = 14;
pub const AS3722_IRQ_RTC_REP: c_int = 15;
pub const AS3722_IRQ_RTC_ALARM: c_int = 16;
pub const AS3722_IRQ_GPIO1: c_int = 17;
pub const AS3722_IRQ_GPIO2: c_int = 18;
pub const AS3722_IRQ_GPIO3: c_int = 19;
pub const AS3722_IRQ_GPIO4: c_int = 20;
pub const AS3722_IRQ_GPIO5: c_int = 21;
pub const AS3722_IRQ_WATCHDOG: c_int = 22;
pub const AS3722_IRQ_ENABLE3: c_int = 23;
pub const AS3722_IRQ_TEMP_SD0_SHUTDOWN: c_int = 24;
pub const AS3722_IRQ_TEMP_SD1_SHUTDOWN: c_int = 25;
pub const AS3722_IRQ_TEMP_SD2_SHUTDOWN: c_int = 26;
pub const AS3722_IRQ_TEMP_SD0_ALARM: c_int = 27;
pub const AS3722_IRQ_TEMP_SD1_ALARM: c_int = 28;
pub const AS3722_IRQ_TEMP_SD6_ALARM: c_int = 29;
pub const AS3722_IRQ_OCCUR_ALARM_SD6: c_int = 30;
pub const AS3722_IRQ_ADC: c_int = 31;
