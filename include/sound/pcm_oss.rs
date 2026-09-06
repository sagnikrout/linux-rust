//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pcm_oss.h
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
// Digital Audio (PCM) - OSS compatibility abstract layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss_setup {
    pub task_name: *mut c_char,
    pub periods: c_uint,
    pub period_size: c_uint,
    pub next: *mut snd_pcm_oss_setup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss_runtime {
    pub /: *mut *mut sync_trigger: 1; / sync trigger flag,
    pub /: *mut *mut int rate; / requested rate,
    pub /: *mut *mut int format; / requested OSS format,
    pub /: *mut *mut unsigned int channels; / requested channels,
    pub fragshift: c_uint,
    pub maxfrags: c_uint,
    pub /: *mut *mut unsigned int subdivision; / requested subdivision,
    pub /: *mut *mut size_t period_bytes; / requested period size,
    pub /: *mut *mut size_t period_frames; / period frames for poll,
    pub /: *mut *mut size_t period_ptr; / actual write pointer to period,
    pub periods: c_uint,
    pub /: *mut *mut size_t buffer_bytes; / requested buffer size,
    pub /: *mut *mut size_t bytes; / total # bytes processed,
    pub mmap_bytes: usize,
    pub /: *mut *mut *mut char buffer; / vmallocated period,
    pub /: *mut *mut size_t buffer_used; / used length from period buffer,
    pub params_lock: mutex,
    pub /: *mut *mut atomic_t rw_ref; / concurrent read/write accesses,

    pub plugin_first: *mut snd_pcm_plugin,
    pub plugin_last: *mut snd_pcm_plugin,

    pub prev_hw_ptr_period: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss_file {
    pub streams: [*mut snd_pcm_substream; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss_substream {
    pub /: *mut *mut unsigned oss: 1; / oss mode,
    pub /: *mut *mut snd_pcm_oss_setup setup; / active setup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss_stream {
    pub /: *mut *mut *mut snd_pcm_oss_setup setup_list; / setup list,
    pub setup_mutex: mutex,

    pub proc_entry: *mut snd_info_entry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_oss {
    pub reg: c_int,
    pub reg_mask: c_uint,
}
