//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soundfont.h
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
// Soundfont defines and definitions.
//
// Copyright (C) 1999 Steve Ratcliffe
// Copyright (c) 1999-2000 Takashi iwai <tiwai@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sf_zone {
    pub /: *mut *mut *mut snd_sf_zone next; / Link to next,
    pub /: *mut *mut unsigned char bank; / Midi bank for this zone,
    pub /: *mut *mut unsigned char instr; / Midi program for this zone,
    pub /: *mut *mut unsigned char mapped; / True if mapped to something else,
    pub /: *mut *mut soundfont_voice_info v; / All the soundfont parameters,
    pub counter: c_int,
    pub /: *mut *mut *mut snd_sf_sample sample; / Link to sample,
// The following deals with preset numbers (programs)
    pub /: *mut *mut *mut snd_sf_zone next_instr; / Next zone of this instrument,
    pub /: *mut *mut *mut snd_sf_zone next_zone; / Next zone in play list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sf_sample {
    pub v: soundfont_sample_info,
    pub counter: c_int,
    pub /: *mut *mut *mut snd_util_memblk block; / allocated data block,
    pub next: *mut snd_sf_sample,
}

//
// This represents all the information relating to a soundfont.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soundfont {
    pub /: *mut *mut *mut snd_soundfont next; / Link to next,
// struct snd_soundfont *prev;*/	/* Link to previous
    pub /: *mut *mut short id; / file id,
    pub /: *mut *mut short type; / font type,
    pub /: *mut *mut unsigned char name[SNDRV_SFNT_PATCH_NAME_LEN]; / identifier,
    pub /: *mut *mut *mut snd_sf_zone zones; / Font information,
    pub /: *mut *mut *mut snd_sf_sample samples; / The sample headers,
}

//
// Type of the sample access callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sf_callback {
    pub private_data: *mut c_void,
    pub count): *const *const void __user buf, long,
    pub hdr): *mut snd_util_memhdr,
    pub private): *mut *mut void (sample_reset)(void,
}

//
// List of soundfonts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sf_list {
    pub /: *mut *mut *mut snd_soundfont currsf; / The currently open soundfont,
    pub /: *mut *mut int open_client; / client pointer for lock,
    pub /: *mut *mut int mem_used; / used memory size,
    pub presets: [*mut snd_sf_zone; SF_MAX_PRESETS],
    pub /: *mut *mut *mut snd_soundfont fonts; / The list of soundfonts,
    pub /: *mut *mut int fonts_size; / number of fonts allocated,
    pub /: *mut *mut int zone_counter; / last allocated time for zone,
    pub /: *mut *mut int sample_counter; / last allocated time for sample,
    pub /: *mut *mut int zone_locked; / locked time for zone,
    pub /: *mut *mut int sample_locked; / locked time for sample,
    pub /: *mut *mut snd_sf_callback callback; / callback functions,
    pub presets_locked: c_int,
    pub presets_mutex: mutex,
    pub lock: spinlock_t,
    pub memhdr: *mut snd_util_memhdr,
}

// Prototypes for soundfont.c
extern "C" {
    pub fn snd_soundfont_close_check(sflist: *mut snd_sf_list, client: c_int) -> c_int;
}
extern "C" {
    pub fn snd_sf_free(sflist: *mut snd_sf_list);
}
extern "C" {
    pub fn snd_soundfont_remove_samples(sflist: *mut snd_sf_list) -> c_int;
}
extern "C" {
    pub fn snd_soundfont_remove_unlocked(sflist: *mut snd_sf_list) -> c_int;
}
// Parameter conversions
extern "C" {
    pub fn snd_sf_calc_parm_hold(msec: c_int) -> c_int;
}
extern "C" {
    pub fn snd_sf_calc_parm_attack(msec: c_int) -> c_int;
}
extern "C" {
    pub fn snd_sf_calc_parm_decay(msec: c_int) -> c_int;
}

extern "C" {
    pub fn snd_sf_linear_to_log(amount: c_uint, offset: c_int, ratio: c_int) -> c_int;
}
// lock access to sflist
// remove lock
