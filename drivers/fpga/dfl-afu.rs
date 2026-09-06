//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/dfl-afu.h
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
// Header file for FPGA Accelerated Function Unit (AFU) Driver
//
// Copyright (C) 2017-2018 Intel Corporation, Inc.
//
// Authors:
// Wu Hao <hao.wu@intel.com>
// Xiao Guangrong <guangrong.xiao@linux.intel.com>
// Joseph Grecco <joe.grecco@intel.com>
// Enno Luebbers <enno.luebbers@intel.com>
// Tim Whisonant <tim.whisonant@intel.com>
// Ananda Ravuri <ananda.ravuri@intel.com>
// Henry Mitchel <henry.mitchel@intel.com>
//

//
// struct dfl_afu_mmio_region - afu mmio region data structure
//
// @index: region index.
// @flags: region flags (access permission).
// @size: region size.
// @offset: region offset from start of the device fd.
// @phys: region's physical address.
// @node: node to add to afu feature dev's region list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_afu_mmio_region {
    pub index: u32,
    pub flags: u32,
    pub size: u64,
    pub offset: u64,
    pub phys: u64,
    pub node: list_head,
}

//
// struct dfl_afu_dma_region - afu DMA region data structure
//
// @user_addr: region userspace virtual address.
// @length: region length.
// @iova: region IO virtual address.
// @pages: ptr to pages of this region.
// @node: rb tree node.
// @in_use: flag to indicate if this region is in_use.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_afu_dma_region {
    pub user_addr: u64,
    pub length: u64,
    pub iova: u64,
    pub pages: *mut page,
    pub node: rb_node,
    pub in_use: bool,
}

//
// struct dfl_afu - afu device data structure
//
// @region_cur_offset: current region offset from start to the device fd.
// @num_regions: num of mmio regions.
// @regions: the mmio region linked list of this afu feature device.
// @dma_regions: root of dma regions rb tree.
// @num_umsgs: num of umsgs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_afu {
    pub region_cur_offset: u64,
    pub num_regions: c_int,
    pub num_umsgs: u8,
    pub regions: list_head,
    pub dma_regions: rb_root,
}

// hold fdata->lock when call __afu_port_enable/disable
extern "C" {
    pub fn __afu_port_enable(fdata: *mut dfl_feature_dev_data) -> c_int;
}
extern "C" {
    pub fn __afu_port_disable(fdata: *mut dfl_feature_dev_data) -> c_int;
}
extern "C" {
    pub fn afu_mmio_region_init(fdata: *mut dfl_feature_dev_data);
}
extern "C" {
    pub fn afu_mmio_region_destroy(fdata: *mut dfl_feature_dev_data);
}
extern "C" {
    pub fn afu_dma_region_init(fdata: *mut dfl_feature_dev_data);
}
extern "C" {
    pub fn afu_dma_region_destroy(fdata: *mut dfl_feature_dev_data);
}
extern "C" {
    pub fn afu_dma_unmap_region(fdata: *mut dfl_feature_dev_data, iova: u64) -> c_int;
}
