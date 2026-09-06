//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/msm_disp_snapshot.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2020-2021, The Linux Foundation. All rights reserved.
//

pub const MSM_DISP_SNAPSHOT_MAX_BLKS: c_int = 10;
// debug option to print the registers in logs
pub const MSM_DISP_SNAPSHOT_DUMP_IN_CONSOLE: c_int = 0;
// print debug ranges in groups of 4 u32s
pub const REG_DUMP_ALIGN: c_int = 16;
//
// struct msm_disp_state - structure to store current dpu state
// @dev: device pointer
// @drm_dev: drm device pointer
// @blocks: list head for hardware state blocks
// @atomic_state: atomic state duplicated at the time of the error
// @time: timestamp at which the coredump was captured
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_disp_state {
    pub dev: *mut device,
    pub drm_dev: *mut drm_device,
    pub blocks: list_head,
    pub atomic_state: *mut drm_atomic_commit,
    pub time: timespec64,
}

//
// struct msm_disp_state_block - structure to store each hardware block state
// @name: name of the block
// @node: handle to the linked list head
// @size: size of the register space of this hardware block
// @state: array holding the register dump of this hardware block
// @base_addr: starting address of this hardware block's register space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_disp_state_block {
    pub name: [c_char; SZ_128],
    pub node: list_head,
    pub size: c_uint,
    pub state: *mut u32,
    pub base_addr: *mut void __iomem,
}

//
// msm_disp_snapshot_init - initialize display snapshot
// @drm_dev:	drm device handle
//
// Returns:		0 or -ERROR
//
extern "C" {
    pub fn msm_disp_snapshot_init(drm_dev: *mut drm_device) -> c_int;
}
//
// msm_disp_snapshot_destroy - destroy the display snapshot
// @drm_dev:    drm device handle
//
// Returns:	none
//
extern "C" {
    pub fn msm_disp_snapshot_destroy(drm_dev: *mut drm_device);
}
//
// msm_disp_snapshot_state_sync - synchronously snapshot display state
// @kms:  the kms object
//
// Returns: state or error
//
// Context:
// Must be called with &kms->dump_mutex held
//
// msm_disp_snapshot_state - trigger to dump the display snapshot
// @drm_dev:	handle to drm device
//
// Returns:	none
//
extern "C" {
    pub fn msm_disp_snapshot_state(drm_dev: *mut drm_device);
}
//
// msm_disp_state_print - print out the current dpu state
// @disp_state:	    handle to drm device
// @p:	    handle to drm printer
//
// Returns:	none
//
extern "C" {
    pub fn msm_disp_state_print(disp_state: *mut msm_disp_state, p: *mut drm_printer);
}
//
// msm_disp_snapshot_capture_state - utility to capture atomic state and hw registers
// @disp_state:	    handle to msm_disp_state struct
//
// Returns:	none
//
extern "C" {
    pub fn msm_disp_snapshot_capture_state(disp_state: *mut msm_disp_state);
}
//
// msm_disp_state_free - free the memory after the coredump has been read
// @data:	    handle to struct msm_disp_state
//
// Returns: none
//
extern "C" {
    pub fn msm_disp_state_free(data: *mut c_void);
}
//
// msm_disp_snapshot_add_block - add a hardware block with its register dump
// @disp_state:	    handle to struct msm_disp_state
// @len:            size of the register space of the hardware block
// @base_addr:      starting address of the register space of the hardware block
// @fmt:            format in which the block names need to be printed
//
// Returns: none
//
