//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/msm_iommu.h
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
// Copyright (c) 2010-2011, Code Aurora Forum. All rights reserved.
//

// Sharability attributes of MSM IOMMU mappings
pub const MSM_IOMMU_ATTR_NON_SH: c_uint = 0x0;
pub const MSM_IOMMU_ATTR_SH: c_uint = 0x4;
// Cacheability attributes of MSM IOMMU mappings
pub const MSM_IOMMU_ATTR_NONCACHED: c_uint = 0x0;
pub const MSM_IOMMU_ATTR_CACHED_WB_WA: c_uint = 0x1;
pub const MSM_IOMMU_ATTR_CACHED_WB_NWA: c_uint = 0x2;
pub const MSM_IOMMU_ATTR_CACHED_WT: c_uint = 0x3;
// Mask for the cache policy attribute
pub const MSM_IOMMU_CP_MASK: c_uint = 0x03;
// Maximum number of Machine IDs that we are allowing to be mapped to the same
// context bank. The number of MIDs mapped to the same CB does not affect
// performance, but there is a practical limit on how many distinct MIDs may
// be present. These mappings are typically determined at design time and are
// not expected to change at run time.
//
pub const MAX_NUM_MIDS: c_int = 32;
// Maximum number of context banks that can be present in IOMMU
pub const IOMMU_MAX_CBS: c_int = 128;
//
// struct msm_iommu_dev - a single IOMMU hardware instance
// ncb		Number of context banks present on this IOMMU HW instance
// dev:		IOMMU device
// irq:		Interrupt number
// clk:		The bus clock for this IOMMU hardware instance
// pclk:	The clock for the IOMMU bus interconnect
// dev_node:	list head in qcom_iommu_device_list
// dom_node:	list head for domain
// ctx_list:	list of 'struct msm_iommu_ctx_dev'
// context_map: Bitmap to track allocated context banks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_iommu_dev {
    pub base: *mut void __iomem,
    pub ncb: c_int,
    pub dev: *mut device,
    pub irq: c_int,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub dev_node: list_head,
    pub dom_node: list_head,
    pub ctx_list: list_head,
    pub IOMMU_MAX_CBS): DECLARE_BITMAP(context_map,,
    pub iommu: iommu_device,
}

//
// struct msm_iommu_ctx_dev - an IOMMU context bank instance
// of_node	node ptr of client device
// num		Index of this context bank within the hardware
// mids		List of Machine IDs that are to be mapped into this context
// bank, terminated by -1. The MID is a set of signals on the
// AXI bus that identifies the function associated with a specific
// memory request. (See ARM spec).
// num_mids	Total number of mids
// node		list head in ctx_list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_iommu_ctx_dev {
    pub of_node: *mut device_node,
    pub num: c_int,
    pub mids: [c_int; MAX_NUM_MIDS],
    pub num_mids: c_int,
    pub list: list_head,
}

//
// Interrupt handler for the IOMMU context fault interrupt. Hooking the
// interrupt is not supported in the API yet, but this will print an error
// message and dump useful IOMMU registers.
//
extern "C" {
    pub fn msm_iommu_fault_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
