//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/hmi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Hypervisor Maintenance Interrupt header file.
//
// Copyright 2015 IBM Corporation
// Author: Mahesh Salgaonkar <mahesh@linux.vnet.ibm.com>
//

pub const CORE_TB_RESYNC_REQ_BIT: c_int = 63;
pub const MAX_SUBCORE_PER_CORE: c_int = 4;
//
// sibling_subcore_state structure is used to co-ordinate all threads
// during HMI to avoid TB corruption. This structure is allocated once
// per each core and shared by all threads on that core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sibling_subcore_state {
    pub flags: c_ulong,
    pub in_guest: [u8; MAX_SUBCORE_PER_CORE],
}

extern "C" {
    pub fn wait_for_subcore_guest_exit();
}
extern "C" {
    pub fn wait_for_tb_resync();
}

extern "C" {
    pub fn hmi_handle_debugtrig(regs: *mut pt_regs) -> c_long;
}
