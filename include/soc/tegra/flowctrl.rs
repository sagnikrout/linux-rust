//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/tegra/flowctrl.h
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
//
// Functions and macros to control the flowcontroller
//
// Copyright (c) 2010-2012, NVIDIA Corporation. All rights reserved.
//
pub const FLOW_CTRL_HALT_CPU0_EVENTS: c_uint = 0x0;

pub const FLOW_CTRL_CPU0_CSR: c_uint = 0x8;

pub const FLOW_CTRL_HALT_CPU1_EVENTS: c_uint = 0x14;
pub const FLOW_CTRL_CPU1_CSR: c_uint = 0x18;

pub const TEGRA20_FLOW_CTRL_CSR_WFI_BITMAP: c_int = 0;

extern "C" {
    pub fn flowctrl_read_cpu_csr(cpuid: c_uint) -> u32;
}
extern "C" {
    pub fn flowctrl_write_cpu_csr(cpuid: c_uint, value: u32);
}
extern "C" {
    pub fn flowctrl_write_cpu_halt(cpuid: c_uint, value: u32);
}
extern "C" {
    pub fn flowctrl_cpu_suspend_enter(cpuid: c_uint);
}
extern "C" {
    pub fn flowctrl_cpu_suspend_exit(cpuid: c_uint);
}

