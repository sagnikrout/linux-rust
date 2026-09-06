//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_perf_oa_regs.h
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

pub const GEN7_OACONTROL_CTX_MASK: c_uint = 0xFFFFF000;
pub const GEN7_OACONTROL_TIMER_PERIOD_MASK: c_uint = 0x3F;
pub const GEN7_OACONTROL_TIMER_PERIOD_SHIFT: c_int = 6;

pub const GEN7_OACONTROL_FORMAT_SHIFT: c_int = 2;

pub const GEN8_OA_REPORT_FORMAT_SHIFT: c_int = 2;

pub const GEN8_OA_TIMER_PERIOD_MASK: c_uint = 0x3F;
pub const GEN8_OA_TIMER_PERIOD_SHIFT: c_int = 2;

pub const GEN7_OASTATUS1_TAIL_MASK: c_uint = 0xffffffc0;

pub const GEN7_OASTATUS2_HEAD_MASK: c_uint = 0xffffffc0;

pub const GEN8_OAHEADPTR_MASK: c_uint = 0xffffffc0;

pub const GEN8_OATAILPTR_MASK: c_uint = 0xffffffc0;

// Gen12 OAR unit

pub const GEN12_OAR_OACONTROL_COUNTER_FORMAT_SHIFT: c_int = 1;

// Gen12 OAG unit

pub const GEN12_OAG_OAHEADPTR_MASK: c_uint = 0xffffffc0;

pub const GEN12_OAG_OATAILPTR_MASK: c_uint = 0xffffffc0;

pub const GEN12_OAG_OAGLBCTXCTRL_TIMER_PERIOD_SHIFT: c_int = 2;

pub const GEN12_OAG_OACONTROL_OA_COUNTER_FORMAT_SHIFT: c_int = 2;

pub const GT_NOA_ENABLE: c_uint = 0x00000080;
// Gen12 OAM unit

pub const GEN12_OAM_HEAD_POINTER_MASK: c_uint = 0xffffffc0;

pub const GEN12_OAM_TAIL_POINTER_MASK: c_uint = 0xffffffc0;

pub const GEN12_OAM_CONTEXT_CONTROL_TIMER_PERIOD_SHIFT: c_int = 2;

pub const GEN12_OAM_CONTROL_COUNTER_FORMAT_SHIFT: c_int = 1;

