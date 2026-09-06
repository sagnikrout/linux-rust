//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/aie2_solver.h
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
// Copyright (C) 2023-2024, Advanced Micro Devices, Inc.
//
pub const XRS_MAX_COL: c_int = 128;
//
// Structure used to describe a partition. A partition is column based
// allocation unit described by its start column and number of columns.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_part {
    pub start_col: u32,
    pub ncols: u32,
}

//
// The QoS capabilities of a given AIE partition.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_qos_cap {
    pub /: *mut *mut u32 opc; / operations per cycle,
    pub /: *mut *mut u32 dma_bw; / DMA bandwidth,
}

//
// QoS requirement of a resource allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aie_qos {
    pub /: *mut *mut u32 gops; / Giga operations,
    pub /: *mut *mut u32 fps; / Frames per second,
    pub /: *mut *mut u32 dma_bw; / DMA bandwidth,
    pub /: *mut *mut u32 latency; / Frame response latency,
    pub /: *mut *mut u32 exec_time; / Frame execution time,
    pub /: *mut *mut u32 priority; / Request priority,
}

//
// Structure used to describe a relocatable CDO (Configuration Data Object).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdo_parts {
    pub /: *mut *mut *mut u32 start_cols; / Start column array,
    pub /: *mut *mut u32 cols_len; / Length of start column array,
    pub /: *mut *mut u32 ncols; / # of column,
    pub /: *mut *mut aie_qos_cap qos_cap; / CDO QoS capabilities,
}

//
// Structure used to describe a request to allocate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_requests {
    pub rid: u64,
    pub cdo: cdo_parts,
    pub /: *mut *mut aie_qos rqos; / Requested QoS,
}

//
// Load callback argument
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs_action_load {
    pub rid: u32,
    pub part: aie_part,
}

//
// Define the power level available
//
// POWER_LEVEL_MIN:
// Lowest power level. Usually set when all actions are unloaded.
//
// POWER_LEVEL_n
// Power levels 0 - n, is a step increase in system frequencies
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum power_level {
    POWER_LEVEL_MIN = 0x0,
    POWER_LEVEL_0   = 0x1,
    POWER_LEVEL_1   = 0x2,
    POWER_LEVEL_2   = 0x3,
    POWER_LEVEL_3   = 0x4,
    POWER_LEVEL_4   = 0x5,
    POWER_LEVEL_5   = 0x6,
    POWER_LEVEL_6   = 0x7,
    POWER_LEVEL_7   = 0x8,
    POWER_LEVEL_NUM,
}

//
// Structure used to describe the frequency table.
// Resource solver chooses the frequency from the table
// to meet the QOS requirements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_list_info {
    pub /: *mut *mut u32 num_levels; / available power levels,
    pub Mhz*/: *mut *mut u32 cu_clk_list[POWER_LEVEL_NUM]; / available aie clock frequencies in,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs_action_ops {
    pub action): *mut *mut *mut int (load)(void cb_arg, struct xrs_action_load,
    pub cb_arg): *mut *mut int (unload)(void,
    pub level): *mut *mut *mut int (set_dft_dpm_level)(struct drm_device ddev, u32,
}

//
// Structure used to describe information for solver during initialization.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_config {
    pub total_col: u32,
    pub /: *mut *mut u32 sys_eff_factor; / system efficiency factor,
    pub /: *mut *mut u32 latency_adj; / latency adjustment in ms,
    pub /: *mut *mut clk_list_info clk_list; / List of frequencies available in system,
    pub ddev: *mut drm_device,
    pub actions: *mut xrs_action_ops,
}

//
// xrsm_init() - Register resource solver. Resource solver client needs
// to call this function to register itself.
//
// @cfg:	The system metrics for resource solver to use
//
// Return:	A resource solver handle
//
// Note: We should only create one handle per AIE array to be managed.
//
// xrs_allocate_resource() - Request to allocate resources for a given context
// and a partition metadata. (See struct part_meta)
//
// @hdl:	Resource solver handle obtained from xrs_init()
// @req:	Input to the Resource solver including request id
// and partition metadata.
// @cb_arg:	callback argument pointer
//
// Return:	0 when successful.
// Or standard error number when failing
//
// Note:
// There is no lock mechanism inside resource solver. So it is
// the caller's responsibility to lock down XCLBINs and grab
// necessary lock.
//
extern "C" {
    pub fn xrs_allocate_resource(hdl: *mut c_void, req: *mut alloc_requests, cb_arg: *mut c_void) -> c_int;
}
//
// xrs_release_resource() - Request to free resources for a given context.
//
// @hdl:	Resource solver handle obtained from xrs_init()
// @rid:	The Request ID to identify the requesting context
//
extern "C" {
    pub fn xrs_release_resource(hdl: *mut c_void, rid: u64) -> c_int;
}
