//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_map.h
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
// Copyright © 2022 Intel Corporation
//

//
// DOC: Map layer
//
// All access to any memory shared with a device (both sysmem and vram) in the
// Xe driver should go through this layer (xe_map). This layer is built on top
// of :ref:`driver-api/device-io:Generalizing Access to System and I/O Memory`
// and with extra hooks into the Xe driver that allows adding asserts to memory
// accesses (e.g. for blocking runtime_pm D3Cold on Discrete Graphics).
//
// FIXME: We likely should kill these two functions sooner or later
extern "C" {
    pub fn readl(_arg: map->vaddr_iomem) -> return;
}
extern "C" {
    pub fn READ_ONCE()map->vaddr: *mut *mut (u32) -> return;
}
// (u32 *)map->vaddr = val;

