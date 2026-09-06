//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/tvp514x_regs.h
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
// drivers/media/i2c/tvp514x_regs.h
//
// Copyright (C) 2008 Texas Instruments Inc
// Author: Vaibhav Hiremath <hvaibhav@ti.com>
//
// Contributors:
// Sivaraj R <sivaraj@ti.com>
// Brijesh R Jadav <brijesh.j@ti.com>
// Hardik Shah <hardik.shah@ti.com>
// Manjunath Hadli <mrh@ti.com>
// Karicheri Muralidharan <m-karicheri2@ti.com>
//
// TVP5146/47 registers
//

// 0x0F Reserved

// 0x13 Reserved

// 0x15 Reserved

// 0x26 - 0x27 Reserved

// 0x29 Reserved

// 0x2B Reserved

// 0x2F - 0x31 Reserved

// 0x3E Reserved

// 0x42 - 0x45 Reserved

// 0x52 - 0x68 Reserved

// 0x6A - 0x6B Reserved

// 0x6D - 0x6E Reserved

// 0x71 - 0x73 Reserved

// 0x7A - 0x7F Reserved

// 0x82 Reserved

// 0x84 - 0x96 Reserved

// 0x98 - 0x99 Reserved

// 0x9C - 0x9D Reserved

// 0x9F - 0xB0 Reserved

// 0xBE Reserved

// 0xC4 - 0xD5 Reserved

// 0xDB - 0xDF Reserved

// 0xE3 - 0xE7 Reserved

// 0xEB - 0xEF Reserved

// 0xF8 - 0xFF Reserved
//
// Mask and bit definitions of TVP5146/47 registers
//
// The ID values we are looking for

//
// Status bit
//

// Tokens for register write

//
// struct tvp514x_reg - Structure for TVP5146/47 register initialization values
// @token: Token: TOK_WRITE, TOK_TERM etc..
// @reg: Register offset
// @val: Register Value for TOK_WRITE or delay in ms for TOK_DELAY
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tvp514x_reg {
    pub token: u8,
    pub reg: u8,
    pub val: u32,
}
