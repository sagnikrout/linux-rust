//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/c67x00.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// usb_c67x00.h: platform definitions for the Cypress C67X00 USB chip
//
// Copyright (C) 2006-2008 Barco N.V.
//
// SIE configuration
pub const C67X00_SIE_UNUSED: c_int = 0;
pub const C67X00_SIE_HOST: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c67x00_platform_data {
    pub /: *mut *mut *mut int sie_config; / SIEs config (C67X00_SIEx_),
    pub /: *mut *mut unsigned long hpi_regstep; / Step between HPI registers,
}
