//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/scarlett2.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Focusrite Scarlett 2 Protocol Driver for ALSA
// (including Scarlett 2nd Gen, 3rd Gen, 4th Gen, Clarett USB, and
// Clarett+ series products)
//
// Copyright (c) 2023 by Geoffrey D. Bennett <g at b4.vu>
//

pub const SCARLETT2_HWDEP_MAJOR: c_int = 1;
pub const SCARLETT2_HWDEP_MINOR: c_int = 0;
pub const SCARLETT2_HWDEP_SUBMINOR: c_int = 0;

// Get protocol version

// Reboot

// Select flash segment
pub const SCARLETT2_SEGMENT_ID_SETTINGS: c_int = 0;
pub const SCARLETT2_SEGMENT_ID_FIRMWARE: c_int = 1;
pub const SCARLETT2_SEGMENT_ID_COUNT: c_int = 2;

// Erase selected flash segment

// Get selected flash segment erase progress
// 1 through to num_blocks, or 255 for complete
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scarlett2_flash_segment_erase_progress {
    pub progress: c_uchar,
    pub num_blocks: c_uchar,
}

