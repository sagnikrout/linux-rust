//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/amlogic,meson-a1-reset.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (c) 2019 Amlogic, Inc. All rights reserved.
// Author: Xingyu Chen <xingyu.chen@amlogic.com>
//
// RESET0
// 0
pub const RESET_AM2AXI_VAD: c_int = 1;
// 2-3
pub const RESET_PSRAM: c_int = 4;
pub const RESET_PAD_CTRL: c_int = 5;
// 6
pub const RESET_TEMP_SENSOR: c_int = 7;
pub const RESET_AM2AXI_DEV: c_int = 8;
// 9
pub const RESET_SPICC_A: c_int = 10;
pub const RESET_MSR_CLK: c_int = 11;
pub const RESET_AUDIO: c_int = 12;
pub const RESET_ANALOG_CTRL: c_int = 13;
pub const RESET_SAR_ADC: c_int = 14;
pub const RESET_AUDIO_VAD: c_int = 15;
pub const RESET_CEC: c_int = 16;
pub const RESET_PWM_EF: c_int = 17;
pub const RESET_PWM_CD: c_int = 18;
pub const RESET_PWM_AB: c_int = 19;
// 20
pub const RESET_IR_CTRL: c_int = 21;
pub const RESET_I2C_S_A: c_int = 22;
// 23
pub const RESET_I2C_M_D: c_int = 24;
pub const RESET_I2C_M_C: c_int = 25;
pub const RESET_I2C_M_B: c_int = 26;
pub const RESET_I2C_M_A: c_int = 27;
pub const RESET_I2C_PROD_AHB: c_int = 28;
pub const RESET_I2C_PROD: c_int = 29;
// 30-31
// RESET1
pub const RESET_ACODEC: c_int = 32;
pub const RESET_DMA: c_int = 33;
pub const RESET_SD_EMMC_A: c_int = 34;
// 35
pub const RESET_USBCTRL: c_int = 36;
// 37
pub const RESET_USBPHY: c_int = 38;
// 39-41
pub const RESET_RSA: c_int = 42;
pub const RESET_DMC: c_int = 43;
// 44
pub const RESET_IRQ_CTRL: c_int = 45;
// 46
pub const RESET_NIC_VAD: c_int = 47;
pub const RESET_NIC_AXI: c_int = 48;
pub const RESET_RAMA: c_int = 49;
pub const RESET_RAMB: c_int = 50;
// 51-52
pub const RESET_ROM: c_int = 53;
pub const RESET_SPIFC: c_int = 54;
pub const RESET_GIC: c_int = 55;
pub const RESET_UART_C: c_int = 56;
pub const RESET_UART_B: c_int = 57;
pub const RESET_UART_A: c_int = 58;
pub const RESET_OSC_RING: c_int = 59;
// 60-63
// RESET2
// 64-95
