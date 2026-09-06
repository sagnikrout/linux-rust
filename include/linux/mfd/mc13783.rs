//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/mc13783.h
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
// Copyright 2010 Yong Shen <yong.shen@linaro.org>
// Copyright 2009-2010 Pengutronix
// Uwe Kleine-Koenig <u.kleine-koenig@pengutronix.de>
//

pub const MC13783_REG_SW1A: c_int = 0;
pub const MC13783_REG_SW1B: c_int = 1;
pub const MC13783_REG_SW2A: c_int = 2;
pub const MC13783_REG_SW2B: c_int = 3;
pub const MC13783_REG_SW3: c_int = 4;
pub const MC13783_REG_PLL: c_int = 5;
pub const MC13783_REG_VAUDIO: c_int = 6;
pub const MC13783_REG_VIOHI: c_int = 7;
pub const MC13783_REG_VIOLO: c_int = 8;
pub const MC13783_REG_VDIG: c_int = 9;
pub const MC13783_REG_VGEN: c_int = 10;
pub const MC13783_REG_VRFDIG: c_int = 11;
pub const MC13783_REG_VRFREF: c_int = 12;
pub const MC13783_REG_VRFCP: c_int = 13;
pub const MC13783_REG_VSIM: c_int = 14;
pub const MC13783_REG_VESIM: c_int = 15;
pub const MC13783_REG_VCAM: c_int = 16;
pub const MC13783_REG_VRFBG: c_int = 17;
pub const MC13783_REG_VVIB: c_int = 18;
pub const MC13783_REG_VRF1: c_int = 19;
pub const MC13783_REG_VRF2: c_int = 20;
pub const MC13783_REG_VMMC1: c_int = 21;
pub const MC13783_REG_VMMC2: c_int = 22;
pub const MC13783_REG_GPO1: c_int = 23;
pub const MC13783_REG_GPO2: c_int = 24;
pub const MC13783_REG_GPO3: c_int = 25;
pub const MC13783_REG_GPO4: c_int = 26;
pub const MC13783_REG_V1: c_int = 27;
pub const MC13783_REG_V2: c_int = 28;
pub const MC13783_REG_V3: c_int = 29;
pub const MC13783_REG_V4: c_int = 30;
pub const MC13783_REG_PWGT1SPI: c_int = 31;
pub const MC13783_REG_PWGT2SPI: c_int = 32;

pub const MC13783_IRQ_WHIGH: c_int = 3;
pub const MC13783_IRQ_WLOW: c_int = 4;

pub const MC13783_IRQ_CHGOV: c_int = 7;

pub const MC13783_IRQ_UDP: c_int = 15;
pub const MC13783_IRQ_USB: c_int = 16;
pub const MC13783_IRQ_ID: c_int = 19;
pub const MC13783_IRQ_SE1: c_int = 21;
pub const MC13783_IRQ_CKDET: c_int = 22;
pub const MC13783_IRQ_UDM: c_int = 23;

pub const MC13783_IRQ_ONOFD1: c_int = 27;
pub const MC13783_IRQ_ONOFD2: c_int = 28;
pub const MC13783_IRQ_ONOFD3: c_int = 29;

pub const MC13783_IRQ_PWRRDY: c_int = 35;

pub const MC13783_IRQ_SEMAF: c_int = 39;
pub const MC13783_IRQ_MC2B: c_int = 41;
pub const MC13783_IRQ_HSDET: c_int = 42;
pub const MC13783_IRQ_HSL: c_int = 43;
pub const MC13783_IRQ_ALSPTH: c_int = 44;
pub const MC13783_IRQ_AHSSHORT: c_int = 45;
