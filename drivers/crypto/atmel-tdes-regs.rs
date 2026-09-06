//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/atmel-tdes-regs.h
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
pub const TDES_CR: c_uint = 0x00;

pub const TDES_MR: c_uint = 0x04;

pub const TDES_MR_CKEY_OFFSET: c_int = 20;

pub const TDES_MR_CTYPE_OFFSET: c_int = 24;
pub const TDES_IER: c_uint = 0x10;
pub const TDES_IDR: c_uint = 0x14;
pub const TDES_IMR: c_uint = 0x18;
pub const TDES_ISR: c_uint = 0x1C;

pub const TDES_KEY1W1R: c_uint = 0x20;
pub const TDES_KEY1W2R: c_uint = 0x24;
pub const TDES_KEY2W1R: c_uint = 0x28;
pub const TDES_KEY2W2R: c_uint = 0x2C;
pub const TDES_KEY3W1R: c_uint = 0x30;
pub const TDES_KEY3W2R: c_uint = 0x34;
pub const TDES_IDATA1R: c_uint = 0x40;
pub const TDES_IDATA2R: c_uint = 0x44;
pub const TDES_ODATA1R: c_uint = 0x50;
pub const TDES_ODATA2R: c_uint = 0x54;
pub const TDES_IV1R: c_uint = 0x60;
pub const TDES_IV2R: c_uint = 0x64;
pub const TDES_XTEARNDR: c_uint = 0x70;

pub const TDES_XTEARNDR_XTEA_RNDS_OFFSET: c_int = 0;
pub const TDES_HW_VERSION: c_uint = 0xFC;
pub const TDES_RPR: c_uint = 0x100;
pub const TDES_RCR: c_uint = 0x104;
pub const TDES_TPR: c_uint = 0x108;
pub const TDES_TCR: c_uint = 0x10C;
pub const TDES_RNPR: c_uint = 0x118;
pub const TDES_RNCR: c_uint = 0x11C;
pub const TDES_TNPR: c_uint = 0x118;
pub const TDES_TNCR: c_uint = 0x11C;
pub const TDES_PTCR: c_uint = 0x120;

pub const TDES_PTSR: c_uint = 0x124;

