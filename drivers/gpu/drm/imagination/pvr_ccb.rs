//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_ccb.h
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

// Forward declaration from pvr_device.h.
// Forward declaration from pvr_gem.h.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_ccb {
// @ctrl_obj: FW object representing CCB control structure.
    pub ctrl_obj: *mut pvr_fw_object,
// @ccb_obj: FW object representing CCB.
    pub ccb_obj: *mut pvr_fw_object,
// @ctrl_fw_addr: FW virtual address of CCB control structure.
    pub ctrl_fw_addr: u32,
// @ccb_fw_addr: FW virtual address of CCB.
    pub ccb_fw_addr: u32,
// @num_cmds: Number of commands in this CCB.
    pub num_cmds: u32,
// @cmd_size: Size of each command in this CCB, in bytes.
    pub cmd_size: u32,
// @lock: Mutex protecting @ctrl and @ccb.
    pub lock: mutex,
//
// @ctrl: Kernel mapping of CCB control structure. @lock must be held
// when accessing.
//
    pub ctrl: *mut rogue_fwif_ccb_ctl,
// @ccb: Kernel mapping of CCB. @lock must be held when accessing.
    pub ccb: *mut c_void,
}

extern "C" {
    pub fn pvr_kccb_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_kccb_fini(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_fwccb_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_ccb_fini(ccb: *mut pvr_ccb);
}
extern "C" {
    pub fn pvr_fwccb_process(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_kccb_fence_put(fence: *mut dma_fence);
}
extern "C" {
    pub fn pvr_kccb_release_slot(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_kccb_is_idle(pvr_dev: *mut pvr_device) -> bool;
}
extern "C" {
    pub fn pvr_kccb_wake_up_waiters(pvr_dev: *mut pvr_device);
}
