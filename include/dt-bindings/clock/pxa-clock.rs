//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/pxa-clock.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Inspired by original work from pxa2xx-regs.h by Nicolas Pitre
// Copyright (C) 2014 Robert Jarzmik
//
pub const CLK_NONE: c_int = 0;
pub const CLK_1WIRE: c_int = 1;
pub const CLK_AC97: c_int = 2;
pub const CLK_AC97CONF: c_int = 3;
pub const CLK_ASSP: c_int = 4;
pub const CLK_BOOT: c_int = 5;
pub const CLK_BTUART: c_int = 6;
pub const CLK_CAMERA: c_int = 7;
pub const CLK_CIR: c_int = 8;
pub const CLK_CORE: c_int = 9;
pub const CLK_DMC: c_int = 10;
pub const CLK_FFUART: c_int = 11;
pub const CLK_FICP: c_int = 12;
pub const CLK_GPIO: c_int = 13;
pub const CLK_HSIO2: c_int = 14;
pub const CLK_HWUART: c_int = 15;
pub const CLK_I2C: c_int = 16;
pub const CLK_I2S: c_int = 17;
pub const CLK_IM: c_int = 18;
pub const CLK_INC: c_int = 19;
pub const CLK_ISC: c_int = 20;
pub const CLK_KEYPAD: c_int = 21;
pub const CLK_LCD: c_int = 22;
pub const CLK_MEMC: c_int = 23;
pub const CLK_MEMSTK: c_int = 24;
pub const CLK_MINI_IM: c_int = 25;
pub const CLK_MINI_LCD: c_int = 26;
pub const CLK_MMC: c_int = 27;
pub const CLK_MMC1: c_int = 28;
pub const CLK_MMC2: c_int = 29;
pub const CLK_MMC3: c_int = 30;
pub const CLK_MSL: c_int = 31;
pub const CLK_MSL0: c_int = 32;
pub const CLK_MVED: c_int = 33;
pub const CLK_NAND: c_int = 34;
pub const CLK_NSSP: c_int = 35;
pub const CLK_OSTIMER: c_int = 36;
pub const CLK_PWM0: c_int = 37;
pub const CLK_PWM1: c_int = 38;
pub const CLK_PWM2: c_int = 39;
pub const CLK_PWM3: c_int = 40;
pub const CLK_PWRI2C: c_int = 41;
pub const CLK_PXA300_GCU: c_int = 42;
pub const CLK_PXA320_GCU: c_int = 43;
pub const CLK_SMC: c_int = 44;
pub const CLK_SSP: c_int = 45;
pub const CLK_SSP1: c_int = 46;
pub const CLK_SSP2: c_int = 47;
pub const CLK_SSP3: c_int = 48;
pub const CLK_SSP4: c_int = 49;
pub const CLK_STUART: c_int = 50;
pub const CLK_TOUCH: c_int = 51;
pub const CLK_TPM: c_int = 52;
pub const CLK_UDC: c_int = 53;
pub const CLK_USB: c_int = 54;
pub const CLK_USB2: c_int = 55;
pub const CLK_USBH: c_int = 56;
pub const CLK_USBHOST: c_int = 57;
pub const CLK_USIM: c_int = 58;
pub const CLK_USIM1: c_int = 59;
pub const CLK_USMI0: c_int = 60;
pub const CLK_OSC32k768: c_int = 61;
pub const CLK_MAX: c_int = 62;
