//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-ctcu.h
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
// Copyright (c) 2024-2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

// Maximum number of supported ETR devices for a single CTCU.
pub const ETR_MAX_NUM: c_int = 2;
//
// struct ctcu_etr_config
// @atid_offset:	offset to the ATID0 Register.
// @port_num:		in-port number of CTCU device that connected to ETR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcu_etr_config {
    pub atid_offset: u32,
    pub port_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcu_config {
    pub etr_cfgs: *const ctcu_etr_config,
    pub num_etr_config: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctcu_drvdata {
    pub base: *mut void __iomem,
    pub apb_clk: *mut clk,
    pub dev: *mut device,
    pub csdev: *mut coresight_device,
    pub spin_lock: raw_spinlock_t,
    pub atid_offset: [u32; ETR_MAX_NUM],
// refcnt for each traceid of each sink
    pub traceid_refcnt: [u8; ETR_MAX_NUM][CORESIGHT_TRACE_ID_RES_TOP],
}
