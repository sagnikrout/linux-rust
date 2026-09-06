//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/audio.h
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
// <linux/usb/audio.h> -- USB Audio definitions.
//
// Copyright (C) 2006 Thumtronics Pty Ltd.
// Developed for Thumtronics by Grey Innovation
// Ben Williamson <ben.williamson@greyinnovation.com>
//
// This file holds USB constants and structures defined
// by the USB Device Class Definition for Audio Devices.
// Comments below reference relevant sections of that document:
//
// http://www.usb.org/developers/devclass_docs/audio10.pdf
//
// Types and defines in this file are either specific to version 1.0 of
// this standard or common for newer versions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_audio_control {
    pub list: list_head,
    pub name: *const c_char,
    pub type: u8,
    pub data: [c_int; 5],
    pub value): *mut *mut *mut int (set)(struct usb_audio_control con, u8 cmd, int,
    pub cmd): *mut *mut *mut int (get)(struct usb_audio_control con, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_audio_control_selector {
    pub list: list_head,
    pub control: list_head,
    pub id: u8,
    pub name: *const c_char,
    pub type: u8,
    pub desc: *mut usb_descriptor_header,
}
