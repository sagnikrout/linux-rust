//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/gpio/amlogic,t7-periphs-pinctrl.h
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
pub const GPIOB_0: c_int = 0;
pub const GPIOB_1: c_int = 1;
pub const GPIOB_2: c_int = 2;
pub const GPIOB_3: c_int = 3;
pub const GPIOB_4: c_int = 4;
pub const GPIOB_5: c_int = 5;
pub const GPIOB_6: c_int = 6;
pub const GPIOB_7: c_int = 7;
pub const GPIOB_8: c_int = 8;
pub const GPIOB_9: c_int = 9;
pub const GPIOB_10: c_int = 10;
pub const GPIOB_11: c_int = 11;
pub const GPIOB_12: c_int = 12;
pub const GPIOC_0: c_int = 13;
pub const GPIOC_1: c_int = 14;
pub const GPIOC_2: c_int = 15;
pub const GPIOC_3: c_int = 16;
pub const GPIOC_4: c_int = 17;
pub const GPIOC_5: c_int = 18;
pub const GPIOC_6: c_int = 19;
pub const GPIOX_0: c_int = 20;
pub const GPIOX_1: c_int = 21;
pub const GPIOX_2: c_int = 22;
pub const GPIOX_3: c_int = 23;
pub const GPIOX_4: c_int = 24;
pub const GPIOX_5: c_int = 25;
pub const GPIOX_6: c_int = 26;
pub const GPIOX_7: c_int = 27;
pub const GPIOX_8: c_int = 28;
pub const GPIOX_9: c_int = 29;
pub const GPIOX_10: c_int = 30;
pub const GPIOX_11: c_int = 31;
pub const GPIOX_12: c_int = 32;
pub const GPIOX_13: c_int = 33;
pub const GPIOX_14: c_int = 34;
pub const GPIOX_15: c_int = 35;
pub const GPIOX_16: c_int = 36;
pub const GPIOX_17: c_int = 37;
pub const GPIOX_18: c_int = 38;
pub const GPIOX_19: c_int = 39;
pub const GPIOW_0: c_int = 40;
pub const GPIOW_1: c_int = 41;
pub const GPIOW_2: c_int = 42;
pub const GPIOW_3: c_int = 43;
pub const GPIOW_4: c_int = 44;
pub const GPIOW_5: c_int = 45;
pub const GPIOW_6: c_int = 46;
pub const GPIOW_7: c_int = 47;
pub const GPIOW_8: c_int = 48;
pub const GPIOW_9: c_int = 49;
pub const GPIOW_10: c_int = 50;
pub const GPIOW_11: c_int = 51;
pub const GPIOW_12: c_int = 52;
pub const GPIOW_13: c_int = 53;
pub const GPIOW_14: c_int = 54;
pub const GPIOW_15: c_int = 55;
pub const GPIOW_16: c_int = 56;
pub const GPIOD_0: c_int = 57;
pub const GPIOD_1: c_int = 58;
pub const GPIOD_2: c_int = 59;
pub const GPIOD_3: c_int = 60;
pub const GPIOD_4: c_int = 61;
pub const GPIOD_5: c_int = 62;
pub const GPIOD_6: c_int = 63;
pub const GPIOD_7: c_int = 64;
pub const GPIOD_8: c_int = 65;
pub const GPIOD_9: c_int = 66;
pub const GPIOD_10: c_int = 67;
pub const GPIOD_11: c_int = 68;
pub const GPIOD_12: c_int = 69;
pub const GPIOE_0: c_int = 70;
pub const GPIOE_1: c_int = 71;
pub const GPIOE_2: c_int = 72;
pub const GPIOE_3: c_int = 73;
pub const GPIOE_4: c_int = 74;
pub const GPIOE_5: c_int = 75;
pub const GPIOE_6: c_int = 76;
pub const GPIOZ_0: c_int = 77;
pub const GPIOZ_1: c_int = 78;
pub const GPIOZ_2: c_int = 79;
pub const GPIOZ_3: c_int = 80;
pub const GPIOZ_4: c_int = 81;
pub const GPIOZ_5: c_int = 82;
pub const GPIOZ_6: c_int = 83;
pub const GPIOZ_7: c_int = 84;
pub const GPIOZ_8: c_int = 85;
pub const GPIOZ_9: c_int = 86;
pub const GPIOZ_10: c_int = 87;
pub const GPIOZ_11: c_int = 88;
pub const GPIOZ_12: c_int = 89;
pub const GPIOZ_13: c_int = 90;
pub const GPIOT_0: c_int = 91;
pub const GPIOT_1: c_int = 92;
pub const GPIOT_2: c_int = 93;
pub const GPIOT_3: c_int = 94;
pub const GPIOT_4: c_int = 95;
pub const GPIOT_5: c_int = 96;
pub const GPIOT_6: c_int = 97;
pub const GPIOT_7: c_int = 98;
pub const GPIOT_8: c_int = 99;
pub const GPIOT_9: c_int = 100;
pub const GPIOT_10: c_int = 101;
pub const GPIOT_11: c_int = 102;
pub const GPIOT_12: c_int = 103;
pub const GPIOT_13: c_int = 104;
pub const GPIOT_14: c_int = 105;
pub const GPIOT_15: c_int = 106;
pub const GPIOT_16: c_int = 107;
pub const GPIOT_17: c_int = 108;
pub const GPIOT_18: c_int = 109;
pub const GPIOT_19: c_int = 110;
pub const GPIOT_20: c_int = 111;
pub const GPIOT_21: c_int = 112;
pub const GPIOT_22: c_int = 113;
pub const GPIOT_23: c_int = 114;
pub const GPIOM_0: c_int = 115;
pub const GPIOM_1: c_int = 116;
pub const GPIOM_2: c_int = 117;
pub const GPIOM_3: c_int = 118;
pub const GPIOM_4: c_int = 119;
pub const GPIOM_5: c_int = 120;
pub const GPIOM_6: c_int = 121;
pub const GPIOM_7: c_int = 122;
pub const GPIOM_8: c_int = 123;
pub const GPIOM_9: c_int = 124;
pub const GPIOM_10: c_int = 125;
pub const GPIOM_11: c_int = 126;
pub const GPIOM_12: c_int = 127;
pub const GPIOM_13: c_int = 128;
pub const GPIOY_0: c_int = 129;
pub const GPIOY_1: c_int = 130;
pub const GPIOY_2: c_int = 131;
pub const GPIOY_3: c_int = 132;
pub const GPIOY_4: c_int = 133;
pub const GPIOY_5: c_int = 134;
pub const GPIOY_6: c_int = 135;
pub const GPIOY_7: c_int = 136;
pub const GPIOY_8: c_int = 137;
pub const GPIOY_9: c_int = 138;
pub const GPIOY_10: c_int = 139;
pub const GPIOY_11: c_int = 140;
pub const GPIOY_12: c_int = 141;
pub const GPIOY_13: c_int = 142;
pub const GPIOY_14: c_int = 143;
pub const GPIOY_15: c_int = 144;
pub const GPIOY_16: c_int = 145;
pub const GPIOY_17: c_int = 146;
pub const GPIOY_18: c_int = 147;
pub const GPIOH_0: c_int = 148;
pub const GPIOH_1: c_int = 149;
pub const GPIOH_2: c_int = 150;
pub const GPIOH_3: c_int = 151;
pub const GPIOH_4: c_int = 152;
pub const GPIOH_5: c_int = 153;
pub const GPIOH_6: c_int = 154;
pub const GPIOH_7: c_int = 155;
pub const GPIO_TEST_N: c_int = 156;
