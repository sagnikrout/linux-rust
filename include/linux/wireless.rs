//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wireless.h
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
//
// This file define a set of standard wireless extensions
//
// Version :	22	16.3.07
//
// Authors :	Jean Tourrilhes - HPL - <jt@hpl.hp.com>
// Copyright (c) 1997-2007 Jean Tourrilhes, All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_iw_point {
    pub pointer: compat_caddr_t,
    pub length: __u16,
    pub flags: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __compat_iw_event {
    pub /: *mut *mut __u16 len; / Real length of this stuff,
    pub /: *mut *mut __u16 cmd; / Wireless IOCTL,
    pub pointer: compat_caddr_t,
// we need ptr_bytes to make memcpy() run-time destination
// buffer bounds checking happy, nothing special
//
    pub ptr_bytes): DECLARE_FLEX_ARRAY(__u8,,
}

// Size of the various events for compat

