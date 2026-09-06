//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/atmel-sha-regs.h
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

pub const SHA_CR: c_uint = 0x00;

pub const SHA_MR: c_uint = 0x04;

pub const SHA_MR_MODE_MANUAL: c_uint = 0x0;
pub const SHA_MR_MODE_AUTO: c_uint = 0x1;
pub const SHA_MR_MODE_PDC: c_uint = 0x2;
pub const SHA_MR_MODE_IDATAR0: c_uint = 0x2;

pub const SHA_IER: c_uint = 0x10;
pub const SHA_IDR: c_uint = 0x14;
pub const SHA_IMR: c_uint = 0x18;
pub const SHA_ISR: c_uint = 0x1C;

pub const SHA_MSR: c_uint = 0x20;
pub const SHA_BCR: c_uint = 0x30;
pub const SHA_HW_VERSION: c_uint = 0xFC;
pub const SHA_TPR: c_uint = 0x108;
pub const SHA_TCR: c_uint = 0x10C;
pub const SHA_TNPR: c_uint = 0x118;
pub const SHA_TNCR: c_uint = 0x11C;
pub const SHA_PTCR: c_uint = 0x120;

pub const SHA_PTSR: c_uint = 0x124;

