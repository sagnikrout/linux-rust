//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/sentelic.h
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
// -
// Finger Sensing Pad PS/2 mouse driver.
//
// Copyright (C) 2005-2007 Asia Vital Components Co., Ltd.
// Copyright (C) 2005-2012 Tai-hwa Liang, Sentelic Corporation.
//
// Finger-sensing Pad information registers
pub const FSP_REG_DEVICE_ID: c_uint = 0x00;
pub const FSP_REG_VERSION: c_uint = 0x01;
pub const FSP_REG_REVISION: c_uint = 0x04;
pub const FSP_REG_TMOD_STATUS1: c_uint = 0x0B;

pub const FSP_REG_PAGE_CTRL: c_uint = 0x0F;
// Finger-sensing Pad control registers
pub const FSP_REG_SYSCTL1: c_uint = 0x10;

pub const FSP_REG_TMOD_STATUS: c_uint = 0x20;
pub const FSP_REG_OPC_QDOWN: c_uint = 0x31;

pub const FSP_REG_OPTZ_XLO: c_uint = 0x34;
pub const FSP_REG_OPTZ_XHI: c_uint = 0x35;
pub const FSP_REG_OPTZ_YLO: c_uint = 0x36;
pub const FSP_REG_OPTZ_YHI: c_uint = 0x37;
pub const FSP_REG_SYSCTL5: c_uint = 0x40;

pub const FSP_REG_ONPAD_CTL: c_uint = 0x43;

// Finger-sensing Pad packet formating related definitions
// absolute packet type

// bit definitions for the first byte of report packet

// hardware revisions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsp_data {
    pub /: *mut *mut unsigned char ver; / hardware version,
    pub /: *mut *mut unsigned char rev; / hardware revison,
    pub /: *mut *mut unsigned int buttons; / Number of buttons,
    pub flags: c_uint,

    pub /: *mut *mut bool vscroll; / Vertical scroll zone enabled,
    pub /: *mut *mut bool hscroll; / Horizontal scroll zone enabled,
    pub /: *mut *mut unsigned char last_reg; / Last register we requested read from,
    pub last_val: c_uchar,
    pub /: *mut *mut unsigned int last_mt_fgr; / Last seen finger(multitouch),
}

extern "C" {
    pub fn fsp_detect(psmouse: *mut psmouse, set_properties: bool) -> c_int;
}
extern "C" {
    pub fn fsp_init(psmouse: *mut psmouse) -> c_int;
}

