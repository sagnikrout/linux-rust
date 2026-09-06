//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/solo6x10/solo6x10-tw28.h
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//

pub const TW_NUM_CHIP: c_int = 4;
pub const TW_BASE_ADDR: c_uint = 0x28;

// tw2815
pub const TW_AV_STAT_ADDR: c_uint = 0x5a;

pub const TW_AUDIO_OUTPUT_VOL_ADDR: c_uint = 0x70;

// tw286x
pub const TW286x_AV_STAT_ADDR: c_uint = 0xfd;

pub const TW286x_AUDIO_OUTPUT_VOL_ADDR: c_uint = 0xdf;

extern "C" {
    pub fn solo_tw28_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn tw28_set_ctrl_val(solo_dev: *mut solo_dev, ctrl: u32, ch: u8, val: i32) -> c_int;
}
extern "C" {
    pub fn tw28_get_ctrl_val(solo_dev: *mut solo_dev, ctrl: u32, ch: u8, val: *mut i32) -> c_int;
}
extern "C" {
    pub fn tw28_has_sharpness(solo_dev: *mut solo_dev, ch: u8) -> bool;
}
extern "C" {
    pub fn tw28_get_audio_gain(solo_dev: *mut solo_dev, ch: u8) -> u8;
}
extern "C" {
    pub fn tw28_set_audio_gain(solo_dev: *mut solo_dev, ch: u8, val: u8);
}
extern "C" {
    pub fn tw28_get_video_status(solo_dev: *mut solo_dev, ch: u8) -> c_int;
}

extern "C" {
    pub fn tw2815_get_audio_status(solo: *mut SOLO) -> c_uint;
}
extern "C" {
    pub fn tw2815_Set_AudioOutVol(solo: *mut SOLO, u_val: c_uint);
}

