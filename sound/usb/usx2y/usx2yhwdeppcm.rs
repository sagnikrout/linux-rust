//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/usx2y/usx2yhwdeppcm.h
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


// SPDX-License-Identifier: GPL-2.0
pub const MAXPACK: c_int = 50;
pub const MAXBUFFERMS: c_int = 100;
pub const MAXSTRIDE: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usx2y_hwdep_pcm_shm {
    pub playback: [c_char; SSS],
    pub capture0x8: [c_char; SSS],
    pub capture0xA: [c_char; SSS],
    pub playback_iso_head: volatile int,
    pub playback_iso_start: c_int,
    pub captured_iso: [}; 128],
    pub captured_iso_head: volatile int,
    pub captured_iso_frames: volatile unsigned,
    pub capture_iso_start: c_int,
}

extern "C" {
    pub fn usx2y_hwdep_pcm_new(card: *mut snd_card) -> c_int;
}
