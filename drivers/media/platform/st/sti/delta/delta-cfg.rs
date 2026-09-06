//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/delta/delta-cfg.h
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
//
// Copyright (C) STMicroelectronics SA 2015
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//

pub const DELTA_MIN_WIDTH: c_int = 32;
pub const DELTA_MAX_WIDTH: c_int = 4096;
pub const DELTA_MIN_HEIGHT: c_int = 32;
pub const DELTA_MAX_HEIGHT: c_int = 2400;
// DELTA requires a 32x32 pixels alignment for frames
pub const DELTA_WIDTH_ALIGNMENT: c_int = 32;
pub const DELTA_HEIGHT_ALIGNMENT: c_int = 32;

// guard value for number of access units
pub const DELTA_MAX_AUS: c_int = 10;
// IP perf dependent, can be tuned
pub const DELTA_PEAK_FRAME_SMOOTHING: c_int = 2;
//
// guard output frame count:
// - at least 1 frame needed for display
// - at worst 21
// ( max h264 dpb (16) +
// decoding peak smoothing (2) +
// user display pipeline (3) )
//
pub const DELTA_MIN_FRAME_USER: c_int = 1;
pub const DELTA_MAX_DPB: c_int = 16;

// extra space to be allocated to store codec specific data per frame
pub const DELTA_MAX_FRAME_PRIV_SIZE: c_int = 100;
// PM runtime auto power-off after 5ms of inactivity
pub const DELTA_HW_AUTOSUSPEND_DELAY_MS: c_int = 5;
pub const DELTA_MAX_DECODERS: c_int = 10;

