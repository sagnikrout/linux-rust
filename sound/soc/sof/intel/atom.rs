//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sof/intel/atom.h
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
// Copyright(c) 2017-2021 Intel Corporation
//
// Author: Liam Girdwood <liam.r.girdwood@linux.intel.com>
//
// DSP memories
pub const IRAM_OFFSET: c_uint = 0x0C0000;

pub const DRAM_OFFSET: c_uint = 0x100000;

pub const SHIM_OFFSET: c_uint = 0x140000;
pub const SHIM_SIZE_BYT: c_uint = 0x100;
pub const SHIM_SIZE_CHT: c_uint = 0x118;
pub const MBOX_OFFSET: c_uint = 0x144000;
pub const MBOX_SIZE: c_uint = 0x1000;
pub const EXCEPT_OFFSET: c_uint = 0x800;
pub const EXCEPT_MAX_HDR_SIZE: c_uint = 0x400;
// DSP peripherals
pub const DMAC0_OFFSET: c_uint = 0x098000;
pub const DMAC1_OFFSET: c_uint = 0x09c000;
pub const DMAC2_OFFSET: c_uint = 0x094000;
pub const DMAC_SIZE: c_uint = 0x420;
pub const SSP0_OFFSET: c_uint = 0x0a0000;
pub const SSP1_OFFSET: c_uint = 0x0a1000;
pub const SSP2_OFFSET: c_uint = 0x0a2000;
pub const SSP3_OFFSET: c_uint = 0x0a4000;
pub const SSP4_OFFSET: c_uint = 0x0a5000;
pub const SSP5_OFFSET: c_uint = 0x0a6000;
pub const SSP_SIZE: c_uint = 0x100;
pub const STACK_DUMP_SIZE: c_int = 32;
pub const PCI_BAR_SIZE: c_uint = 0x200000;

//
// Debug
//
pub const MBOX_DUMP_SIZE: c_uint = 0x30;
// BARs
pub const DSP_BAR: c_int = 0;
pub const PCI_BAR: c_int = 1;
pub const IMR_BAR: c_int = 2;
extern "C" {
    pub fn atom_irq_handler(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn atom_irq_thread(irq: c_int, context: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn atom_send_msg(sdev: *mut snd_sof_dev, msg: *mut snd_sof_ipc_msg) -> c_int;
}
extern "C" {
    pub fn atom_get_mailbox_offset(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn atom_get_window_offset(sdev: *mut snd_sof_dev, id: u32) -> c_int;
}
extern "C" {
    pub fn atom_run(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn atom_reset(sdev: *mut snd_sof_dev) -> c_int;
}
extern "C" {
    pub fn atom_dump(sdev: *mut snd_sof_dev, flags: u32);
}
