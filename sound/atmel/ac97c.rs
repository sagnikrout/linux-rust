//! Automatically rewritten from C Header to Rust Module
//! Source: sound/atmel/ac97c.h
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
// Register definitions for Atmel AC97C
//
// Copyright (C) 2005-2009 Atmel Corporation
//
pub const AC97C_MR: c_uint = 0x08;
pub const AC97C_ICA: c_uint = 0x10;
pub const AC97C_OCA: c_uint = 0x14;
pub const AC97C_CARHR: c_uint = 0x20;
pub const AC97C_CATHR: c_uint = 0x24;
pub const AC97C_CASR: c_uint = 0x28;
pub const AC97C_CAMR: c_uint = 0x2c;
pub const AC97C_CORHR: c_uint = 0x40;
pub const AC97C_COTHR: c_uint = 0x44;
pub const AC97C_COSR: c_uint = 0x48;
pub const AC97C_COMR: c_uint = 0x4c;
pub const AC97C_SR: c_uint = 0x50;
pub const AC97C_IER: c_uint = 0x54;
pub const AC97C_IDR: c_uint = 0x58;
pub const AC97C_IMR: c_uint = 0x5c;
pub const AC97C_VERSION: c_uint = 0xfc;

pub const AC97C_CHANNEL_NONE: c_uint = 0x0;
pub const AC97C_CHANNEL_A: c_uint = 0x1;
