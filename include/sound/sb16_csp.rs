//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sb16_csp.h
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
// Copyright (c) 1999 by Uros Bizjak <uros@kss-loka.si>
// Takashi Iwai <tiwai@suse.de>
//
// SB16ASP/AWE32 CSP control
//

// indices for the known CSP programs
//
// CSP operators
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp_ops {
    pub p): *mut *mut *mut int (csp_use) (struct snd_sb_csp,
    pub p): *mut *mut *mut int (csp_unuse) (struct snd_sb_csp,
    pub play_rec_mode): *mut *mut *mut int (csp_autoload) (struct snd_sb_csp  p, snd_pcm_format_t pcm_sfmt, int,
    pub channels): *mut *mut *mut int (csp_start) (struct snd_sb_csp  p, int sample_width, int,
    pub p): *mut *mut *mut int (csp_stop) (struct snd_sb_csp,
    pub p): *mut *mut *mut int (csp_qsound_transfer) (struct snd_sb_csp,
}

//
// CSP private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp {
    pub /: *mut *mut *mut snd_sb chip; / SB16 DSP,
    pub /: *mut *mut int used; / usage flag - exclusive,
    pub /: *mut *mut char codec_name[16]; / name of codec,
    pub /: *mut *mut unsigned short func_nr; / function number,
    pub /: *mut *mut unsigned int acc_format; / accepted PCM formats,
    pub /: *mut *mut int acc_channels; / accepted channels,
    pub /: *mut *mut int acc_width; / accepted sample width,
    pub /: *mut *mut int acc_rates; / accepted sample rates,
    pub /: *mut *mut int mode; / MODE,
    pub /: *mut *mut int run_channels; / current CSP channels,
    pub /: *mut *mut int run_width; / current sample width,
    pub /: *mut *mut int version; / CSP version (0x10 - 0x1f),
    pub /: *mut *mut int running; / running state,
    pub /: *mut *mut snd_sb_csp_ops ops; / operators,
    pub /: *mut *mut spinlock_t q_lock; / locking,
    pub /: *mut *mut int q_enabled; / enabled flag,
    pub /: *mut *mut int qpos_left; / left position,
    pub /: *mut *mut int qpos_right; / right position,
    pub /: *mut *mut int qpos_changed; / position changed flag,
    pub qsound_switch: *mut snd_kcontrol,
    pub qsound_space: *mut snd_kcontrol,
    pub /: *mut *mut mutex access_mutex; / locking,
    pub csp_programs: [*const firmware; CSP_PROGRAM_COUNT],
}

extern "C" {
    pub fn snd_sb_csp_new(chip: *mut snd_sb, device: c_int, rhwdep: *mut *mut *mut snd_hwdep) -> c_int;
}
