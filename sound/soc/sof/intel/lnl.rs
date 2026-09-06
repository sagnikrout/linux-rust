//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/lnl.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2024 Intel Corporation
//
pub const LNL_DSP_REG_HFDSC: c_uint = 0x160200 /* DSP core0 status */;
pub const LNL_DSP_REG_HFDEC: c_uint = 0x160204 /* DSP core0 error */;
extern "C" {
    pub fn sof_lnl_set_ops(sdev: *mut snd_sof_dev, dsp_ops: *mut snd_sof_dsp_ops) -> c_int;
}
extern "C" {
    pub fn lnl_dsp_check_sdw_irq(sdev: *mut snd_sof_dev) -> bool;
}
extern "C" {
    pub fn lnl_dsp_disable_interrupts(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn lnl_sdw_check_wakeen_irq(sdev: *mut snd_sof_dev) -> bool;
}
