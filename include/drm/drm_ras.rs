//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_ras.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2026 Intel Corporation
//

//
// struct drm_ras_node - A DRM RAS Node
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_ras_node {
// @id: Unique identifier for the node. Dynamically assigned.
    pub id: u32,
//
// @device_name: Human-readable name of the device. Given by the driver.
//
    pub device_name: *const c_char,
// @node_name: Human-readable name of the node. Given by the driver.
    pub node_name: *const c_char,
// @type: Type of the node (enum drm_ras_node_type).
    pub type: drm_ras_node_type,
// Error-Counter Related Callback and Variables
// @error_counter_range: Range of valid Error IDs for this node.
// @first: First valid Error ID.
    pub first: u32,
// @last: Last valid Error ID. Mandatory entry.
    pub last: u32,
    pub error_counter_range: },
//
// @query_error_counter:
//
// This callback is used by drm-ras to query a specific error counter.
// Used for input check and to iterate all error counters in a node.
//
// Driver should expect query_error_counter() to be called with
// error_id from `error_counter_range.first` to
// `error_counter_range.last`.
//
// The @query_error_counter is a mandatory callback for
// error_counter_node.
//
// Returns: 0 on success,
// -ENOENT when error_id is not supported as an indication that
// drm_ras should silently skip this entry. Used for
// supporting non-contiguous error ranges.
// Driver is responsible for maintaining the list of
// supported error IDs in the range of first to last.
// Other negative values on errors that should terminate the
// netlink query.
//
    pub val): *const *const *const char name, u32,
//
// @clear_error_counter:
//
// This callback is used by drm_ras to clear a specific error counter.
// Driver should implement this callback to support clearing error counters
// of a node.
//
// Returns: 0 on success, negative error code on failure.
//
    pub error_id): *mut *mut *mut int (clear_error_counter)(struct drm_ras_node node, u32,
// @priv: Driver private data
    pub priv: *mut c_void,
}

extern "C" {
    pub fn drm_ras_node_register(node: *mut drm_ras_node) -> c_int;
}
extern "C" {
    pub fn drm_ras_node_unregister(node: *mut drm_ras_node);
}

