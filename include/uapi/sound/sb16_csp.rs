//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/sb16_csp.h
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
// Copyright (c) 1999 by Uros Bizjak <uros@kss-loka.si>
// Takashi Iwai <tiwai@suse.de>
//
// SB16ASP/AWE32 CSP control
//
// CSP modes
pub const SNDRV_SB_CSP_MODE_NONE: c_uint = 0x00;
pub const SNDRV_SB_CSP_MODE_DSP_READ: c_uint = 0x01	/* Record from DSP */;
pub const SNDRV_SB_CSP_MODE_DSP_WRITE: c_uint = 0x02	/* Play to DSP */;
pub const SNDRV_SB_CSP_MODE_QSOUND: c_uint = 0x04	/* QSound */;
// CSP load flags
pub const SNDRV_SB_CSP_LOAD_FROMUSER: c_uint = 0x01;
pub const SNDRV_SB_CSP_LOAD_INITBLOCK: c_uint = 0x02;
// CSP sample width
pub const SNDRV_SB_CSP_SAMPLE_8BIT: c_uint = 0x01;
pub const SNDRV_SB_CSP_SAMPLE_16BIT: c_uint = 0x02;
// CSP channels
pub const SNDRV_SB_CSP_MONO: c_uint = 0x01;
pub const SNDRV_SB_CSP_STEREO: c_uint = 0x02;
// CSP rates
pub const SNDRV_SB_CSP_RATE_8000: c_uint = 0x01;
pub const SNDRV_SB_CSP_RATE_11025: c_uint = 0x02;
pub const SNDRV_SB_CSP_RATE_22050: c_uint = 0x04;
pub const SNDRV_SB_CSP_RATE_44100: c_uint = 0x08;
pub const SNDRV_SB_CSP_RATE_ALL: c_uint = 0x0f;
// CSP running state
pub const SNDRV_SB_CSP_ST_IDLE: c_uint = 0x00;
pub const SNDRV_SB_CSP_ST_LOADED: c_uint = 0x01;
pub const SNDRV_SB_CSP_ST_RUNNING: c_uint = 0x02;
pub const SNDRV_SB_CSP_ST_PAUSED: c_uint = 0x04;
pub const SNDRV_SB_CSP_ST_AUTO: c_uint = 0x08;
pub const SNDRV_SB_CSP_ST_QSOUND: c_uint = 0x10;
// maximum QSound value (180 degrees right)
pub const SNDRV_SB_CSP_QSOUND_MAX_RIGHT: c_uint = 0x20;
// maximum microcode RIFF file size
pub const SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE: c_uint = 0x3000;
// microcode header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp_mc_header {
    pub /: *mut *mut char codec_name[16]; / id name of codec,
    pub /: *mut *mut unsigned short func_req; / requested function,
}

// microcode to be loaded
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp_microcode {
    pub info: snd_sb_csp_mc_header,
    pub data: [c_uchar; SNDRV_SB_CSP_MAX_MICROCODE_FILE_SIZE],
}

// start CSP with sample_width in mono/stereo
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp_start {
    pub /: *mut *mut int sample_width; / sample width, look above,
    pub /: *mut *mut int channels; / channels, look above,
}

// CSP information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_sb_csp_info {
    pub /: *mut *mut char codec_name[16]; / id name of codec,
    pub /: *mut *mut unsigned short func_nr; / function number,
    pub /: *mut *mut unsigned int acc_format; / accepted PCM formats,
    pub /: *mut *mut unsigned short acc_channels; / accepted channels,
    pub /: *mut *mut unsigned short acc_width; / accepted sample width,
    pub /: *mut *mut unsigned short acc_rates; / accepted sample rates,
    pub /: *mut *mut unsigned short csp_mode; / CSP mode, see above,
    pub /: *mut *mut unsigned short run_channels; / current channels,
    pub /: *mut *mut unsigned short run_width; / current sample width,
    pub /: *mut *mut unsigned short version; / version id: 0x10 - 0x1f,
    pub /: *mut *mut unsigned short state; / state bits,
}

// HWDEP controls
// get CSP information

// load microcode to CSP
// NOTE: struct snd_sb_csp_microcode overflows the max size (13 bits)
// defined for some architectures like MIPS, and it leads to build errors.
// (x86 and co have 14-bit size, thus it's valid, though.)
// As a workaround for skipping the size-limit check, here we don't use the
// normal _IOW() macro but _IOC() with the manual argument.
//

// unload microcode from CSP

// start CSP

// stop CSP

// pause CSP and DMA transfer

// restart CSP and DMA transfer

