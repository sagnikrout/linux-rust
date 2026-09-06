//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/instructions/xe_gsc_commands.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// All GSCCS-specific commands have fixed length, so we can include it in the
// defines. Note that the generic GSC command header structure includes an
// optional data field in bits 9-21, but there are no commands that actually use
// it; some of the commands are instead defined as having an extended length
// field spanning bits 0-15, even if the extra bits are not required because the
// longest GSCCS command is only 8 dwords. To handle this, the defines below use
// a single field for both data and len. If we ever get a commands that does
// actually have data and this approach doesn't work for it we can re-work it
// at that point.
//

