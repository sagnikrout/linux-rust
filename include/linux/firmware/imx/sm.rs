//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/imx/sm.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2024 NXP
//

extern "C" {
    pub fn scmi_imx_misc_ctrl_get(id: u32, num: *mut u32, val: *mut u32) -> c_int;
}
extern "C" {
    pub fn scmi_imx_misc_ctrl_set(id: u32, val: u32) -> c_int;
}

extern "C" {
    pub fn scmi_imx_cpu_start(cpuid: u32, start: bool) -> c_int;
}
extern "C" {
    pub fn scmi_imx_cpu_started(cpuid: u32, started: *mut bool) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_imx_lmm_op {
    SCMI_IMX_LMM_BOOT,
    SCMI_IMX_LMM_POWER_ON,
    SCMI_IMX_LMM_SHUTDOWN,
}

// For shutdown pperation
pub const SCMI_IMX_LMM_OP_FORCEFUL: c_int = 0;

extern "C" {
    pub fn scmi_imx_lmm_operation(lmid: u32, op: scmi_imx_lmm_op, flags: u32) -> c_int;
}
extern "C" {
    pub fn scmi_imx_lmm_info(lmid: u32, info: *mut scmi_imx_lmm_info) -> c_int;
}
extern "C" {
    pub fn scmi_imx_lmm_reset_vector_set(lmid: u32, cpuid: u32, flags: u32, vector: u64) -> c_int;
}

