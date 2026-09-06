//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/atmel/atmel-pdmic.h
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

pub const PDMIC_CR: c_uint = 0x00000000;
pub const PDMIC_CR_SWRST: c_uint = 0x1;

pub const PDMIC_CR_ENPDM_DIS: c_uint = 0x0;
pub const PDMIC_CR_ENPDM_EN: c_uint = 0x1;

pub const PDMIC_MR: c_uint = 0x00000004;
pub const PDMIC_MR_CLKS_PCK: c_uint = 0x0;
pub const PDMIC_MR_CLKS_GCK: c_uint = 0x1;

pub const PDMIC_CDR: c_uint = 0x00000014;
pub const PDMIC_IER: c_uint = 0x00000018;

pub const PDMIC_IDR: c_uint = 0x0000001c;

pub const PDMIC_IMR: c_uint = 0x00000020;
pub const PDMIC_ISR: c_uint = 0x00000024;

pub const PDMIC_DSPR0: c_uint = 0x00000058;
pub const PDMIC_DSPR0_HPFBYP_DIS: c_uint = 0x1;
pub const PDMIC_DSPR0_HPFBYP_EN: c_uint = 0x0;

pub const PDMIC_DSPR0_SINBYP_DIS: c_uint = 0x1;
pub const PDMIC_DSPR0_SINBYP_EN: c_uint = 0x0;

pub const PDMIC_DSPR0_SIZE_16_BITS: c_uint = 0x0;
pub const PDMIC_DSPR0_SIZE_32_BITS: c_uint = 0x1;

pub const PDMIC_DSPR0_OSR_128: c_uint = 0x0;
pub const PDMIC_DSPR0_OSR_64: c_uint = 0x1;

pub const PDMIC_DSPR1: c_uint = 0x0000005c;

pub const PDMIC_WPMR: c_uint = 0x000000e4;
pub const PDMIC_WPSR: c_uint = 0x000000e8;
