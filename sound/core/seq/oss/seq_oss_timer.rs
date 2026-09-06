//! Automatically rewritten from C Header to Rust Module
//! Source: sound/core/seq/oss/seq_oss_timer.h
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
// timer handling routines
//
// Copyright (C) 1998,99 Takashi Iwai <tiwai@suse.de>
//

//
// timer information definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_oss_timer {
    pub dp: *mut seq_oss_devinfo,
    pub cur_tick: reltime_t,
    pub realtime: c_int,
    pub running: c_int,
    pub /: *mut *mut int tempo, ppq; / ALSA queue,
    pub oss_timebase: int oss_tempo,,
}

extern "C" {
    pub fn snd_seq_oss_timer_delete(dp: *mut seq_oss_timer);
}
extern "C" {
    pub fn snd_seq_oss_timer_start(timer: *mut seq_oss_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_timer_stop(timer: *mut seq_oss_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_timer_continue(timer: *mut seq_oss_timer) -> c_int;
}
extern "C" {
    pub fn snd_seq_oss_timer_tempo(timer: *mut seq_oss_timer, value: c_int) -> c_int;
}

extern "C" {
    pub fn snd_seq_oss_timer_ioctl(timer: *mut seq_oss_timer, cmd: c_uint, arg: *mut int __user) -> c_int;
}
//
// get current processed time
//
