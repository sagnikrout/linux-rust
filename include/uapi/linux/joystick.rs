//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/joystick.h
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
// Copyright (C) 1996-2000 Vojtech Pavlik
//
// Sponsored by SuSE
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA
//

//
// Version
//
pub const JS_VERSION: c_uint = 0x020100;
//
// Types and constants for reading from /dev/js
//
pub const JS_EVENT_BUTTON: c_uint = 0x01	/* button pressed/released */;
pub const JS_EVENT_AXIS: c_uint = 0x02	/* joystick moved */;
pub const JS_EVENT_INIT: c_uint = 0x80	/* initial state of device */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct js_event {
    pub /: *mut *mut __u32 time; / event timestamp in milliseconds,
    pub /: *mut *mut __s16 value; / value,
    pub /: *mut *mut __u8 type; / event type,
    pub /: *mut *mut __u8 number; / axis/button number,
}

//
// IOCTL commands for joystick driver
//

//
// Types and constants for get/set correction
//
pub const JS_CORR_NONE: c_uint = 0x00	/* returns raw values */;
pub const JS_CORR_BROKEN: c_uint = 0x01	/* broken line */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct js_corr {
    pub coef: [__s32; 8],
    pub prec: __s16,
    pub type: __u16,
}

//
// v0.x compatibility definitions
//

pub const JS_TRUE: c_int = 1;
pub const JS_FALSE: c_int = 0;
pub const JS_X_0: c_uint = 0x01;
pub const JS_Y_0: c_uint = 0x02;
pub const JS_X_1: c_uint = 0x04;
pub const JS_Y_1: c_uint = 0x08;
pub const JS_MAX: c_int = 2;
pub const JS_DEF_TIMEOUT: c_uint = 0x1300;
pub const JS_DEF_CORR: c_int = 0;

pub const JS_SET_CAL: c_int = 1;
pub const JS_GET_CAL: c_int = 2;
pub const JS_SET_TIMEOUT: c_int = 3;
pub const JS_GET_TIMEOUT: c_int = 4;
pub const JS_SET_TIMELIMIT: c_int = 5;
pub const JS_GET_TIMELIMIT: c_int = 6;
pub const JS_GET_ALL: c_int = 7;
pub const JS_SET_ALL: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JS_DATA_TYPE {
    pub buttons: __s32,
    pub x: __s32,
    pub y: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JS_DATA_SAVE_TYPE_32 {
    pub JS_TIMEOUT: __s32,
    pub BUSY: __s32,
    pub JS_EXPIRETIME: __s32,
    pub JS_TIMELIMIT: __s32,
    pub JS_SAVE: JS_DATA_TYPE,
    pub JS_CORR: JS_DATA_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct JS_DATA_SAVE_TYPE_64 {
    pub JS_TIMEOUT: __s32,
    pub BUSY: __s32,
    pub JS_EXPIRETIME: __s64,
    pub JS_TIMELIMIT: __s64,
    pub JS_SAVE: JS_DATA_TYPE,
    pub JS_CORR: JS_DATA_TYPE,
}
