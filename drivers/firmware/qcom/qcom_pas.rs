//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firmware/qcom/qcom_pas.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// struct qcom_pas_ops - Qcom Peripheral Authentication Service (PAS) ops
// @drv_name:			PAS driver name.
// @dev:			PAS device pointer.
// @supported:			Peripheral supported callback.
// @init_image:			Peripheral image initialization callback.
// @mem_setup:			Peripheral memory setup callback.
// @get_rsc_table:		Peripheral get resource table callback.
// @prepare_and_auth_reset:	Peripheral prepare firmware authentication and
// reset callback.
// @auth_and_reset:		Peripheral firmware authentication and reset
// callback.
// @set_remote_state:		Peripheral set remote state callback.
// @shutdown:			Peripheral shutdown callback.
// @metadata_release:		Image metadata release callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pas_ops {
    pub drv_name: *const c_char,
    pub dev: *mut device,
    pub pas_id): *mut *mut *mut bool (supported)(struct device dev, u32,
    pub ctx): *mut size_t size, struct qcom_pas_context,
    pub size): phys_addr_t,
    pub output_rt_size): *mut usize,
    pub ctx): *mut qcom_pas_context,
    pub pas_id): *mut *mut *mut int (auth_and_reset)(struct device dev, u32,
    pub pas_id): *mut *mut *mut int (set_remote_state)(struct device dev, u32 state, u32,
    pub pas_id): *mut *mut *mut int (shutdown)(struct device dev, u32,
    pub ctx): *mut qcom_pas_context,
}

extern "C" {
    pub fn qcom_pas_ops_register(ops: *mut qcom_pas_ops);
}
extern "C" {
    pub fn qcom_pas_ops_unregister();
}
