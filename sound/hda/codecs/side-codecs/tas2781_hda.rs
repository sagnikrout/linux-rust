//! Automatically rewritten from C Header to Rust Module
//! Source: sound/hda/codecs/side-codecs/tas2781_hda.h
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
// HDA audio driver for Texas Instruments TAS2781 smart amp
//
// Copyright (C) 2025 Texas Instruments, Inc.
//

// Flag of calibration registers address.

pub const TASDEV_CALIB_N: c_int = 5;
//
// No standard control callbacks for SNDRV_CTL_ELEM_IFACE_CARD
// Define two controls, one is Volume control callbacks, the other is
// flag setting control callbacks.
//
// Volume control callbacks for tas2781

// Flag control callbacks for tas2781

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_catlog_id {
    DELL = 0,
    HP,
    LENOVO,
    OTHERS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tas2781_hda {
    pub dev: *mut device,
    pub priv: *mut tasdevice_priv,
    pub dsp_prog_ctl: *mut snd_kcontrol,
    pub dsp_conf_ctl: *mut snd_kcontrol,
    pub prof_ctl: *mut snd_kcontrol,
    pub catlog_id: device_catlog_id,
    pub hda_priv: *mut c_void,
}

extern "C" {
    pub fn tas2781_save_calibration(p: *mut tas2781_hda) -> c_int;
}
