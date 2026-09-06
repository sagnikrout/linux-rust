//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_guc_regs.h
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
// Copyright © 2022 Intel Corporation
//

// Definitions of GuC H/W registers, bits, etc

pub const SOFT_SCRATCH_COUNT: c_int = 16;

pub const UOS_RSA_SCRATCH_COUNT: c_int = 64;

pub const GUC_WOPCM_OFFSET_SHIFT: c_int = 14;

pub const VF_SW_FLAG_COUNT: c_int = 4;

pub const MED_VF_SW_FLAG_COUNT: c_int = 4;

// GuC Interrupt Vector

pub const GUC_NUM_DOORBELLS: c_int = 256;
// format of the HW-monitored doorbell cacheline
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_doorbell_info {
    pub db_status: u32,
pub const GUC_DOORBELL_DISABLED: c_int = 0;
pub const GUC_DOORBELL_ENABLED: c_int = 1;
    pub cookie: u32,
    pub reserved: [u32; 14],
    pub __packed: },
