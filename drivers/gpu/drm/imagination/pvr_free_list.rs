//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_free_list.h
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

// Forward declaration from pvr_gem.h.
// Forward declaration from pvr_hwrt.h.
//
// struct pvr_free_list_node - structure representing an allocation in the free
// list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_free_list_node {
// @node: List node for &pvr_free_list.mem_block_list.
    pub node: list_head,
// @free_list: Pointer to owning free list.
    pub free_list: *mut pvr_free_list,
// @num_pages: Number of pages in this node.
    pub num_pages: u32,
// @mem_obj: GEM object representing the pages in this node.
    pub mem_obj: *mut pvr_gem_object,
}

//
// struct pvr_free_list - structure representing a free list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_free_list {
// @ref_count: Reference count of object.
    pub ref_count: kref,
// @pvr_dev: Pointer to device that owns this object.
    pub pvr_dev: *mut pvr_device,
// @obj: GEM object representing the free list.
    pub obj: *mut pvr_gem_object,
// @fw_obj: FW object representing the FW-side structure.
    pub fw_obj: *mut pvr_fw_object,
// @fw_data: Pointer to CPU mapping of the FW-side structure.
    pub fw_data: *mut rogue_fwif_freelist,
//
// @lock: Mutex protecting modification of the free list. Must be held when accessing any
// of the members below.
//
    pub lock: mutex,
// @fw_id: Firmware ID for this object.
    pub fw_id: u32,
// @current_pages: Current number of pages in free list.
    pub current_pages: u32,
// @max_pages: Maximum number of pages in free list.
    pub max_pages: u32,
// @grow_pages: Pages to grow free list by per request.
    pub grow_pages: u32,
//
// @grow_threshold: Percentage of FL memory used that should trigger a
// new grow request.
//
    pub grow_threshold: u32,
//
// @ready_pages: Number of pages reserved for FW to use while a grow
// request is being processed.
//
    pub ready_pages: u32,
// @mem_block_list: List of memory blocks in this free list.
    pub mem_block_list: list_head,
// @hwrt_list: List of HWRTs using this free list.
    pub hwrt_list: list_head,
// @initial_num_pages: Initial number of pages in free list.
    pub initial_num_pages: u32,
// @free_list_gpu_addr: Address of free list in GPU address space.
    pub free_list_gpu_addr: u64,
}

//
// pvr_free_list_lookup() - Lookup free list pointer from handle and file
// @pvr_file: Pointer to pvr_file structure.
// @handle: Object handle.
//
// Takes reference on free list object. Call pvr_free_list_put() to release.
//
// Returns:
// * The requested object on success, or
// * %NULL on failure (object does not exist in list, is not a free list, or
// does not belong to @pvr_file)
//
// pvr_free_list_lookup_id() - Lookup free list pointer from FW ID
// @pvr_dev: Device pointer.
// @id: FW object ID.
//
// Takes reference on free list object. Call pvr_free_list_put() to release.
//
// Returns:
// * The requested object on success, or
// * %NULL on failure (object does not exist in list, or is not a free list)
//
// Contexts are removed from the ctx_ids set in the context release path,
// meaning the ref_count reached zero before they get removed. We need
// to make sure we're not trying to acquire a context that's being
// destroyed.
//
