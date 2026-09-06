//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/exynos/exynos_drm_ipp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2017 Samsung Electronics Co., Ltd.
//
// struct exynos_drm_ipp_funcs - exynos_drm_ipp control functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_ipp_funcs {
//
// @commit:
//
// This is the main entry point to start framebuffer processing
// in the hardware. The exynos_drm_ipp_task has been already validated.
// This function must not wait until the device finishes processing.
// When the driver finishes processing, it has to call
// exynos_exynos_drm_ipp_task_done() function.
//
// RETURNS:
//
// 0 on success or negative error codes in case of failure.
//
    pub task): *mut exynos_drm_ipp_task,
//
// @abort:
//
// Informs the driver that it has to abort the currently running
// task as soon as possible (i.e. as soon as it can stop the device
// safely), even if the task would not have been finished by then.
// After the driver performs the necessary steps, it has to call
// exynos_drm_ipp_task_done() (as if the task ended normally).
// This function does not have to (and will usually not) wait
// until the device enters a state when it can be stopped.
//
    pub task): *mut exynos_drm_ipp_task,
}

//
// struct exynos_drm_ipp - central picture processor module structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_ipp {
    pub drm_dev: *mut drm_device,
    pub dev: *mut device,
    pub head: list_head,
    pub id: c_uint,
    pub name: *const c_char,
    pub funcs: *const exynos_drm_ipp_funcs,
    pub capabilities: c_uint,
    pub formats: *const exynos_drm_ipp_formats,
    pub num_formats: c_uint,
    pub sequence: core::sync::atomic::AtomicI32,
    pub lock: spinlock_t,
    pub task: *mut exynos_drm_ipp_task,
    pub todo_list: list_head,
    pub done_wq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_ipp_buffer {
    pub buf: drm_exynos_ipp_task_buffer,
    pub rect: drm_exynos_ipp_task_rect,
    pub exynos_gem: [*mut exynos_drm_gem; DRM_FORMAT_MAX_PLANES],
    pub format: *const drm_format_info,
    pub dma_addr: [dma_addr_t; DRM_FORMAT_MAX_PLANES],
}

//
// struct exynos_drm_ipp_task - a structure describing transformation that
// has to be performed by the picture processor hardware module
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_ipp_task {
    pub dev: *mut device,
    pub ipp: *mut exynos_drm_ipp,
    pub head: list_head,
    pub src: exynos_drm_ipp_buffer,
    pub dst: exynos_drm_ipp_buffer,
    pub transform: drm_exynos_ipp_task_transform,
    pub alpha: drm_exynos_ipp_task_alpha,
    pub cleanup_work: work_struct,
    pub flags: c_uint,
    pub ret: c_int,
    pub event: *mut drm_pending_exynos_ipp_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exynos_drm_ipp_formats {
    pub fourcc: u32,
    pub type: u32,
    pub modifier: u64,
    pub limits: *const drm_exynos_ipp_limit,
    pub num_limits: c_uint,
}

// helper macros to set exynos_drm_ipp_formats structure and limits

extern "C" {
    pub fn exynos_drm_ipp_task_done(task: *mut exynos_drm_ipp_task, ret: c_int);
}

