//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/amlogic/amlogic-a5-reset.h
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
// Copyright (c) 2024 Amlogic, Inc. All rights reserved.
//
// RESET0
// 0-3
pub const RESET_USB: c_int = 4;
// 5-7
pub const RESET_USBPHY20: c_int = 8;
// 9
pub const RESET_USB2DRD: c_int = 10;
// 11-31
// RESET1
pub const RESET_AUDIO: c_int = 32;
pub const RESET_AUDIO_VAD: c_int = 33;
// 34
pub const RESET_DDR_APB: c_int = 35;
pub const RESET_DDR: c_int = 36;
// 37-40
pub const RESET_DSPA_DEBUG: c_int = 41;
// 42
pub const RESET_DSPA: c_int = 43;
// 44-46
pub const RESET_NNA: c_int = 47;
pub const RESET_ETHERNET: c_int = 48;
// 49-63
// RESET2
pub const RESET_ABUS_ARB: c_int = 64;
pub const RESET_IRCTRL: c_int = 65;
// 66
pub const RESET_TS_PLL: c_int = 67;
// 68-72
pub const RESET_SPICC_0: c_int = 73;
pub const RESET_SPICC_1: c_int = 74;
pub const RESET_RSA: c_int = 75;
// 76-79
pub const RESET_MSR_CLK: c_int = 80;
pub const RESET_SPIFC: c_int = 81;
pub const RESET_SAR_ADC: c_int = 82;
// 83-90
pub const RESET_WATCHDOG: c_int = 91;
// 92-95
// RESET3
// 96-127
// RESET4
pub const RESET_RTC: c_int = 128;
// 129-131
pub const RESET_PWM_AB: c_int = 132;
pub const RESET_PWM_CD: c_int = 133;
pub const RESET_PWM_EF: c_int = 134;
pub const RESET_PWM_GH: c_int = 135;
// 104-105
pub const RESET_UART_A: c_int = 138;
pub const RESET_UART_B: c_int = 139;
pub const RESET_UART_C: c_int = 140;
pub const RESET_UART_D: c_int = 141;
pub const RESET_UART_E: c_int = 142;
// 143
pub const RESET_I2C_S_A: c_int = 144;
pub const RESET_I2C_M_A: c_int = 145;
pub const RESET_I2C_M_B: c_int = 146;
pub const RESET_I2C_M_C: c_int = 147;
pub const RESET_I2C_M_D: c_int = 148;
// 149-151
pub const RESET_SDEMMC_A: c_int = 152;
// 153
pub const RESET_SDEMMC_C: c_int = 154;
// 155-159
// RESET5
// 160-175
pub const RESET_BRG_AO_NIC_SYS: c_int = 176;
pub const RESET_BRG_AO_NIC_DSPA: c_int = 177;
pub const RESET_BRG_AO_NIC_MAIN: c_int = 178;
pub const RESET_BRG_AO_NIC_AUDIO: c_int = 179;
// 180-183
pub const RESET_BRG_AO_NIC_ALL: c_int = 184;
pub const RESET_BRG_NIC_NNA: c_int = 185;
pub const RESET_BRG_NIC_SDIO: c_int = 186;
pub const RESET_BRG_NIC_EMMC: c_int = 187;
pub const RESET_BRG_NIC_DSU: c_int = 188;
pub const RESET_BRG_NIC_SYSCLK: c_int = 189;
pub const RESET_BRG_NIC_MAIN: c_int = 190;
pub const RESET_BRG_NIC_ALL: c_int = 191;
