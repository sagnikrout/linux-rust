//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/twl6040.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ALSA SoC TWL6040 codec driver
//
// Author:	Misael Lopez Cruz <x0052729@ti.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum twl6040_trim {
    TWL6040_TRIM_TRIM1 = 0,
    TWL6040_TRIM_TRIM2,
    TWL6040_TRIM_TRIM3,
    TWL6040_TRIM_HSOTRIM,
    TWL6040_TRIM_HFOTRIM,
    TWL6040_TRIM_INVAL,
}

extern "C" {
    pub fn twl6040_get_dl1_gain(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn twl6040_get_clk_id(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn twl6040_get_trim_value(component: *mut snd_soc_component, trim: twl6040_trim) -> c_int;
}
extern "C" {
    pub fn twl6040_get_hs_step_size(component: *mut snd_soc_component) -> c_int;
}
