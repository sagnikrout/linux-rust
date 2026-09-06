//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/amlogic/amlogic-a4-reset.h
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
// 5-6
pub const RESET_U2PHY22: c_int = 7;
pub const RESET_USBPHY20: c_int = 8;
pub const RESET_U2PHY21: c_int = 9;
pub const RESET_USB2DRD: c_int = 10;
pub const RESET_U2H: c_int = 11;
pub const RESET_LED_CTRL: c_int = 12;
// 13-31
// RESET1
pub const RESET_AUDIO: c_int = 32;
pub const RESET_AUDIO_VAD: c_int = 33;
// 34
pub const RESET_DDR_APB: c_int = 35;
pub const RESET_DDR: c_int = 36;
pub const RESET_VOUT_VENC: c_int = 37;
pub const RESET_VOUT: c_int = 38;
// 39-47
pub const RESET_ETHERNET: c_int = 48;
// 49-63
// RESET2
pub const RESET_DEVICE_MMC_ARB: c_int = 64;
pub const RESET_IRCTRL: c_int = 65;
// 66
pub const RESET_TS_PLL: c_int = 67;
// 68-72
pub const RESET_SPICC_0: c_int = 73;
pub const RESET_SPICC_1: c_int = 74;
// 75-79
pub const RESET_MSR_CLK: c_int = 80;
// 81
pub const RESET_SAR_ADC: c_int = 82;
// 83-87
pub const RESET_ACODEC: c_int = 88;
// 89-90
pub const RESET_WATCHDOG: c_int = 91;
// 92-95
// RESET3
// 96-127
// RESET4
// 128-131
pub const RESET_PWM_AB: c_int = 132;
pub const RESET_PWM_CD: c_int = 133;
pub const RESET_PWM_EF: c_int = 134;
pub const RESET_PWM_GH: c_int = 135;
// 136-137
pub const RESET_UART_A: c_int = 138;
pub const RESET_UART_B: c_int = 139;
// 140
pub const RESET_UART_D: c_int = 141;
pub const RESET_UART_E: c_int = 142;
// 143-144
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
// 177
pub const RESET_BRG_AO_NIC_MAIN: c_int = 178;
pub const RESET_BRG_AO_NIC_AUDIO: c_int = 179;
// 180-183
pub const RESET_BRG_AO_NIC_ALL: c_int = 184;
// 185
pub const RESET_BRG_NIC_SDIO: c_int = 186;
pub const RESET_BRG_NIC_EMMC: c_int = 187;
pub const RESET_BRG_NIC_DSU: c_int = 188;
pub const RESET_BRG_NIC_CLK81: c_int = 189;
pub const RESET_BRG_NIC_MAIN: c_int = 190;
pub const RESET_BRG_NIC_ALL: c_int = 191;
