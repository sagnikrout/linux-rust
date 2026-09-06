//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/at91/at91sam9_sdramc.h
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
// arch/arm/mach-at91/include/mach/at91sam9_sdramc.h
//
// Copyright (C) 2007 Andrew Victor
// Copyright (C) 2007 Atmel Corporation.
//
// SDRAM Controllers (SDRAMC) - System peripherals registers.
// Based on AT91SAM9261 datasheet revision D.
//
// SDRAM Controller (SDRAMC) registers
pub const AT91_SDRAMC_MR: c_uint = 0x00	/* SDRAM Controller Mode Register */;

pub const AT91_SDRAMC_MODE_NORMAL: c_int = 0;
pub const AT91_SDRAMC_MODE_NOP: c_int = 1;
pub const AT91_SDRAMC_MODE_PRECHARGE: c_int = 2;
pub const AT91_SDRAMC_MODE_LMR: c_int = 3;
pub const AT91_SDRAMC_MODE_REFRESH: c_int = 4;
pub const AT91_SDRAMC_MODE_EXT_LMR: c_int = 5;
pub const AT91_SDRAMC_MODE_DEEP: c_int = 6;
pub const AT91_SDRAMC_TR: c_uint = 0x04	/* SDRAM Controller Refresh Timer Register */;

pub const AT91_SDRAMC_CR: c_uint = 0x08	/* SDRAM Controller Configuration Register */;

pub const AT91_SDRAMC_LPR: c_uint = 0x10	/* SDRAM Controller Low Power Register */;

pub const AT91_SDRAMC_LPCB_DISABLE: c_int = 0;
pub const AT91_SDRAMC_LPCB_SELF_REFRESH: c_int = 1;
pub const AT91_SDRAMC_LPCB_POWER_DOWN: c_int = 2;
pub const AT91_SDRAMC_LPCB_DEEP_POWER_DOWN: c_int = 3;

pub const AT91_SDRAMC_IER: c_uint = 0x14	/* SDRAM Controller Interrupt Enable Register */;
pub const AT91_SDRAMC_IDR: c_uint = 0x18	/* SDRAM Controller Interrupt Disable Register */;
pub const AT91_SDRAMC_IMR: c_uint = 0x1C	/* SDRAM Controller Interrupt Mask Register */;
pub const AT91_SDRAMC_ISR: c_uint = 0x20	/* SDRAM Controller Interrupt Status Register */;

pub const AT91_SDRAMC_MDR: c_uint = 0x24	/* SDRAM Memory Device Register */;

pub const AT91_SDRAMC_MD_SDRAM: c_int = 0;
pub const AT91_SDRAMC_MD_LOW_POWER_SDRAM: c_int = 1;
