//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/hid-roccat-savu.h
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
// Copyright (c) 2012 Stefan Achatz <erazor_de@users.sourceforge.net>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savu_mouse_report_special {
    pub /: *mut *mut uint8_t report_number; / always 3,
    pub zero: u8,
    pub type: u8,
    pub data: [u8; 2],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum savu_mouse_report_button_types {
// data1 = new profile range 1-5
    SAVU_MOUSE_REPORT_BUTTON_TYPE_PROFILE = 0x20,

// data1 = button number range 1-24; data2 = action
    SAVU_MOUSE_REPORT_BUTTON_TYPE_QUICKLAUNCH = 0x60,

// data1 = button number range 1-24; data2 = action
    SAVU_MOUSE_REPORT_BUTTON_TYPE_TIMER = 0x80,

// data1 = setting number range 1-5
    SAVU_MOUSE_REPORT_BUTTON_TYPE_CPI = 0xb0,

// data1 and data2 = range 0x1-0xb
    SAVU_MOUSE_REPORT_BUTTON_TYPE_SENSITIVITY = 0xc0,

// data1 = 22 = next track...
// data2 = action
//
    SAVU_MOUSE_REPORT_BUTTON_TYPE_MULTIMEDIA = 0xf0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct savu_roccat_report {
    pub type: u8,
    pub data: [u8; 2],
    pub __packed: },
