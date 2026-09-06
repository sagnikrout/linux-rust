//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/intel/avs/control.h
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
// Copyright(c) 2021-2022 Intel Corporation
//
// Authors: Amadeusz Slawinski <amadeuszx.slawinski@linux.intel.com>
// Cezary Rojewski <cezary.rojewski@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avs_control_data {
    pub id: u32,
    pub values: [c_long; SND_SOC_TPLG_MAX_CHAN],
}

extern "C" {
    pub fn avs_control_volume_get(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn avs_control_volume_put(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn avs_control_volume_info(kctl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
}
extern "C" {
    pub fn avs_control_mute_get(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn avs_control_mute_put(kctl: *mut snd_kcontrol, uctl: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn avs_control_mute_info(kctl: *mut snd_kcontrol, uinfo: *mut snd_ctl_elem_info) -> c_int;
}
