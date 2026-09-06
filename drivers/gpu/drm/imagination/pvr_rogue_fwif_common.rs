//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_fwif_common.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

//
// This macro represents a mask of LSBs that must be zero on data structure
// sizes and offsets to ensure they are 8-byte granular on types shared between
// the FW and host driver.
//

// Macro to test structure size alignment.

// The master definition for data masters known to the firmware.

// Either TDM or 2D DM is present.
// When the 'tla' feature is present in the hw (as per @pvr_device_features).

//
// When the 'fastrender_dm' feature is present in the hw (as per
// @pvr_device_features).
//

// Maximum number of DM in use: GP, 2D/TDM, GEOM, 3D, CDM, RAY, GEOM2, GEOM3, GEOM4

// GPU Utilisation states

pub const PVR_FWIF_GPU_UTIL_STATE_MASK: c_uint = 0x3ULL;
//
// Maximum amount of register writes that can be done by the register
// programmer (FW or META DMA). This is not a HW limitation, it is only
// a protection against malformed inputs to the register programmer.
//

