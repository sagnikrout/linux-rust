//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/asound_fm.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Advanced Linux Sound Architecture - ALSA
//
// Interface file between ALSA driver & user space
// Copyright (c) 1994-98 by Jaroslav Kysela <perex@perex.cz>,
// 4Front Technologies
//
// Direct FM control
//
pub const SNDRV_DM_FM_MODE_OPL2: c_uint = 0x00;
pub const SNDRV_DM_FM_MODE_OPL3: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dm_fm_info {
    pub /: *mut *mut unsigned char fm_mode; / OPL mode, see SNDRV_DM_FM_MODE_XXX,
    pub /: *mut *mut unsigned char rhythm; / percussion mode flag,
}

//
// Data structure composing an FM "note" or sound event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dm_fm_voice {
    pub /: *mut *mut unsigned char op; / operator cell (0 or 1),
    pub /: *mut *mut unsigned char voice; / FM voice (0 to 17),
    pub /: *mut *mut unsigned char am; / amplitude modulation,
    pub /: *mut *mut unsigned char vibrato; / vibrato effect,
    pub /: *mut *mut unsigned char do_sustain; / sustain phase,
    pub /: *mut *mut unsigned char kbd_scale; / keyboard scaling,
    pub /: *mut *mut unsigned char harmonic; / 4 bits: harmonic and multiplier,
    pub /: *mut *mut unsigned char scale_level; / 2 bits: decrease output freq rises,
    pub /: *mut *mut unsigned char volume; / 6 bits: volume,
    pub /: *mut *mut unsigned char attack; / 4 bits: attack rate,
    pub /: *mut *mut unsigned char decay; / 4 bits: decay rate,
    pub /: *mut *mut unsigned char sustain; / 4 bits: sustain level,
    pub /: *mut *mut unsigned char release; / 4 bits: release rate,
    pub /: *mut *mut unsigned char feedback; / 3 bits: feedback for op0,
    pub /: *mut *mut unsigned char connection; / 0 for serial, 1 for parallel,
    pub /: *mut *mut unsigned char left; / stereo left,
    pub /: *mut *mut unsigned char right; / stereo right,
    pub /: *mut *mut unsigned char waveform; / 3 bits: waveform shape,
}

//
// This describes an FM note by its voice, octave, frequency number (10bit)
// and key on/off.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dm_fm_note {
    pub /: *mut *mut unsigned char voice; / 0-17 voice channel,
    pub /: *mut *mut unsigned char octave; / 3 bits: what octave to play,
    pub /: *mut *mut unsigned int fnum; / 10 bits: frequency number,
    pub /: *mut *mut unsigned char key_on; / set for active, clear for silent,
}

//
// FM parameters that apply globally to all voices, and thus are not "notes"
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dm_fm_params {
    pub /: *mut *mut unsigned char am_depth; / amplitude modulation depth (1=hi),
    pub /: *mut *mut unsigned char vib_depth; / vibrato depth (1=hi),
    pub /: *mut *mut unsigned char kbd_split; / keyboard split,
    pub /: *mut *mut unsigned char rhythm; / percussion mode select,
// This block is the percussion instrument data
    pub bass: c_uchar,
    pub snare: c_uchar,
    pub tomtom: c_uchar,
    pub cymbal: c_uchar,
    pub hihat: c_uchar,
}

//
// FM mode ioctl settings
//

// for OPL3 only

// SBI patch management

pub const SNDRV_DM_FM_OSS_IOCTL_RESET: c_uint = 0x20;
pub const SNDRV_DM_FM_OSS_IOCTL_PLAY_NOTE: c_uint = 0x21;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_VOICE: c_uint = 0x22;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_PARAMS: c_uint = 0x23;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_MODE: c_uint = 0x24;
pub const SNDRV_DM_FM_OSS_IOCTL_SET_OPL: c_uint = 0x25;
//
// Patch Record - fixed size for write
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbi_patch {
    pub prog: c_uchar,
    pub bank: c_uchar,
    pub key: [c_char; 4],
    pub name: [c_char; 25],
    pub extension: [c_char; 7],
    pub data: [c_uchar; 32],
}
