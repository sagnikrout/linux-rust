//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/pm_helpers.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2019 Linaro Ltd.
pub const POWER_ON: c_int = 1;
pub const POWER_OFF: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct venus_pm_ops {
    pub core): *mut *mut int (core_get)(struct venus_core,
    pub core): *mut *mut void (core_put)(struct venus_core,
    pub on): *mut *mut *mut int (core_power)(struct venus_core core, int,
    pub dev): *mut *mut int (vdec_get)(struct device,
    pub dev): *mut *mut void (vdec_put)(struct device,
    pub on): *mut *mut *mut int (vdec_power)(struct device dev, int,
    pub dev): *mut *mut int (venc_get)(struct device,
    pub dev): *mut *mut void (venc_put)(struct device,
    pub on): *mut *mut *mut int (venc_power)(struct device dev, int,
    pub on): *mut *mut *mut int (coreid_power)(struct venus_inst inst, int,
    pub inst): *mut *mut int (load_scale)(struct venus_inst,
}
