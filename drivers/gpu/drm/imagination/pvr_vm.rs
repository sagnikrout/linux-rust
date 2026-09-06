//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_vm.h
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

// Forward declaration from "pvr_device.h"
// Forward declaration from "pvr_gem.h"
// Forward declaration from "pvr_vm.c"
// Forward declaration from <uapi/drm/pvr_drm.h>
// Forward declaration from <drm/drm_exec.h>
// Functions defined in pvr_vm.c
extern "C" {
    pub fn pvr_device_addr_is_valid(device_addr: u64) -> bool;
}
extern "C" {
    pub fn pvr_vm_unmap(vm_ctx: *mut pvr_vm_context, device_addr: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn pvr_vm_unmap_all(vm_ctx: *mut pvr_vm_context);
}
extern "C" {
    pub fn pvr_vm_get_page_table_root_addr(vm_ctx: *mut pvr_vm_context) -> dma_addr_t;
}
extern "C" {
    pub fn pvr_vm_context_put(vm_ctx: *mut pvr_vm_context) -> bool;
}
extern "C" {
    pub fn pvr_destroy_vm_contexts_for_file(pvr_file: *mut pvr_file);
}
