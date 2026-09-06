//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/at91/at91sam9_ddrsdr.h
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
// Header file for the Atmel DDR/SDR SDRAM Controller
//
// Copyright (C) 2010 Atmel Corporation
// Nicolas Ferre <nicolas.ferre@atmel.com>
//
pub const AT91_DDRSDRC_MR: c_uint = 0x00	/* Mode Register */;

pub const AT91_DDRSDRC_MODE_NORMAL: c_int = 0;
pub const AT91_DDRSDRC_MODE_NOP: c_int = 1;
pub const AT91_DDRSDRC_MODE_PRECHARGE: c_int = 2;
pub const AT91_DDRSDRC_MODE_LMR: c_int = 3;
pub const AT91_DDRSDRC_MODE_REFRESH: c_int = 4;
pub const AT91_DDRSDRC_MODE_EXT_LMR: c_int = 5;
pub const AT91_DDRSDRC_MODE_DEEP: c_int = 6;
pub const AT91_DDRSDRC_RTR: c_uint = 0x04	/* Refresh Timer Register */;

pub const AT91_DDRSDRC_CR: c_uint = 0x08	/* Configuration Register */;

pub const AT91_DDRSDRC_T0PR: c_uint = 0x0C	/* Timing 0 Register */;

pub const AT91_DDRSDRC_T1PR: c_uint = 0x10	/* Timing 1 Register */;

pub const AT91_DDRSDRC_T2PR: c_uint = 0x14	/* Timing 2 Register [SAM9 Only] */;

pub const AT91_DDRSDRC_LPR: c_uint = 0x1C	/* Low Power Register */;

pub const AT91_DDRSDRC_LPCB_DISABLE: c_int = 0;
pub const AT91_DDRSDRC_LPCB_SELF_REFRESH: c_int = 1;
pub const AT91_DDRSDRC_LPCB_POWER_DOWN: c_int = 2;
pub const AT91_DDRSDRC_LPCB_DEEP_POWER_DOWN: c_int = 3;

pub const AT91_DDRSDRC_MDR: c_uint = 0x20	/* Memory Device Register */;

pub const AT91_DDRSDRC_MD_SDR: c_int = 0;
pub const AT91_DDRSDRC_MD_LOW_POWER_SDR: c_int = 1;
pub const AT91_DDRSDRC_MD_LOW_POWER_DDR: c_int = 3;
pub const AT91_DDRSDRC_MD_LPDDR3: c_int = 5;

pub const AT91_DDRSDRC_MD_LPDDR2: c_int = 7;

pub const AT91_DDRSDRC_DLL: c_uint = 0x24	/* DLL Information Register */;

pub const AT91_DDRSDRC_HS: c_uint = 0x2C	/* High Speed Register [SAM9 Only] */;

pub const AT91_DDRSDRC_WPMR: c_uint = 0xE4	/* Write Protect Mode Register [SAM9 Only] */;

pub const AT91_DDRSDRC_WPSR: c_uint = 0xE8	/* Write Protect Status Register [SAM9 Only] */;

