//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/tea575x.h
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
// ALSA driver for TEA5757/5759 Philips AM/FM tuner chips
//
// Copyright (c) 2004 Jaroslav Kysela <perex@perex.cz>
//

pub const TEA575X_FMIF: c_int = 10700;
pub const TEA575X_AMIF: c_int = 450;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_tea575x_ops {
// Drivers using snd_tea575x must either define read_ and write_val
    pub val): *mut *mut *mut void (write_val)(struct snd_tea575x tea, u32,
    pub tea): *mut *mut u32 (read_val)(struct snd_tea575x,
// Or define the 3 pin functions
    pub pins): *mut *mut *mut void (set_pins)(struct snd_tea575x tea, u8,
    pub tea): *mut *mut u8 (get_pins)(struct snd_tea575x,
    pub output): *mut *mut *mut void (set_direction)(struct snd_tea575x tea, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_tea575x {
    pub v4l2_dev: *mut v4l2_device,
    pub fops: v4l2_file_operations,
    pub /: *mut *mut video_device vd; / video device,
    pub /: *mut *mut int radio_nr; / radio_nr,
    pub /: *mut *mut bool tea5759; / 5759 chip is present,
    pub /: *mut *mut bool has_am; / Device can tune to AM freqs,
    pub /: *mut *mut bool cannot_read_data; / Device cannot read the data pin,
    pub /: *mut *mut bool cannot_mute; / Device cannot mute,
    pub /: *mut *mut bool mute; / Device is muted?,
    pub /: *mut *mut bool stereo; / receiving stereo,
    pub /: *mut *mut bool tuned; / tuned to a station,
    pub /: *mut *mut unsigned int val; / hw value,
    pub /: *mut *mut u32 band; / 0: FM, 1: FM-Japan, 2: AM,
    pub /: *mut *mut u32 freq; / frequency,
    pub mutex: mutex,
    pub ops: *const snd_tea575x_ops,
    pub private_data: *mut c_void,
    pub card: [u8; 32],
    pub bus_info: [u8; 32],
    pub ctrl_handler: v4l2_ctrl_handler,
    pub tea): *mut *mut int (ext_init)(struct snd_tea575x,
}

extern "C" {
    pub fn snd_tea575x_g_tuner(tea: *mut snd_tea575x, v: *mut v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn snd_tea575x_hw_init(tea: *mut snd_tea575x) -> c_int;
}
extern "C" {
    pub fn snd_tea575x_init(tea: *mut snd_tea575x, owner: *mut module) -> c_int;
}
extern "C" {
    pub fn snd_tea575x_exit(tea: *mut snd_tea575x);
}
extern "C" {
    pub fn snd_tea575x_set_freq(tea: *mut snd_tea575x);
}
