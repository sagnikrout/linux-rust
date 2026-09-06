//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/dfl-fme-pr.h
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
// Header file for FPGA Management Engine (FME) Partial Reconfiguration Driver
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

//
// struct dfl_fme_region - FME fpga region data structure
//
// @region: platform device of the FPGA region.
// @node: used to link fme_region to a list.
// @port_id: indicate which port this region connected to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme_region {
    pub region: *mut platform_device,
    pub node: list_head,
    pub port_id: c_int,
}

//
// struct dfl_fme_region_pdata - platform data for FME region platform device.
//
// @mgr: platform device of the FPGA manager.
// @br: platform device of the FPGA bridge.
// @region_id: region id (same as port_id).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme_region_pdata {
    pub mgr: *mut platform_device,
    pub br: *mut platform_device,
    pub region_id: c_int,
}

//
// struct dfl_fme_bridge - FME fpga bridge data structure
//
// @br: platform device of the FPGA bridge.
// @node: used to link fme_bridge to a list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme_bridge {
    pub br: *mut platform_device,
    pub node: list_head,
}

//
// struct dfl_fme_br_pdata - platform data for FME bridge platform device.
//
// @cdev: container device.
// @port_id: port id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme_br_pdata {
    pub cdev: *mut dfl_fpga_cdev,
    pub port_id: c_int,
}

//
// struct dfl_fme_mgr_pdata - platform data for FME manager platform device.
//
// @ioaddr: mapped io address for FME manager platform device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_fme_mgr_pdata {
    pub ioaddr: *mut void __iomem,
}

