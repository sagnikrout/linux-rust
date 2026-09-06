//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interrupt-controller/amlogic,meson-g12a-gpio-intc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR MIT)
//
// Copyright (c) 2023 Amlogic, Inc. All rights reserved.
// Author: Huqiang Qin <huqiang.qin@amlogic.com>
//
// IRQID[11:0] - GPIOAO[11:0]
pub const IRQID_GPIOAO_0: c_int = 0;
pub const IRQID_GPIOAO_1: c_int = 1;
pub const IRQID_GPIOAO_2: c_int = 2;
pub const IRQID_GPIOAO_3: c_int = 3;
pub const IRQID_GPIOAO_4: c_int = 4;
pub const IRQID_GPIOAO_5: c_int = 5;
pub const IRQID_GPIOAO_6: c_int = 6;
pub const IRQID_GPIOAO_7: c_int = 7;
pub const IRQID_GPIOAO_8: c_int = 8;
pub const IRQID_GPIOAO_9: c_int = 9;
pub const IRQID_GPIOAO_10: c_int = 10;
pub const IRQID_GPIOAO_11: c_int = 11;
// IRQID[27:12] - GPIOZ[15:0]
pub const IRQID_GPIOZ_0: c_int = 12;
pub const IRQID_GPIOZ_1: c_int = 13;
pub const IRQID_GPIOZ_2: c_int = 14;
pub const IRQID_GPIOZ_3: c_int = 15;
pub const IRQID_GPIOZ_4: c_int = 16;
pub const IRQID_GPIOZ_5: c_int = 17;
pub const IRQID_GPIOZ_6: c_int = 18;
pub const IRQID_GPIOZ_7: c_int = 19;
pub const IRQID_GPIOZ_8: c_int = 20;
pub const IRQID_GPIOZ_9: c_int = 21;
pub const IRQID_GPIOZ_10: c_int = 22;
pub const IRQID_GPIOZ_11: c_int = 23;
pub const IRQID_GPIOZ_12: c_int = 24;
pub const IRQID_GPIOZ_13: c_int = 25;
pub const IRQID_GPIOZ_14: c_int = 26;
pub const IRQID_GPIOZ_15: c_int = 27;
// IRQID[36:28] - GPIOH[8:0]
pub const IRQID_GPIOH_0: c_int = 28;
pub const IRQID_GPIOH_1: c_int = 29;
pub const IRQID_GPIOH_2: c_int = 30;
pub const IRQID_GPIOH_3: c_int = 31;
pub const IRQID_GPIOH_4: c_int = 32;
pub const IRQID_GPIOH_5: c_int = 33;
pub const IRQID_GPIOH_6: c_int = 34;
pub const IRQID_GPIOH_7: c_int = 35;
pub const IRQID_GPIOH_8: c_int = 36;
// IRQID[52:37] - BOOT[15:0]
pub const IRQID_BOOT_0: c_int = 37;
pub const IRQID_BOOT_1: c_int = 38;
pub const IRQID_BOOT_2: c_int = 39;
pub const IRQID_BOOT_3: c_int = 40;
pub const IRQID_BOOT_4: c_int = 41;
pub const IRQID_BOOT_5: c_int = 42;
pub const IRQID_BOOT_6: c_int = 43;
pub const IRQID_BOOT_7: c_int = 44;
pub const IRQID_BOOT_8: c_int = 45;
pub const IRQID_BOOT_9: c_int = 46;
pub const IRQID_BOOT_10: c_int = 47;
pub const IRQID_BOOT_11: c_int = 48;
pub const IRQID_BOOT_12: c_int = 49;
pub const IRQID_BOOT_13: c_int = 50;
pub const IRQID_BOOT_14: c_int = 51;
pub const IRQID_BOOT_15: c_int = 52;
// IRQID[60:53] - GPIOC[7:0]
pub const IRQID_GPIOC_0: c_int = 53;
pub const IRQID_GPIOC_1: c_int = 54;
pub const IRQID_GPIOC_2: c_int = 55;
pub const IRQID_GPIOC_3: c_int = 56;
pub const IRQID_GPIOC_4: c_int = 57;
pub const IRQID_GPIOC_5: c_int = 58;
pub const IRQID_GPIOC_6: c_int = 59;
pub const IRQID_GPIOC_7: c_int = 60;
// IRQID[76:61] - GPIOA[15:0]
pub const IRQID_GPIOA_0: c_int = 61;
pub const IRQID_GPIOA_1: c_int = 62;
pub const IRQID_GPIOA_2: c_int = 63;
pub const IRQID_GPIOA_3: c_int = 64;
pub const IRQID_GPIOA_4: c_int = 65;
pub const IRQID_GPIOA_5: c_int = 66;
pub const IRQID_GPIOA_6: c_int = 67;
pub const IRQID_GPIOA_7: c_int = 68;
pub const IRQID_GPIOA_8: c_int = 69;
pub const IRQID_GPIOA_9: c_int = 70;
pub const IRQID_GPIOA_10: c_int = 71;
pub const IRQID_GPIOA_11: c_int = 72;
pub const IRQID_GPIOA_12: c_int = 73;
pub const IRQID_GPIOA_13: c_int = 74;
pub const IRQID_GPIOA_14: c_int = 75;
pub const IRQID_GPIOA_15: c_int = 76;
// IRQID[96:77] - GPIOX[19:0]
pub const IRQID_GPIOX_0: c_int = 77;
pub const IRQID_GPIOX_1: c_int = 78;
pub const IRQID_GPIOX_2: c_int = 79;
pub const IRQID_GPIOX_3: c_int = 80;
pub const IRQID_GPIOX_4: c_int = 81;
pub const IRQID_GPIOX_5: c_int = 82;
pub const IRQID_GPIOX_6: c_int = 83;
pub const IRQID_GPIOX_7: c_int = 84;
pub const IRQID_GPIOX_8: c_int = 85;
pub const IRQID_GPIOX_9: c_int = 86;
pub const IRQID_GPIOX_10: c_int = 87;
pub const IRQID_GPIOX_11: c_int = 88;
pub const IRQID_GPIOX_12: c_int = 89;
pub const IRQID_GPIOX_13: c_int = 90;
pub const IRQID_GPIOX_14: c_int = 91;
pub const IRQID_GPIOX_15: c_int = 92;
pub const IRQID_GPIOX_16: c_int = 93;
pub const IRQID_GPIOX_17: c_int = 94;
pub const IRQID_GPIOX_18: c_int = 95;
pub const IRQID_GPIOX_19: c_int = 96;
// IRQID[99:97] - GPIOE[2:0]
pub const IRQID_GPIOE_0: c_int = 97;
pub const IRQID_GPIOE_1: c_int = 98;
pub const IRQID_GPIOE_2: c_int = 99;
