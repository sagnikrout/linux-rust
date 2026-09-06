//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/jack.h
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
// Jack abstraction layer
//
// Copyright 2008 Wolfson Microelectronics plc
//

//
// enum snd_jack_types - Jack types which can be reported
// @SND_JACK_HEADPHONE: Headphone
// @SND_JACK_MICROPHONE: Microphone
// @SND_JACK_HEADSET: Headset
// @SND_JACK_LINEOUT: Line out
// @SND_JACK_MECHANICAL: Mechanical switch
// @SND_JACK_VIDEOOUT: Video out
// @SND_JACK_AVOUT: AV (Audio Video) out
// @SND_JACK_LINEIN:  Line in
// @SND_JACK_USB: USB audio device
// @SND_JACK_BTN_0: Button 0
// @SND_JACK_BTN_1: Button 1
// @SND_JACK_BTN_2: Button 2
// @SND_JACK_BTN_3: Button 3
// @SND_JACK_BTN_4: Button 4
// @SND_JACK_BTN_5: Button 5
//
// These values are used as a bitmask.
//
// Note that this must be kept in sync with the lookup table in
// sound/core/jack.c.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum snd_jack_types {
    SND_JACK_HEADPHONE	= 0x0001,
    SND_JACK_MICROPHONE	= 0x0002,
    SND_JACK_HEADSET	= SND_JACK_HEADPHONE | SND_JACK_MICROPHONE,
    SND_JACK_LINEOUT	= 0x0004,
    SND_JACK_MECHANICAL	= 0x0008, /* If detected separately */
    SND_JACK_VIDEOOUT	= 0x0010,
    SND_JACK_AVOUT		= SND_JACK_LINEOUT | SND_JACK_VIDEOOUT,
    SND_JACK_LINEIN		= 0x0020,
    SND_JACK_USB		= 0x0040,

// Kept separate from switches to facilitate implementation
    SND_JACK_BTN_0		= 0x4000,
    SND_JACK_BTN_1		= 0x2000,
    SND_JACK_BTN_2		= 0x1000,
    SND_JACK_BTN_3		= 0x0800,
    SND_JACK_BTN_4		= 0x0400,
    SND_JACK_BTN_5		= 0x0200,
}

// Keep in sync with definitions above
pub const SND_JACK_SWITCH_TYPES: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_jack {
    pub kctl_list: list_head,
    pub card: *mut snd_card,
    pub id: *const c_char,

    pub input_dev: *mut input_dev,
    pub input_dev_lock: mutex,
    pub registered: c_int,
    pub type: c_int,
    pub name: [c_char; 100],
    pub /: *mut *mut unsigned int key[6]; / Keep in sync with definitions above,

    pub hw_status_cache: c_int,
    pub private_data: *mut c_void,
    pub ): *mut *mut void (private_free)(struct snd_jack,
}

extern "C" {
    pub fn snd_jack_add_new_kctl(jack: *mut snd_jack, name: *const *const c_char, mask: c_int) -> c_int;
}

extern "C" {
    pub fn snd_jack_report(jack: *mut snd_jack, status: c_int);
}

