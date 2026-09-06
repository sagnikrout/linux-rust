//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/intel_soc_pmic_mrfld.h
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
// Header file for Intel Merrifield Basin Cove PMIC
//
// Copyright (C) 2019 Intel Corporation. All rights reserved.
//

pub const BCOVE_ID: c_uint = 0x00;

pub const BCOVE_IRQLVL1: c_uint = 0x01;
pub const BCOVE_PBIRQ: c_uint = 0x02;
pub const BCOVE_TMUIRQ: c_uint = 0x03;
pub const BCOVE_THRMIRQ: c_uint = 0x04;
pub const BCOVE_BCUIRQ: c_uint = 0x05;
pub const BCOVE_ADCIRQ: c_uint = 0x06;
pub const BCOVE_CHGRIRQ0: c_uint = 0x07;
pub const BCOVE_CHGRIRQ1: c_uint = 0x08;
pub const BCOVE_GPIOIRQ: c_uint = 0x09;
pub const BCOVE_CRITIRQ: c_uint = 0x0B;
pub const BCOVE_MIRQLVL1: c_uint = 0x0C;
pub const BCOVE_MPBIRQ: c_uint = 0x0D;
pub const BCOVE_MTMUIRQ: c_uint = 0x0E;
pub const BCOVE_MTHRMIRQ: c_uint = 0x0F;
pub const BCOVE_MBCUIRQ: c_uint = 0x10;
pub const BCOVE_MADCIRQ: c_uint = 0x11;
pub const BCOVE_MCHGRIRQ0: c_uint = 0x12;
pub const BCOVE_MCHGRIRQ1: c_uint = 0x13;
pub const BCOVE_MGPIOIRQ: c_uint = 0x14;
pub const BCOVE_MCRITIRQ: c_uint = 0x16;
pub const BCOVE_SCHGRIRQ0: c_uint = 0x4E;
pub const BCOVE_SCHGRIRQ1: c_uint = 0x4F;
// Level 1 IRQs

// Level 2 IRQs: power button

// Level 2 IRQs: ADC

// Level 2 IRQs: charger

