//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/regs/xe_irq_regs.h
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
// Copyright © 2024 Intel Corporation
//

pub const PCU_IRQ_OFFSET: c_uint = 0x444e0;
pub const GU_MISC_IRQ_OFFSET: c_uint = 0x444f0;

//
// Note: Interrupt registers 1900xx are VF accessible only until version 12.50.
// On newer platforms, VFs are using memory-based interrupts instead.
// However, for simplicity we keep this XE_REG_OPTION_VF tag intact.
//

pub const OTHER_GUC_INSTANCE: c_int = 0;
pub const OTHER_GSC_HECI2_INSTANCE: c_int = 3;
pub const OTHER_KCR_INSTANCE: c_int = 4;
pub const OTHER_GSC_INSTANCE: c_int = 6;

// irqs for OTHER_KCR_INSTANCE

