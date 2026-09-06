//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/mixart/mixart_mixer.h
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
// Driver for Digigram miXart soundcards
//
// include file for mixer
//
// Copyright (c) 2003 by Digigram <alsa@digigram.com>
//
// exported
extern "C" {
    pub fn mixart_update_playback_stream_level(chip: *mut *mut snd_mixart, is_aes: c_int, idx: c_int) -> c_int;
}
extern "C" {
    pub fn mixart_update_capture_stream_level(chip: *mut *mut snd_mixart, is_aes: c_int) -> c_int;
}
extern "C" {
    pub fn snd_mixart_create_mixer(mgr: *mut *mut mixart_mgr) -> c_int;
}
