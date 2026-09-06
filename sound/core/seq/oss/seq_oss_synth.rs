//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/oss/seq_oss_synth.h
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
// OSS compatible sequencer driver
//
// synth device information
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

extern "C" {
    pub fn snd_seq_oss_synth_init();
}
extern "C" {
    pub fn snd_seq_oss_synth_probe(dev: *mut snd_seq_device) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_synth_remove(dev: *mut snd_seq_device);
}
extern "C" {
    pub fn snd_seq_oss_synth_setup(dp: *mut seq_oss_devinfo);
}
extern "C" {
    pub fn snd_seq_oss_synth_setup_midi(dp: *mut seq_oss_devinfo);
}
extern "C" {
    pub fn snd_seq_oss_synth_cleanup(dp: *mut seq_oss_devinfo);
}
extern "C" {
    pub fn snd_seq_oss_synth_reset(dp: *mut seq_oss_devinfo, dev: c_int);
}
extern "C" {
    pub fn snd_seq_oss_synth_addr(dp: *mut seq_oss_devinfo, dev: c_int, ev: *mut snd_seq_event) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_synth_make_info(dp: *mut seq_oss_devinfo, dev: c_int, inf: *mut synth_info) -> c_int;
}
