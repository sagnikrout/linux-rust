//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/clk-scmi.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2025 NXP
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scmi_clk_feats {
    SCMI_CLK_ATOMIC_SUPPORTED,
    SCMI_CLK_STATE_CTRL_SUPPORTED,
    SCMI_CLK_RATE_CTRL_SUPPORTED,
    SCMI_CLK_PARENT_CTRL_SUPPORTED,
    SCMI_CLK_DUTY_CYCLE_SUPPORTED,
    SCMI_CLK_EXT_OEM_SSC_SUPPORTED,
    SCMI_CLK_FEATS_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clk {
    pub id: u32,
    pub dev: *mut device,
    pub hw: clk_hw,
    pub info: *const scmi_clock_info,
    pub ph: *const scmi_protocol_handle,
    pub parent_data: *mut clk_parent_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clk_oem {
    pub feats_key): *mut u32 id, unsigned int,
    pub ss_conf): *const clk_spread_spectrum,
}

extern "C" {
    pub fn scmi_clk_oem_init(dev: *mut scmi_device) -> c_int;
}
