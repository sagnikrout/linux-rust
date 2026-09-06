//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/dreamchip/rppx1/rpp_module.h
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
// Copyright (C) 2026 Renesas Electronics Corp.
// Copyright (C) 2026 Ideas on Board Oy
// Copyright (C) 2026 Ragnatech AB
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rpp_raw_pattern {
    RPP_RGGB = 0,
    RPP_GRBG,
    RPP_GBRG,
    RPP_BGGR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpp_module {
    pub rpp: *mut rppx1,
    pub base: u32,
    pub ops: *const rpp_module_ops,
    pub raw_pattern: rpp_raw_pattern,
    pub acq: },
    pub info: },
}

extern "C" {
    pub fn rpp_module_write(mod: *mut rpp_module, offset: u32, value: u32);
}
extern "C" {
    pub fn rpp_module_read(mod: *mut rpp_module, offset: u32) -> u32;
}
extern "C" {
    pub fn rpp_module_clrset(mod: *mut rpp_module, offset: u32, mask: u32, value: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union rppx1_params_block {
    pub header: v4l2_isp_block_header,
    pub bls: rppx1_bls_params,
    pub lin: rppx1_lin_params,
    pub lsc: rppx1_lsc_params,
    pub awbg: rppx1_awbg_params,
    pub ccor: rppx1_ccor_params,
    pub hist: rppx1_hist_params,
    pub exm: rppx1_exm_params,
    pub wbmeas: rppx1_wbmeas_params,
    pub ga: rppx1_ga_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rppx1_stats_block {
    pub header: v4l2_isp_block_header,
    pub hist: rppx1_hist_stats,
    pub exm: rppx1_exm_stats,
    pub wbmeas: rppx1_wbmeas_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpp_module_ops {
    pub mod): *mut *mut int (probe)(struct rpp_module,
    pub fmt): *const *const *const int (start)(struct rpp_module mod, struct v4l2_mbus_framefmt,
    pub priv): *mut rppx1_reg_write write, void,
    pub block): *mut rppx1_stats_block,
}

