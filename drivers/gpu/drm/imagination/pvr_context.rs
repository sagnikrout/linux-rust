//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_context.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvr_context_priority {
    PVR_CTX_PRIORITY_LOW = 0,
    PVR_CTX_PRIORITY_MEDIUM,
    PVR_CTX_PRIORITY_HIGH,
}

//
// struct pvr_context - Context data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_context {
// @ref_count: Refcount for context.
    pub ref_count: kref,
// @pvr_dev: Pointer to owning device.
    pub pvr_dev: *mut pvr_device,
// @vm_ctx: Pointer to associated VM context.
    pub vm_ctx: *mut pvr_vm_context,
// @type: Type of context.
    pub type: drm_pvr_ctx_type,
// @flags: Context flags.
    pub flags: u32,
// @priority: Context priority
    pub priority: pvr_context_priority,
// @fw_obj: FW object representing FW-side context data.
    pub fw_obj: *mut pvr_fw_object,
// @data: Pointer to local copy of FW context data.
    pub data: *mut c_void,
// @data_size: Size of FW context data, in bytes.
    pub data_size: u32,
// @ctx_id: FW context ID.
    pub ctx_id: u32,
//
// @faulty: Set to 1 when the context queues had unfinished job when
// a GPU reset happened.
//
// In that case, the context is in an inconsistent state and can't be
// used anymore.
//
    pub faulty: core::sync::atomic::AtomicI32,
// @queues: Union containing all kind of queues.
// @geometry: Geometry queue.
    pub geometry: *mut pvr_queue,
// @fragment: Fragment queue.
    pub fragment: *mut pvr_queue,
}

// @compute: Compute queue.
// @compute: Transfer queue.
// @file_link: pvr_file PVR context list link.
//
// pvr_context_get() - Take additional reference on context.
// @ctx: Context pointer.
//
// Call pvr_context_put() to release.
//
// Returns:
// * The requested context on success, or
// * %NULL if no context pointer passed.
//
// pvr_context_get_if_referenced() - Take an additional reference on a still
// referenced context.
// @ctx: Context pointer.
//
// Call pvr_context_put() to release.
//
// Returns:
// * True on success, or
// * false if no context pointer passed, or the context wasn't still
// * referenced.
//
// pvr_context_lookup() - Lookup context pointer from handle and file.
// @pvr_file: Pointer to pvr_file structure.
// @handle: Context handle.
//
// Takes reference on context. Call pvr_context_put() to release.
//
// Return:
// * The requested context on success, or
// * %NULL on failure (context does not exist, or does not belong to @pvr_file).
//
// Take the array lock to protect against context removal.
//
// pvr_context_lookup_id() - Lookup context pointer from ID.
// @pvr_dev: Device pointer.
// @id: FW context ID.
//
// Takes reference on context. Call pvr_context_put() to release.
//
// Return:
// * The requested context on success, or
// * %NULL on failure (context does not exist).
//
// Take the array lock to protect against context removal.
// Contexts are removed from the ctx_ids set in the context release path,
// meaning the ref_count reached zero before they get removed. We need
// to make sure we're not trying to acquire a context that's being
// destroyed.
//
extern "C" {
    pub fn pvr_context_put(ctx: *mut pvr_context);
}
extern "C" {
    pub fn pvr_context_create(pvr_file: *mut pvr_file, args: *mut drm_pvr_ioctl_create_context_args) -> c_int;
}
extern "C" {
    pub fn pvr_context_destroy(pvr_file: *mut pvr_file, handle: u32) -> c_int;
}
extern "C" {
    pub fn pvr_destroy_contexts_for_file(pvr_file: *mut pvr_file);
}
extern "C" {
    pub fn pvr_context_device_init(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_context_device_fini(pvr_dev: *mut pvr_device);
}
