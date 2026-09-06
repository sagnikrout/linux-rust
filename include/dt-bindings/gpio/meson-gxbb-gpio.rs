//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/meson-gxbb-gpio.h
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
// GPIO definitions for Amlogic Meson GXBB SoCs
//
// Copyright (C) 2016 Endless Mobile, Inc.
// Author: Carlo Caione <carlo@endlessm.com>
//
pub const GPIOAO_0: c_int = 0;
pub const GPIOAO_1: c_int = 1;
pub const GPIOAO_2: c_int = 2;
pub const GPIOAO_3: c_int = 3;
pub const GPIOAO_4: c_int = 4;
pub const GPIOAO_5: c_int = 5;
pub const GPIOAO_6: c_int = 6;
pub const GPIOAO_7: c_int = 7;
pub const GPIOAO_8: c_int = 8;
pub const GPIOAO_9: c_int = 9;
pub const GPIOAO_10: c_int = 10;
pub const GPIOAO_11: c_int = 11;
pub const GPIOAO_12: c_int = 12;
pub const GPIOAO_13: c_int = 13;
pub const GPIO_TEST_N: c_int = 14;
pub const GPIOZ_0: c_int = 0;
pub const GPIOZ_1: c_int = 1;
pub const GPIOZ_2: c_int = 2;
pub const GPIOZ_3: c_int = 3;
pub const GPIOZ_4: c_int = 4;
pub const GPIOZ_5: c_int = 5;
pub const GPIOZ_6: c_int = 6;
pub const GPIOZ_7: c_int = 7;
pub const GPIOZ_8: c_int = 8;
pub const GPIOZ_9: c_int = 9;
pub const GPIOZ_10: c_int = 10;
pub const GPIOZ_11: c_int = 11;
pub const GPIOZ_12: c_int = 12;
pub const GPIOZ_13: c_int = 13;
pub const GPIOZ_14: c_int = 14;
pub const GPIOZ_15: c_int = 15;
pub const GPIOH_0: c_int = 16;
pub const GPIOH_1: c_int = 17;
pub const GPIOH_2: c_int = 18;
pub const GPIOH_3: c_int = 19;
pub const BOOT_0: c_int = 20;
pub const BOOT_1: c_int = 21;
pub const BOOT_2: c_int = 22;
pub const BOOT_3: c_int = 23;
pub const BOOT_4: c_int = 24;
pub const BOOT_5: c_int = 25;
pub const BOOT_6: c_int = 26;
pub const BOOT_7: c_int = 27;
pub const BOOT_8: c_int = 28;
pub const BOOT_9: c_int = 29;
pub const BOOT_10: c_int = 30;
pub const BOOT_11: c_int = 31;
pub const BOOT_12: c_int = 32;
pub const BOOT_13: c_int = 33;
pub const BOOT_14: c_int = 34;
pub const BOOT_15: c_int = 35;
pub const BOOT_16: c_int = 36;
pub const BOOT_17: c_int = 37;
pub const CARD_0: c_int = 38;
pub const CARD_1: c_int = 39;
pub const CARD_2: c_int = 40;
pub const CARD_3: c_int = 41;
pub const CARD_4: c_int = 42;
pub const CARD_5: c_int = 43;
pub const CARD_6: c_int = 44;
pub const GPIODV_0: c_int = 45;
pub const GPIODV_1: c_int = 46;
pub const GPIODV_2: c_int = 47;
pub const GPIODV_3: c_int = 48;
pub const GPIODV_4: c_int = 49;
pub const GPIODV_5: c_int = 50;
pub const GPIODV_6: c_int = 51;
pub const GPIODV_7: c_int = 52;
pub const GPIODV_8: c_int = 53;
pub const GPIODV_9: c_int = 54;
pub const GPIODV_10: c_int = 55;
pub const GPIODV_11: c_int = 56;
pub const GPIODV_12: c_int = 57;
pub const GPIODV_13: c_int = 58;
pub const GPIODV_14: c_int = 59;
pub const GPIODV_15: c_int = 60;
pub const GPIODV_16: c_int = 61;
pub const GPIODV_17: c_int = 62;
pub const GPIODV_18: c_int = 63;
pub const GPIODV_19: c_int = 64;
pub const GPIODV_20: c_int = 65;
pub const GPIODV_21: c_int = 66;
pub const GPIODV_22: c_int = 67;
pub const GPIODV_23: c_int = 68;
pub const GPIODV_24: c_int = 69;
pub const GPIODV_25: c_int = 70;
pub const GPIODV_26: c_int = 71;
pub const GPIODV_27: c_int = 72;
pub const GPIODV_28: c_int = 73;
pub const GPIODV_29: c_int = 74;
pub const GPIOY_0: c_int = 75;
pub const GPIOY_1: c_int = 76;
pub const GPIOY_2: c_int = 77;
pub const GPIOY_3: c_int = 78;
pub const GPIOY_4: c_int = 79;
pub const GPIOY_5: c_int = 80;
pub const GPIOY_6: c_int = 81;
pub const GPIOY_7: c_int = 82;
pub const GPIOY_8: c_int = 83;
pub const GPIOY_9: c_int = 84;
pub const GPIOY_10: c_int = 85;
pub const GPIOY_11: c_int = 86;
pub const GPIOY_12: c_int = 87;
pub const GPIOY_13: c_int = 88;
pub const GPIOY_14: c_int = 89;
pub const GPIOY_15: c_int = 90;
pub const GPIOY_16: c_int = 91;
pub const GPIOX_0: c_int = 92;
pub const GPIOX_1: c_int = 93;
pub const GPIOX_2: c_int = 94;
pub const GPIOX_3: c_int = 95;
pub const GPIOX_4: c_int = 96;
pub const GPIOX_5: c_int = 97;
pub const GPIOX_6: c_int = 98;
pub const GPIOX_7: c_int = 99;
pub const GPIOX_8: c_int = 100;
pub const GPIOX_9: c_int = 101;
pub const GPIOX_10: c_int = 102;
pub const GPIOX_11: c_int = 103;
pub const GPIOX_12: c_int = 104;
pub const GPIOX_13: c_int = 105;
pub const GPIOX_14: c_int = 106;
pub const GPIOX_15: c_int = 107;
pub const GPIOX_16: c_int = 108;
pub const GPIOX_17: c_int = 109;
pub const GPIOX_18: c_int = 110;
pub const GPIOX_19: c_int = 111;
pub const GPIOX_20: c_int = 112;
pub const GPIOX_21: c_int = 113;
pub const GPIOX_22: c_int = 114;
pub const GPIOCLK_0: c_int = 115;
pub const GPIOCLK_1: c_int = 116;
pub const GPIOCLK_2: c_int = 117;
pub const GPIOCLK_3: c_int = 118;
