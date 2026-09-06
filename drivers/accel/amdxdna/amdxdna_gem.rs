//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/amdxdna/amdxdna_gem.h
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
// Copyright (C) 2024, Advanced Micro Devices, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_umap {
    pub notifier: mmu_interval_notifier,
    pub range: hmm_range,
    pub hmm_unreg_work: work_struct,
    pub abo: *mut amdxdna_gem_obj,
    pub node: list_head,
    pub refcnt: kref,
    pub invalid: bool,
    pub unmapped: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_mem {
    pub kva: *mut c_void,
    pub dma_addr: u64,
    pub size: usize,
    pub umap_list: list_head,
    pub map_invalid: bool,
//
// Cache the first mmap uva as PASID addr, which can be accessed by driver
// without taking notifier_lock.
//
    pub uva: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdxdna_gem_obj {
    pub base: drm_gem_shmem_object,
    pub client: *mut amdxdna_client,
    pub type: u8,
    pub pinned: bool,
    pub /: *mut *mut mutex lock; / Protects: pinned, mem.kva, open_ref,
    pub mem: amdxdna_mem,
    pub open_ref: c_int,
// Below members are initialized when needed
    pub /: *mut *mut drm_mm_node mm_node; / For AMDXDNA_BO_DEV,
    pub heap_start_id: u32,
    pub heap_end_id: u32,
    pub /: *mut *mut u64 dev_addr; / For heap bo,
    pub assigned_hwctx: u32,
    pub dma_buf: *mut dma_buf,
    pub attach: *mut dma_buf_attachment,
// True, if BO is managed by XRT, not application
    pub internal: bool,
// True, if BO is not exportable
    pub private_buffer: bool,
}

extern "C" {
    pub fn container_of(_arg: gobj, amdxdna_gem_obj: struct, _arg: base.base) -> return;
}
//
// Obtain the user virtual address for accessing the BO.
// It can be used for device to access the BO when PASID is enabled.
//
extern "C" {
    pub fn amdxdna_gem_dev_addr(abo: *mut amdxdna_gem_obj) -> u64;
}
//
// amdxdna_gem_obj_open() calls amdxdna_dma_map_bo() only when PASID is
// off, leaving mem.dma_addr at AMDXDNA_INVALID_ADDR when PASID is on.
// Avoid dereferencing abo->client, which is cleared to NULL by
// amdxdna_gem_obj_close() while internal kernel references remain.
//
extern "C" {
    pub fn amdxdna_umap_put(mapp: *mut amdxdna_umap);
}
extern "C" {
    pub fn amdxdna_gem_pin_nolock(abo: *mut amdxdna_gem_obj) -> c_int;
}
extern "C" {
    pub fn amdxdna_gem_pin(abo: *mut amdxdna_gem_obj) -> c_int;
}
extern "C" {
    pub fn amdxdna_gem_unpin(abo: *mut amdxdna_gem_obj);
}
extern "C" {
    pub fn amdxdna_drm_create_bo_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_get_bo_info_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_sync_bo_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdxdna_drm_get_bo_usage(dev: *mut drm_device, args: *mut amdxdna_drm_get_array) -> c_int;
}
