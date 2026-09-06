//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/hda-ipc.h
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
// Copyright(c) 2019 Intel Corporation
//
// Author: Keyon Jie <yang.jie@linux.intel.com>
//
// Primary register, mapped to
// - DIPCTDR (HIPCIDR) in sideband IPC (cAVS 1.8+)
// - DIPCT in cAVS 1.5 IPC
//
// Secondary register, mapped to:
// - DIPCTDD (HIPCIDD) in sideband IPC (cAVS 1.8+)
// - DIPCTE in cAVS 1.5 IPC
//
// Common bits in primary register
// Reserved for doorbell

// Target, 0 - normal message, 1 - compact message(cAVS compatible)

// Direction, 0 - request, 1 - response

pub const HDA_IPC_TYPE_SHIFT: c_int = 24;

// Command specific payload bits in secondary register
// Disable DMA tracing (0 - keep tracing, 1 - to disable DMA trace)

// Prevent clock gating (0 - cg allowed, 1 - DSP clock always on)

// Prevent power gating (0 - deep power state transitions allowed)

// Indicates whether streaming is active

extern "C" {
    pub fn cnl_ipc_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cnl_ipc_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
}
extern "C" {
    pub fn cnl_ipc_dump(sdev: *mut snd_sof_dev);
}
extern "C" {
    pub fn cnl_ipc4_dump(sdev: *mut snd_sof_dev);
}
