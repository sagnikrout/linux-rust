//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/uniphier/aio-reg.h
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
// Socionext UniPhier AIO ALSA driver.
//
// Copyright (c) 2016-2018 Socionext Inc.
//

// Macro flag: #define SND_UNIPHIER_AIO_REG_H__

// soc-glue
pub const SG_AOUTEN: c_uint = 0x1c04;
// SW view

pub const MAPCTR0_EN: c_uint = 0x80000000;
// CTL
pub const A2APLLCTR0: c_uint = 0x07000;

pub const A2APLLCTR1: c_uint = 0x07004;
pub const A2APLLCTR1_APLLX_MASK: c_uint = 0x00010101;
pub const A2APLLCTR1_APLLX_36MHZ: c_uint = 0x00000000;
pub const A2APLLCTR1_APLLX_33MHZ: c_uint = 0x00000001;
pub const A2EXMCLKSEL0: c_uint = 0x07030;

pub const A2SSIFSW: c_uint = 0x07050;
pub const A2CH22_2CTR: c_uint = 0x07054;
pub const A2AIOINPUTSEL: c_uint = 0x070e0;

// INTC

// AIN(PCMINN)

// AIN(PBinMX)

// AOUT
pub const AOUTFADECTR0: c_uint = 0x40020;
pub const AOUTENCTR0: c_uint = 0x40040;
pub const AOUTENCTR1: c_uint = 0x40044;
pub const AOUTENCTR2: c_uint = 0x40048;
pub const AOUTRSTCTR0: c_uint = 0x40060;
pub const AOUTRSTCTR1: c_uint = 0x40064;
pub const AOUTRSTCTR2: c_uint = 0x40068;
pub const AOUTSRCRSTCTR0: c_uint = 0x400c0;
pub const AOUTSRCRSTCTR1: c_uint = 0x400c4;
pub const AOUTSRCRSTCTR2: c_uint = 0x400c8;
// AOUT PCMOUT has 5 slots, slot0-3: D0-3, slot4: DMIX
pub const OPORT_SLOT_MAX: c_int = 5;
// AOUT(PCMOUTN)

// if OPORTMXRATE_I_ACLKSRC_APLL

// AOUT(PBoutMX)

// A2D(subsystem)
pub const CDA2D_STRT0: c_uint = 0x10000;

pub const CDA2D_STAT0: c_uint = 0x10020;
pub const CDA2D_TEST: c_uint = 0x100a0;

pub const CDA2D_STRTADRSLOAD: c_uint = 0x100b0;

// A2D(ring buffer)
pub const CDA2D_RBFLUSH0: c_uint = 0x10040;
pub const CDA2D_RBADRSLOAD: c_uint = 0x100b4;
pub const CDA2D_RDPTRLOAD: c_uint = 0x100b8;

pub const CDA2D_WRPTRLOAD: c_uint = 0x100bc;

