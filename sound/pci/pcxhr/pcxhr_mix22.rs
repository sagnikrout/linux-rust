//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/pcxhr/pcxhr_mix22.h
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
// Driver for Digigram pcxhr compatible soundcards
//
// low level interface with interrupt ans message handling
//
// Copyright (c) 2004 by Digigram <alsa@digigram.com>
//
extern "C" {
    pub fn hr222_sub_init(mgr: *mut pcxhr_mgr) -> c_int;
}
extern "C" {
    pub fn hr222_read_gpio(mgr: *mut pcxhr_mgr, is_gpi: c_int, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn hr222_write_gpo(mgr: *mut pcxhr_mgr, value: c_int) -> c_int;
}
extern "C" {
    pub fn hr222_manage_timecode(mgr: *mut pcxhr_mgr, enable: c_int) -> c_int;
}

extern "C" {
    pub fn hr222_set_audio_source(chip: *mut snd_pcxhr) -> c_int;
}
extern "C" {
    pub fn hr222_add_mic_controls(chip: *mut snd_pcxhr) -> c_int;
}
