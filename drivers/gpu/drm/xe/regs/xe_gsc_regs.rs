//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_gsc_regs.h
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

// Definitions of GSC H/W registers, bits, etc
pub const BMG_GSC_HECI1_BASE: c_uint = 0x373000;
pub const MTL_GSC_HECI1_BASE: c_uint = 0x00116000;
pub const MTL_GSC_HECI2_BASE: c_uint = 0x00117000;
pub const DG1_GSC_HECI2_BASE: c_uint = 0x00259000;
pub const PVC_GSC_HECI2_BASE: c_uint = 0x00285000;
pub const DG2_GSC_HECI2_BASE: c_uint = 0x00374000;

//
// The FWSTS register values are FW defined and can be different between
// HECI1 and HECI2
//

pub const HECI1_FWSTS1_CURRENT_STATE_RESET: c_int = 0;
pub const HECI1_FWSTS1_PROXY_STATE_NORMAL: c_int = 5;

pub const GSCI_TIMER_STATUS_RESET_IN_PROGRESS: c_int = 0;
pub const GSCI_TIMER_STATUS_TIMER_EXPIRED: c_int = 1;
pub const GSCI_TIMER_STATUS_RESET_COMPLETE: c_int = 2;
pub const GSCI_TIMER_STATUS_OUT_OF_RESET: c_int = 3;
