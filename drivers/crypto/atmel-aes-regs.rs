//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/atmel-aes-regs.h
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
pub const AES_CR: c_uint = 0x00;

pub const AES_MR: c_uint = 0x04;

pub const AES_MR_PROCDLY_OFFSET: c_int = 4;

pub const AES_MR_CKEY_OFFSET: c_int = 20;

pub const AES_MR_CMTYP_OFFSET: c_int = 24;
pub const AES_IER: c_uint = 0x10;
pub const AES_IDR: c_uint = 0x14;
pub const AES_IMR: c_uint = 0x18;
pub const AES_ISR: c_uint = 0x1C;

pub const AES_AADLENR: c_uint = 0x70;
pub const AES_CLENR: c_uint = 0x74;

pub const AES_CTRR: c_uint = 0x98;

pub const AES_EMR: c_uint = 0xb0;

pub const AES_EMR_APM_IPSEC: c_uint = 0x0;

pub const AES_EMR_PADLEN_OFFSET: c_int = 8;

pub const AES_EMR_NHEAD_OFFSET: c_int = 16;

pub const AES_HW_VERSION: c_uint = 0xFC;
