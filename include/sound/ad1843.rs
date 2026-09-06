//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/ad1843.h
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


//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright 2003 Vivien Chappelier <vivien.chappelier@linux-mips.org>
// Copyright 2008 Thomas Bogendoerfer <tsbogend@franken.de>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ad1843 {
    pub chip: *mut c_void,
    pub reg): *mut *mut *mut int (read)(void chip, int,
    pub val): *mut *mut *mut int (write)(void chip, int reg, int,
}

pub const AD1843_GAIN_RECLEV: c_int = 0;
pub const AD1843_GAIN_LINE: c_int = 1;
pub const AD1843_GAIN_LINE_2: c_int = 2;
pub const AD1843_GAIN_MIC: c_int = 3;
pub const AD1843_GAIN_PCM_0: c_int = 4;
pub const AD1843_GAIN_PCM_1: c_int = 5;

extern "C" {
    pub fn ad1843_get_gain_max(ad1843: *mut snd_ad1843, id: c_int) -> c_int;
}
extern "C" {
    pub fn ad1843_get_gain(ad1843: *mut snd_ad1843, id: c_int) -> c_int;
}
extern "C" {
    pub fn ad1843_set_gain(ad1843: *mut snd_ad1843, id: c_int, newval: c_int) -> c_int;
}
extern "C" {
    pub fn ad1843_get_recsrc(ad1843: *mut snd_ad1843) -> c_int;
}
extern "C" {
    pub fn ad1843_set_recsrc(ad1843: *mut snd_ad1843, newsrc: c_int) -> c_int;
}
extern "C" {
    pub fn ad1843_shutdown_adc(ad1843: *mut snd_ad1843);
}
extern "C" {
    pub fn ad1843_init(ad1843: *mut snd_ad1843) -> c_int;
}
