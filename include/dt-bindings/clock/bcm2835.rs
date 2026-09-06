//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/bcm2835.h
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
// Copyright (C) 2015 Broadcom Corporation
//
pub const BCM2835_PLLA: c_int = 0;
pub const BCM2835_PLLB: c_int = 1;
pub const BCM2835_PLLC: c_int = 2;
pub const BCM2835_PLLD: c_int = 3;
pub const BCM2835_PLLH: c_int = 4;
pub const BCM2835_PLLA_CORE: c_int = 5;
pub const BCM2835_PLLA_PER: c_int = 6;
pub const BCM2835_PLLB_ARM: c_int = 7;
pub const BCM2835_PLLC_CORE0: c_int = 8;
pub const BCM2835_PLLC_CORE1: c_int = 9;
pub const BCM2835_PLLC_CORE2: c_int = 10;
pub const BCM2835_PLLC_PER: c_int = 11;
pub const BCM2835_PLLD_CORE: c_int = 12;
pub const BCM2835_PLLD_PER: c_int = 13;
pub const BCM2835_PLLH_RCAL: c_int = 14;
pub const BCM2835_PLLH_AUX: c_int = 15;
pub const BCM2835_PLLH_PIX: c_int = 16;
pub const BCM2835_CLOCK_TIMER: c_int = 17;
pub const BCM2835_CLOCK_OTP: c_int = 18;
pub const BCM2835_CLOCK_UART: c_int = 19;
pub const BCM2835_CLOCK_VPU: c_int = 20;
pub const BCM2835_CLOCK_V3D: c_int = 21;
pub const BCM2835_CLOCK_ISP: c_int = 22;
pub const BCM2835_CLOCK_H264: c_int = 23;
pub const BCM2835_CLOCK_VEC: c_int = 24;
pub const BCM2835_CLOCK_HSM: c_int = 25;
pub const BCM2835_CLOCK_SDRAM: c_int = 26;
pub const BCM2835_CLOCK_TSENS: c_int = 27;
pub const BCM2835_CLOCK_EMMC: c_int = 28;
pub const BCM2835_CLOCK_PERI_IMAGE: c_int = 29;
pub const BCM2835_CLOCK_PWM: c_int = 30;
pub const BCM2835_CLOCK_PCM: c_int = 31;
pub const BCM2835_PLLA_DSI0: c_int = 32;
pub const BCM2835_PLLA_CCP2: c_int = 33;
pub const BCM2835_PLLD_DSI0: c_int = 34;
pub const BCM2835_PLLD_DSI1: c_int = 35;
pub const BCM2835_CLOCK_AVEO: c_int = 36;
pub const BCM2835_CLOCK_DFT: c_int = 37;
pub const BCM2835_CLOCK_GP0: c_int = 38;
pub const BCM2835_CLOCK_GP1: c_int = 39;
pub const BCM2835_CLOCK_GP2: c_int = 40;
pub const BCM2835_CLOCK_SLIM: c_int = 41;
pub const BCM2835_CLOCK_SMI: c_int = 42;
pub const BCM2835_CLOCK_TEC: c_int = 43;
pub const BCM2835_CLOCK_DPI: c_int = 44;
pub const BCM2835_CLOCK_CAM0: c_int = 45;
pub const BCM2835_CLOCK_CAM1: c_int = 46;
pub const BCM2835_CLOCK_DSI0E: c_int = 47;
pub const BCM2835_CLOCK_DSI1E: c_int = 48;
pub const BCM2835_CLOCK_DSI0P: c_int = 49;
pub const BCM2835_CLOCK_DSI1P: c_int = 50;
pub const BCM2711_CLOCK_EMMC2: c_int = 51;
