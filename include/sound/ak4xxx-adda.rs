//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ak4xxx-adda.h
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
// ALSA driver for AK4524 / AK4528 / AK4529 / AK4355 / AK4381
// AD and DA converters
//
// Copyright (c) 2000 Jaroslav Kysela <perex@perex.cz>
//

pub const AK4XXX_MAX_CHIPS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ak4xxx_ops {
    pub chip): *mut *mut *mut void (lock)(struct snd_akm4xxx ak, int,
    pub chip): *mut *mut *mut void (unlock)(struct snd_akm4xxx ak, int,
    pub val): c_uchar,
    pub rate): *mut *mut *mut void (set_rate_val)(struct snd_akm4xxx ak, unsigned int,
}

// DAC label and channels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_akm4xxx_dac_channel {
    pub /: *mut *mut *mut char name; / mixer volume name,
    pub num_channels: c_uint,
    pub switch*/: *mut *mut *mut char switch_name; / mixer,
}

// ADC labels and channels
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_akm4xxx_adc_channel {
    pub /: *mut *mut *mut char name; / capture gain volume label,
    pub /: *mut *mut *mut char switch_name; / capture switch,
    pub num_channels: c_uint,
    pub /: *mut *mut *mut char selector_name; / capture source select label,
    pub /: *const *const *const *const char input_names; / capture source names (NULL terminated),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_akm4xxx {
    pub card: *mut snd_card,
    pub /: *mut *mut unsigned int num_adcs; / AK4524 or AK4528 ADCs,
    pub /: *mut *mut unsigned int num_dacs; / AK4524 or AK4528 DACs,
    pub /: *mut *mut unsigned char images[AK4XXX_IMAGE_SIZE]; / saved register image,
    pub /: *mut *mut unsigned char volumes[AK4XXX_IMAGE_SIZE]; / saved volume values,
    pub /: *mut *mut unsigned long private_value[AK4XXX_MAX_CHIPS]; / helper for driver,
    pub /: *mut *mut *mut void private_data[AK4XXX_MAX_CHIPS]; / helper for driver,
// template should fill the following fields
    pub /: *mut *mut unsigned int idx_offset; / control index offset,
    pub type: },
// (array) information of combined codecs
    pub dac_info: *const snd_akm4xxx_dac_channel,
    pub adc_info: *const snd_akm4xxx_adc_channel,
    pub ops: snd_ak4xxx_ops,
    pub num_chips: c_uint,
    pub total_regs: c_uint,
    pub name: *const c_char,
}

extern "C" {
    pub fn snd_akm4xxx_reset(ak: *mut snd_akm4xxx, state: c_int);
}
extern "C" {
    pub fn snd_akm4xxx_init(ak: *mut snd_akm4xxx);
}
extern "C" {
    pub fn snd_akm4xxx_build_controls(ak: *mut snd_akm4xxx) -> c_int;
}

