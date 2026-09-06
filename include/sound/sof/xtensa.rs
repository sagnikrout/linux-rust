//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/xtensa.h
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
// Copyright(c) 2018 Intel Corporation
//

//
// Architecture specific debug
//
// Xtensa Firmware Oops data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_dsp_oops_arch_hdr,
    pub plat_hdr: sof_ipc_dsp_oops_plat_hdr,
    pub exccause: u32,
    pub excvaddr: u32,
    pub ps: u32,
    pub epc1: u32,
    pub epc2: u32,
    pub epc3: u32,
    pub epc4: u32,
    pub epc5: u32,
    pub epc6: u32,
    pub epc7: u32,
    pub eps2: u32,
    pub eps3: u32,
    pub eps4: u32,
    pub eps5: u32,
    pub eps6: u32,
    pub eps7: u32,
    pub depc: u32,
    pub intenable: u32,
    pub interrupt: u32,
    pub sar: u32,
    pub debugcause: u32,
    pub windowbase: u32,
    pub windowstart: u32,
    pub excsave1: u32,
    pub ar: [u32; ],
    pub __packed: },
