//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/audio.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// audio.h - DEPRECATED MPEG-TS audio decoder API
//
// NOTE: should not be used on future drivers
//
// Copyright (C) 2000 Ralph  Metzler <ralph@convergence.de>
// & Marcus Metzler <marcus@convergence.de>
// for convergence integrated media GmbH
//

// what else do we need? bass, pass-through, ...
// for GET_CAPABILITIES and SET_FORMAT, the latter should only set one bit
pub const AUDIO_CAP_DTS: c_int = 1;
pub const AUDIO_CAP_LPCM: c_int = 2;
pub const AUDIO_CAP_MP1: c_int = 4;
pub const AUDIO_CAP_MP2: c_int = 8;
pub const AUDIO_CAP_MP3: c_int = 16;
pub const AUDIO_CAP_AAC: c_int = 32;
pub const AUDIO_CAP_OGG: c_int = 64;
pub const AUDIO_CAP_SDDS: c_int = 128;
pub const AUDIO_CAP_AC3: c_int = 256;

