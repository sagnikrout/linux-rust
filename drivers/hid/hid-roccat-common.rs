//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-common.h
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
//
// Copyright (c) 2011 Stefan Achatz <erazor_de@users.sourceforge.net>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum roccat_common2_commands {
    ROCCAT_COMMON_COMMAND_CONTROL = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct roccat_common2_control {
    pub command: u8,
    pub value: u8,
    pub /: *mut *mut uint8_t request; / always 0 on requesting write check,
    pub __packed: },
    pub size): *mut *mut void data, uint,
    pub size): *const *const void data, uint,
    pub size): *const *const uint command, void buf, uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct roccat_common2_device {
    pub roccat_claimed: c_int,
    pub chrdev_minor: c_int,
    pub lock: mutex,
}

