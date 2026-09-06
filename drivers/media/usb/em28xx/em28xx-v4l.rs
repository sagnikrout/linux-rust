//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/em28xx/em28xx-v4l.h
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
// em28xx-video.c - driver for Empia EM2800/EM2820/2840 USB
// video capture devices
//
// Copyright (C) 2013-2014 Mauro Carvalho Chehab <mchehab+samsung@kernel.org>
//
extern "C" {
    pub fn em28xx_start_analog_streaming(vq: *mut vb2_queue, count: c_uint) -> c_int;
}
extern "C" {
    pub fn em28xx_stop_vbi_streaming(vq: *mut vb2_queue);
}
