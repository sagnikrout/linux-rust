//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/phantom.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright (C) 2005-2007 Jiri Slaby <jirislaby@gmail.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//

// PHN_(G/S)ET_REG param
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_reg {
    pub reg: __u32,
    pub value: __u32,
}

// PHN_(G/S)ET_REGS param
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phm_regs {
    pub count: __u32,
    pub mask: __u32,
    pub values: [__u32; 8],
}

// this ioctl tells the driver, that the caller is not OpenHaptics and might
// use improved registers update (no more phantom switchoffs when using
// libphantom)

pub const PHN_CONTROL: c_uint = 0x6     /* control byte in iaddr space */;
pub const PHN_CTL_AMP: c_uint = 0x1     /*   switch after torques change */;
pub const PHN_CTL_BUT: c_uint = 0x2     /*   is button switched */;
pub const PHN_CTL_IRQ: c_uint = 0x10    /*   is irq enabled */;

