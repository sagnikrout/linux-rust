//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/verisilicon/vs_dc.h
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
// Copyright (C) 2025 Icenowy Zheng <uwu@icenowy.me>
//
// Based on vs_dc_hw.h, which is:
// Copyright (C) 2023 VeriSilicon Holdings Co., Ltd.
//

pub const VSDC_MAX_OUTPUTS: c_int = 2;
pub const VSDC_RESET_COUNT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vs_dc {
    pub regs: *mut regmap,
    pub core_clk: *mut clk,
    pub axi_clk: *mut clk,
    pub ahb_clk: *mut clk,
    pub pix_clk: [*mut clk; VSDC_MAX_OUTPUTS],
    pub rsts: [reset_control_bulk_data; VSDC_RESET_COUNT],
    pub drm_dev: *mut vs_drm_dev,
    pub identity: vs_chip_identity,
}
