//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/common/siano/smsir.h
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
// Siano Mobile Silicon, Inc.
// MDTV receiver kernel modules.
// Copyright (C) 2006-2009, Uri Shkolnik
//
// Copyright (c) 2010 - Mauro Carvalho Chehab
// - Ported the driver to use rc-core
// - IR raw event decoding is now done at rc-core
// - Code almost re-written
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ir_t {
    pub dev: *mut rc_dev,
    pub name: [c_char; 40],
    pub phys: [c_char; 32],
    pub rc_codes: *mut c_char,
    pub timeout: u32,
    pub controller: u32,
}

extern "C" {
    pub fn sms_ir_init(coredev: *mut smscore_device_t) -> c_int;
}
extern "C" {
    pub fn sms_ir_exit(coredev: *mut smscore_device_t);
}

