//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mb86a20s.h
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
// Fujitsu mb86a20s driver
//
// Copyright (C) 2010 Mauro Carvalho Chehab
//

//
// struct mb86a20s_config - Define the per-device attributes of the frontend
//
// @fclk:		Clock frequency. If zero, assumes the default
// (32.57142 Mhz)
// @demod_address:	the demodulator's i2c address
// @is_serial:		if true, TS is serial. Otherwise, TS is parallel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mb86a20s_config {
    pub fclk: u32,
    pub demod_address: u8,
    pub is_serial: bool,
}

//
// mb86a20s_attach - Attach a mb86a20s demod
//
// @config: pointer to &struct mb86a20s_config with demod configuration.
// @i2c: i2c adapter to use.
//
// return: FE pointer on success, NULL on failure.
//

