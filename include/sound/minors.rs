//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/minors.h
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
// MINOR numbers
//
pub const SNDRV_OS_MINORS: c_int = 256;
pub const SNDRV_MINOR_DEVICES: c_int = 32;

// these minors can still be used for autoloading devices (/dev/aload*)

// same as first respective minor number to make minor allocation easier

pub const SNDRV_MINOR_HWDEPS: c_int = 4;
pub const SNDRV_MINOR_RAWMIDIS: c_int = 8;
pub const SNDRV_MINOR_PCMS: c_int = 8;

pub const SNDRV_MINOR_OSS_DEVICES: c_int = 16;

pub const SNDRV_OSS_DEVICE_TYPE_MIXER: c_int = 0;
pub const SNDRV_OSS_DEVICE_TYPE_SEQUENCER: c_int = 1;
pub const SNDRV_OSS_DEVICE_TYPE_PCM: c_int = 2;
pub const SNDRV_OSS_DEVICE_TYPE_MIDI: c_int = 3;
pub const SNDRV_OSS_DEVICE_TYPE_DMFM: c_int = 4;
pub const SNDRV_OSS_DEVICE_TYPE_SNDSTAT: c_int = 5;
pub const SNDRV_OSS_DEVICE_TYPE_MUSIC: c_int = 6;

