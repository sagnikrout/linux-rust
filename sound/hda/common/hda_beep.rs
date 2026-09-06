//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/common/hda_beep.h
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
// Digital Beep Input Interface for HD-audio codec
//
// Author: Matt Ranostay <matt.ranostay@konsulko.com>
// Copyright (c) 2008 Embedded Alley Solutions Inc
//

pub const HDA_BEEP_MODE_OFF: c_int = 0;
pub const HDA_BEEP_MODE_ON: c_int = 1;
// beep information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hda_beep {
    pub dev: *mut input_dev,
    pub codec: *mut hda_codec,
    pub phys: [c_char; 32],
    pub tone: c_int,
    pub nid: hda_nid_t,
    pub registered:1: c_uint,
    pub enabled:1: c_uint,
    pub /: *mut *mut unsigned int linear_tone:1; / linear tone for IDT/STAC codec,
    pub playing:1: c_uint,
    pub /: *mut *mut unsigned int keep_power_at_enable:1; / set by driver,
    pub /: *mut *mut work_beep_work; / scheduled task for beep event,
    pub on): *mut *mut *mut void (power_hook)(struct hda_beep beep, bool,
}

extern "C" {
    pub fn snd_hda_enable_beep_device(codec: *mut hda_codec, enable: c_int) -> c_int;
}
extern "C" {
    pub fn snd_hda_attach_beep_device(codec: *mut hda_codec, nid: c_int) -> c_int;
}
extern "C" {
    pub fn snd_hda_detach_beep_device(codec: *mut hda_codec);
}

