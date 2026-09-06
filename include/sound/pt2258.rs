//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pt2258.h
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
// ALSA Driver for the PT2258 volume controller.
//
// Copyright (c) 2006  Jochen Voss <voss@seehuhn.de>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pt2258 {
    pub card: *mut snd_card,
    pub i2c_bus: *mut snd_i2c_bus,
    pub i2c_dev: *mut snd_i2c_device,
    pub volume: [c_uchar; 6],
    pub mute: c_int,
}

extern "C" {
    pub fn snd_pt2258_reset(pt: *mut snd_pt2258) -> c_int;
}
extern "C" {
    pub fn snd_pt2258_build_controls(pt: *mut snd_pt2258) -> c_int;
}
