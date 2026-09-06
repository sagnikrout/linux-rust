//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_hwrt.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from pvr_free_list.h.
// Forward declaration from pvr_gem.h.
//
// struct pvr_hwrt_data - structure representing HWRT data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_hwrt_data {
// @fw_obj: FW object representing the FW-side structure.
    pub fw_obj: *mut pvr_fw_object,
// @data: Local copy of FW-side structure.
    pub data: rogue_fwif_hwrtdata,
// @freelist_node: List node connecting this HWRT to the local freelist.
    pub freelist_node: list_head,
//
// @srtc_obj: FW object representing shadow render target cache.
//
// Only valid if @max_rts > 1.
//
    pub srtc_obj: *mut pvr_fw_object,
//
// @raa_obj: FW object representing renders accumulation array.
//
// Only valid if @max_rts > 1.
//
    pub raa_obj: *mut pvr_fw_object,
// @hwrt_dataset: Back pointer to owning HWRT dataset.
    pub hwrt_dataset: *mut pvr_hwrt_dataset,
}

//
// struct pvr_hwrt_dataset - structure representing a HWRT data set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_hwrt_dataset {
// @ref_count: Reference count of object.
    pub ref_count: kref,
// @pvr_dev: Pointer to device that owns this object.
    pub pvr_dev: *mut pvr_device,
// @common_fw_obj: FW object representing common FW-side structure.
    pub common_fw_obj: *mut pvr_fw_object,
// @common: Common HWRT data.
    pub common: rogue_fwif_hwrtdata_common,
// @data: HWRT data structures belonging to this set.
    pub data: [pvr_hwrt_data; ROGUE_FWIF_NUM_RTDATAS],
// @free_lists: Free lists used by HWRT data set.
    pub free_lists: [*mut pvr_free_list; ROGUE_FWIF_NUM_RTDATA_FREELISTS],
// @max_rts: Maximum render targets for this HWRT data set.
    pub max_rts: u16,
}

//
// pvr_hwrt_dataset_lookup() - Lookup HWRT dataset pointer from handle
// @pvr_file: Pointer to pvr_file structure.
// @handle: Object handle.
//
// Takes reference on dataset object. Call pvr_hwrt_dataset_put() to release.
//
// Returns:
// * The requested object on success, or
// * %NULL on failure (object does not exist in list, or is not a HWRT
// dataset)
//
// pvr_hwrt_data_lookup() - Lookup HWRT data pointer from handle and index
// @pvr_file: Pointer to pvr_file structure.
// @handle: Object handle.
// @index: Index of RT data within dataset.
//
// Takes reference on dataset object. Call pvr_hwrt_data_put() to release.
//
// Returns:
// * The requested object on success, or
// * %NULL on failure (object does not exist in list, or is not a HWRT
// dataset, or index is out of range)
//
// pvr_hwrt_data_put() - Release reference on HWRT data
// @hwrt: Pointer to HWRT data to release reference on
//
