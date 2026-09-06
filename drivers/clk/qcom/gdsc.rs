//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/qcom/gdsc.h
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
// Copyright (c) 2015, 2017-2018, 2022, The Linux Foundation. All rights reserved.
//

//
// struct gdsc - Globally Distributed Switch Controller
// @pd: generic power domain
// @regmap: regmap for MMIO accesses
// @gdscr: gsdc control register
// @collapse_ctrl: APCS collapse-vote register
// @collapse_mask: APCS collapse-vote mask
// @gds_hw_ctrl: gds_hw_ctrl register
// @cxcs: offsets of branch registers to toggle mem/periph bits in
// @cxc_count: number of @cxcs
// @pwrsts: Possible powerdomain power states
// @en_rest_wait_val: transition delay value for receiving enr ack signal
// @en_few_wait_val: transition delay value for receiving enf ack signal
// @clk_dis_wait_val: transition delay value for halting clock
// @resets: ids of resets associated with this gdsc
// @reset_count: number of @resets
// @rcdev: reset controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdsc {
    pub pd: generic_pm_domain,
    pub parent: *mut generic_pm_domain,
    pub regmap: *mut regmap,
    pub gdscr: c_uint,
    pub collapse_ctrl: c_uint,
    pub collapse_mask: c_uint,
    pub gds_hw_ctrl: c_uint,
    pub clamp_io_ctrl: c_uint,
    pub cxcs: *mut c_uint,
    pub cxc_count: c_uint,
    pub en_rest_wait_val: c_uint,
    pub en_few_wait_val: c_uint,
    pub clk_dis_wait_val: c_uint,
    pub pwrsts: u8,
// Powerdomain allowable state bitfields

//
// There is no SW control to transition a GDSC into
// PWRSTS_RET. This happens in HW when the parent
// domain goes down to a low power state
//

    pub flags: u16,

    pub rcdev: *mut reset_controller_dev,
    pub resets: *mut c_uint,
    pub reset_count: c_uint,
    pub supply: *const c_char,
    pub rsupply: *mut regulator,
    pub needs_icc: bool,
    pub icc_path_index: c_uint,
    pub icc_path: *mut icc_path,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdsc_desc {
    pub dev: *mut device,
    pub scs: *mut gdsc,
    pub num: usize,
    pub pd_list: *mut dev_pm_domain_list,
}

extern "C" {
    pub fn gdsc_unregister(desc: *mut gdsc_desc);
}
extern "C" {
    pub fn gdsc_gx_do_nothing_enable(domain: *mut generic_pm_domain) -> c_int;
}
extern "C" {
    pub fn gdsc_gx_disable(domain: *mut generic_pm_domain) -> c_int;
}

