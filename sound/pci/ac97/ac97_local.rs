//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ac97/ac97_local.h
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
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Universal interface for Audio Codec '97
//
// For more details look to AC '97 component specification revision 2.2
// by Intel Corporation (http://developer.intel.com).
//
// ac97_proc.c

extern "C" {
    pub fn snd_ac97_bus_proc_init(ac97: *mut *mut snd_ac97_bus);
}
extern "C" {
    pub fn snd_ac97_bus_proc_done(ac97: *mut *mut snd_ac97_bus);
}
extern "C" {
    pub fn snd_ac97_proc_init(ac97: *mut *mut snd_ac97);
}
extern "C" {
    pub fn snd_ac97_proc_done(ac97: *mut *mut snd_ac97);
}

