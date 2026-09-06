//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/mixer_oss.h
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
// OSS MIXER API
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//

pub const SNDRV_OSS_MAX_MIXERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mixer_oss_slot {
    pub number: c_int,
    pub 1: unsigned int stereo:,
    pub right): *mut *mut int left, int,
    pub right): int left, int,
    pub active): *mut c_int,
    pub active): c_int,
    pub private_value: c_ulong,
    pub private_data: *mut c_void,
    pub slot): *mut *mut void (private_free)(struct snd_mixer_oss_slot,
    pub volume: [c_int; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mixer_oss {
    pub card: *mut snd_card,
    pub id: [c_char; 16],
    pub name: [c_char; 32],
    pub /: *mut *mut snd_mixer_oss_slot slots[SNDRV_OSS_MAX_MIXERS]; / OSS mixer slots,
    pub /: *mut *mut unsigned int mask_recsrc; / exclusive recsrc mask,
    pub active_index): *mut c_uint,
    pub active_index): c_uint,
    pub private_data_recsrc: *mut c_void,
    pub mixer): *mut *mut void (private_free_recsrc)(struct snd_mixer_oss,
    pub reg_mutex: mutex,
    pub proc_entry: *mut snd_info_entry,
    pub oss_dev_alloc: c_int,
// ---
    pub oss_recsrc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mixer_oss_file {
    pub card: *mut snd_card,
    pub mixer: *mut snd_mixer_oss,
}

