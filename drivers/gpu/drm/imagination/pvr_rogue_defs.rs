//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_defs.h
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
// ROGUE Defines
//

// SOFT_RESET Rascal and DUSTs bits

// SOFT_RESET steps as defined in the TRM

//
// To get the number of required Dusts, divide the number of
// clusters by 2 and round up
//

//
// To get the number of required Bernado/Phantom(s), divide
// the number of clusters by 4 and round up
//

//
// FW MMU contexts
//

//
// Utility macros to calculate CAT_BASE register addresses
//

//
// FWCORE wrapper register defines
//

pub const ROGUE_MAX_VERTEX_SHARED_REGISTERS: c_int = 1024;
pub const ROGUE_MAX_PIXEL_SHARED_REGISTERS: c_int = 1024;

pub const ROGUE_CDMCTRL_USC_COMMON_SIZE_ALIGNSIZE: c_int = 64;
pub const ROGUE_CDMCTRL_USC_COMMON_SIZE_UPPER: c_int = 256;
//
// The maximum amount of local memory which can be allocated by a single kernel
// (in dwords/32-bit registers).
//
// ROGUE_CDMCTRL_USC_COMMON_SIZE_ALIGNSIZE is in bytes so we divide by four.
//

//
// WA HWBRNs
//
// GPU CR timer tick in GPU cycles

// for nohw multicore return max cores possible to client

//
// If the size of the SLC is less than this value then the TPU bypasses the SLC.
//

//
// If the size of the SLC is bigger than this value then the TCU must not be
// bypassed in the SLC.
// In XE_MEMORY_HIERARCHY cores, the TCU is bypassed by default.
//

//
// Register used by the FW to track the current boot stage (not used in MIPS)
//

//
// Virtualisation definitions
//

//
// Macro used to indicate which version of HWPerf is active
//
// Macro flag: #define ROGUE_FEATURE_HWPERF_ROGUE
//
// Maximum number of cores supported by TRP
//

