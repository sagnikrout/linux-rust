//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/npcm-video.h
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
// Controls header for NPCM video driver
//
// Copyright (C) 2022 Nuvoton Technologies
//

//
// Check Documentation/userspace-api/media/drivers/npcm-video.rst for control
// details.
//
// This control is meant to set the mode of NPCM Video Capture/Differentiation
// (VCD) engine.
//
// The VCD engine supports two modes:
// COMPLETE - Capture the next complete frame into memory.
// DIFF	    - Compare the incoming frame with the frame stored in memory, and
// updates the differentiated frame in memory.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum v4l2_npcm_capture_mode {
    V4L2_NPCM_CAPTURE_MODE_COMPLETE	= 0, /* COMPLETE mode */
    V4L2_NPCM_CAPTURE_MODE_DIFF	= 1, /* DIFF mode */
}

//
// This control is meant to get the count of compressed HEXTILE rectangles which
// is relevant to the number of differentiated frames if VCD is in DIFF mode.
// And the count will always be 1 if VCD is in COMPLETE mode.
//

