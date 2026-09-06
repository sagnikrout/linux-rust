//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/radio/radio-tea5777.h
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
// v4l2 driver for TEA5777 Philips AM/FM radio tuner chips
//
// Copyright (c) 2012 Hans de Goede <hdegoede@redhat.com>
//
// Based on the ALSA driver for TEA5757/5759 Philips AM/FM radio tuner chips:
//
// Copyright (c) 2004 Jaroslav Kysela <perex@perex.cz>
// Copyright (c) 2012 Hans de Goede <hdegoede@redhat.com>
//

pub const TEA575X_FMIF: c_int = 10700;
pub const TEA575X_AMIF: c_int = 450;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_tea5777_ops {
//
// Write the 6 bytes large write register of the tea5777
//
// val represents the 6 write registers, with byte 1 from the
// datasheet being the most significant byte (so byte 5 of the u64),
// and byte 6 from the datasheet being the least significant byte.
//
// returns 0 on success.
//
    pub val): *mut *mut *mut int (write_reg)(struct radio_tea5777 tea, u64,
//
// Read the 3 bytes large read register of the tea5777
//
// The read value gets returned in val, akin to write_reg, byte 1 from
// the datasheet is stored as the most significant byte (so byte 2 of
// the u32), and byte 3 from the datasheet gets stored as the least
// significant byte (iow byte 0 of the u32).
//
// returns 0 on success.
//
    pub val): *mut *mut *mut int (read_reg)(struct radio_tea5777 tea, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radio_tea5777 {
    pub v4l2_dev: *mut v4l2_device,
    pub fops: v4l2_file_operations,
    pub /: *mut *mut video_device vd; / video device,
    pub /: *mut *mut bool has_am; / Device can tune to AM freqs,
    pub /: *mut *mut bool write_before_read; / must write before read quirk,
    pub /: *mut *mut bool needs_write; / for write before read quirk,
    pub /: *mut *mut u32 band; / current band,
    pub /: *mut *mut u32 freq; / current frequency,
    pub /: *mut *mut u32 audmode; / last set audmode,
    pub /: *mut *mut u32 seek_rangelow; / current hwseek limits,
    pub seek_rangehigh: u32,
    pub read_reg: u32,
    pub write_reg: u64,
    pub mutex: mutex,
    pub ops: *const radio_tea5777_ops,
    pub private_data: *mut c_void,
    pub card: [u8; 32],
    pub bus_info: [u8; 32],
    pub ctrl_handler: v4l2_ctrl_handler,
}

extern "C" {
    pub fn radio_tea5777_init(tea: *mut radio_tea5777, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn radio_tea5777_exit(tea: *mut radio_tea5777);
}
extern "C" {
    pub fn radio_tea5777_set_freq(tea: *mut radio_tea5777) -> c_int;
}
