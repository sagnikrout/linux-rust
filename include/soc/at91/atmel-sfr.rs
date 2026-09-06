//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/at91/atmel-sfr.h
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
// Atmel SFR (Special Function Registers) register offsets and bit definitions.
//
// Copyright (C) 2016 Atmel
//
// Author: Ludovic Desroches <ludovic.desroches@atmel.com>
//
pub const AT91_SFR_DDRCFG: c_uint = 0x04	/* DDR Configuration Register */;
pub const AT91_SFR_CCFG_EBICSA: c_uint = 0x04	/* EBI Chip Select Register */;
// 0x08 ~ 0x0c: Reserved
pub const AT91_SFR_OHCIICR: c_uint = 0x10	/* OHCI INT Configuration Register */;
pub const AT91_SFR_OHCIISR: c_uint = 0x14	/* OHCI INT Status Register */;
pub const AT91_SFR_UTMICKTRIM: c_uint = 0x30	/* UTMI Clock Trimming Register */;
pub const AT91_SFR_UTMISWAP: c_uint = 0x3c	/* UTMI DP/DM Pin Swapping Register */;
pub const AT91_SFR_LS: c_uint = 0x7c	/* Light Sleep Register */;
pub const AT91_SFR_I2SCLKSEL: c_uint = 0x90	/* I2SC Register */;
pub const AT91_SFR_WPMR: c_uint = 0xe4	/* Write Protection Mode Register */;
// Field definitions

