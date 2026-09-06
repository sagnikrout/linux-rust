//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/geode-aes.h
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
// Copyright (C) 2003-2006, Advanced Micro Devices, Inc.
//
// driver logic flags
pub const AES_MODE_ECB: c_int = 0;
pub const AES_MODE_CBC: c_int = 1;
pub const AES_DIR_DECRYPT: c_int = 0;
pub const AES_DIR_ENCRYPT: c_int = 1;

// Register definitions
pub const AES_CTRLA_REG: c_uint = 0x0000;
pub const AES_CTRL_START: c_uint = 0x01;
pub const AES_CTRL_DECRYPT: c_uint = 0x00;
pub const AES_CTRL_ENCRYPT: c_uint = 0x02;
pub const AES_CTRL_WRKEY: c_uint = 0x04;
pub const AES_CTRL_DCA: c_uint = 0x08;
pub const AES_CTRL_SCA: c_uint = 0x10;
pub const AES_CTRL_CBC: c_uint = 0x20;
pub const AES_INTR_REG: c_uint = 0x0008;

pub const AES_INTR_MASK: c_uint = 0x07;
pub const AES_SOURCEA_REG: c_uint = 0x0010;
pub const AES_DSTA_REG: c_uint = 0x0014;
pub const AES_LENA_REG: c_uint = 0x0018;
pub const AES_WRITEKEY0_REG: c_uint = 0x0030;
pub const AES_WRITEIV0_REG: c_uint = 0x0040;
// A very large counter that is used to gracefully bail out of an
// operation in case of trouble
//
pub const AES_OP_TIMEOUT: c_uint = 0x50000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geode_aes_tfm_ctx {
    pub key: [u8; AES_KEYSIZE_128],
    pub skcipher: *mut crypto_skcipher,
    pub cip: *mut crypto_cipher,
    pub fallback: },
    pub keylen: u32,
}
