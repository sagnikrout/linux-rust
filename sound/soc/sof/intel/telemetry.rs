//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/telemetry.h
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
// Copyright(c) 2023 Intel Corporation
//
// telemetry data in debug windows
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xtensa_arch_block {
    pub /: *mut *mut u8 soc; / should be equal to XTENSA_SOC_INTEL_ADSP,
    pub version: u16,
    pub /: *mut *mut u8 toolchain; / ZEPHYR or XCC,
    pub pc: u32,
    pub exccause: u32,
    pub excvaddr: u32,
    pub sar: u32,
    pub ps: u32,
    pub scompare1: u32,
    pub ar: [u32; XTENSA_CORE_AR_REGS_COUNT],
    pub lbeg: u32,
    pub lend: u32,
    pub lcount: u32,
    pub __packed: },
    pub flags): *mut *mut void sof_ipc4_intel_dump_telemetry_state(struct snd_sof_dev sdev, u32,
