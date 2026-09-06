//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/dfl-fme.h
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
// Header file for FPGA Management Engine (FME) Driver
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Kang Luwei <luwei.kang@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Wu Hao <hao.wu@intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Henry Mitchel <henry.mitchel@intel.com>
//
// struct dfl_fme - dfl fme private data
//
// @mgr: FME's FPGA manager platform device.
// @region_list: linked list of FME's FPGA regions.
// @bridge_list: linked list of FME's FPGA bridges.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme {
    pub mgr: *mut platform_device,
    pub region_list: list_head,
    pub bridge_list: list_head,
}
