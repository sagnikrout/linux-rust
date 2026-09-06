//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/fsl/fsl_rpmsg.h
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
// Copyright 2017-2021 NXP
//
// struct fsl_rpmsg_soc_data
// @rates: supported rates
// @formats: supported formats
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_rpmsg_soc_data {
    pub rates: c_int,
    pub formats: u64,
}

//
// struct fsl_rpmsg - rpmsg private data
//
// @ipg: ipg clock for cpu dai (SAI)
// @mclk: master clock for cpu dai (SAI)
// @dma: clock for dma device
// @pll8k: parent clock for multiple of 8kHz frequency
// @pll11k: parent clock for multiple of 11kHz frequency
// @card_pdev: Platform_device pointer to register a sound card
// @soc_data: soc specific data
// @mclk_streams: Active streams that are using baudclk
// @force_lpa: force enable low power audio routine if condition satisfy
// @enable_lpa: enable low power audio routine according to dts setting
// @buffer_size: pre allocated dma buffer size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_rpmsg {
    pub ipg: *mut clk,
    pub mclk: *mut clk,
    pub dma: *mut clk,
    pub pll8k: *mut clk,
    pub pll11k: *mut clk,
    pub card_pdev: *mut platform_device,
    pub soc_data: *const fsl_rpmsg_soc_data,
    pub mclk_streams: c_uint,
    pub force_lpa: c_int,
    pub enable_lpa: c_int,
    pub buffer_size: [c_int; 2],
}
