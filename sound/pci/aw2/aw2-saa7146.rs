//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/aw2/aw2-saa7146.h
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
// Copyright (C) 2008 Cedric Bregardis <cedric.bregardis@free.fr> and
// Jean-Christian Hassler <jhassler@free.fr>
//
// This file is part of the Audiowerk2 ALSA driver
//
pub const NB_STREAM_PLAYBACK: c_int = 2;
pub const NB_STREAM_CAPTURE: c_int = 1;
pub const NUM_STREAM_PLAYBACK_ANA: c_int = 0;
pub const NUM_STREAM_PLAYBACK_DIG: c_int = 1;
pub const NUM_STREAM_CAPTURE_ANA: c_int = 0;
extern "C" {
    pub fn void(: *mut *mut snd_aw2_saa7146_it_cb) (struct snd_pcm_substream) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aw2_saa7146_cb_param {
    pub p_it_callback: snd_aw2_saa7146_it_cb,
    pub p_callback_param: *mut snd_pcm_substream,
}

// definition of the chip-specific record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aw2_saa7146 {
    pub base_addr: *mut void __iomem,
}

extern "C" {
    pub fn snd_aw2_saa7146_free(chip: *mut snd_aw2_saa7146) -> c_int;
}
// chip, int stream_number);
// chip,
// chip, int stream_number);
extern "C" {
    pub fn snd_aw2_saa7146_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
// chip,
// start_addr,
// chip,
// start_addr,
// chip);
