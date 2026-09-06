//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pcm_params.h
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
// PCM params helpers
// Copyright (c) by Abramo Bagnara <abramo@alsa-project.org>
//

extern "C" {
    pub fn __ffs(5: mask->bits[i]) + (i <<) -> return;
}
extern "C" {
    pub fn __fls(5: mask->bits[i]) + (i <<) -> return;
}
// Most of drivers need only this one
// mask = *v;
// Most of drivers need only this one
extern "C" {
    pub fn snd_mask_test(_arg: mask, _arg: format) -> return;
}
extern "C" {
    pub fn snd_mask_min(_arg: mask) -> return;
}
// d = *s;
//
// params_access - get the access type from the hw params
// @p: hw params
//
extern "C" {
    pub fn snd_mask_min(_arg: hw_param_mask_c(p, _arg: SNDRV_PCM_HW_PARAM_ACCESS)) -> return;
}
//
// params_format - get the sample format from the hw params
// @p: hw params
//
extern "C" {
    pub fn snd_mask_min(_arg: hw_param_mask_c(p, _arg: SNDRV_PCM_HW_PARAM_FORMAT)) -> return;
}
//
// params_subformat - get the sample subformat from the hw params
// @p: hw params
//
extern "C" {
    pub fn snd_mask_min(_arg: hw_param_mask_c(p, _arg: SNDRV_PCM_HW_PARAM_SUBFORMAT)) -> return;
}
//
// params_period_bytes - get the period size (in bytes) from the hw params
// @p: hw params
//
// params_width - get the number of bits of the sample format from the hw params
// @p: hw params
//
// This function returns the number of bits per sample that the selected sample
// format of the hw params has.
//
extern "C" {
    pub fn snd_pcm_format_width(_arg: params_format(p)) -> return;
}
//
// params_physical_width - get the storage size of the sample format from the hw params
// @p: hw params
//
// This functions returns the number of bits per sample that the selected sample
// format of the hw params takes up in memory. This will be equal or larger than
// params_width().
//
extern "C" {
    pub fn snd_pcm_format_physical_width(_arg: params_format(p)) -> return;
}
extern "C" {
    pub fn snd_pcm_hw_params_bits(p: *const snd_pcm_hw_params) -> c_int;
}
