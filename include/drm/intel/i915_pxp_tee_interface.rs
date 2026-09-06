//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/i915_pxp_tee_interface.h
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
// Copyright © 2020 Intel Corporation
//

//
// struct i915_pxp_component_ops - ops for PXP services.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_pxp_component_ops {
//
// @owner: Module providing the ops.
//
    pub owner: *mut module,
//
// @send: Send a PXP message.
//
    pub timeout_ms): c_ulong,
//
// @recv: Receive a PXP message.
//
    pub timeout_ms): c_ulong,
//
// @gsc_command: Send a GSC command.
//
    pub sg_out): *mut scatterlist,
}

//
// struct i915_pxp_component - Used for communication between i915 and TEE
// drivers for the PXP services
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_pxp_component {
//
// @tee_dev: device that provide the PXP service from TEE Bus.
//
    pub tee_dev: *mut device,
//
// @ops: Ops implemented by TEE driver, used by i915 driver.
//
    pub ops: *const i915_pxp_component_ops,
//
// @mutex: To protect the above members.
//
    pub mutex: mutex,
}
