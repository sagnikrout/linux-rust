//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rpmsg/mtk_rpmsg.h
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
// Copyright 2019 Google LLC.
//

extern "C" {
    pub fn void(data: *mut *mut ipi_handler_t)(void, len: c_uint, priv: *mut c_void) -> typedef;
}
//
// struct mtk_rpmsg_info - IPI functions tied to the rpmsg device.
// @register_ipi: register IPI handler for an IPI id.
// @unregister_ipi: unregister IPI handler for a registered IPI id.
// @send_ipi: send IPI to an IPI id. wait is the timeout (in msecs) to wait
// until response, or 0 if there's no timeout.
// @ns_ipi_id: the IPI id used for name service, or -1 if name service isn't
// supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rpmsg_info {
    pub priv): *mut ipi_handler_t handler, void,
    pub id): *mut *mut *mut void (unregister_ipi)(struct platform_device pdev, u32,
    pub wait): *const *const void buf, unsigned int len, unsigned int,
    pub ns_ipi_id: c_int,
}

extern "C" {
    pub fn mtk_rpmsg_destroy_rproc_subdev(subdev: *mut rproc_subdev);
}
