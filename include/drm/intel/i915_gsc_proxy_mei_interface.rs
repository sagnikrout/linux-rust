//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/intel/i915_gsc_proxy_mei_interface.h
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
// Copyright (c) 2022-2023 Intel Corporation
//

//
// struct i915_gsc_proxy_component_ops - ops for GSC Proxy services.
// @owner: Module providing the ops
// @send: sends a proxy message from GSC FW to ME FW
// @recv: receives a proxy message for GSC FW from ME FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gsc_proxy_component_ops {
    pub owner: *mut module,
//
// @send: Sends a proxy message to ME FW.
// @dev: device struct corresponding to the mei device
// @buf: message buffer to send
// @size: size of the message
// Return: bytes sent on success, negative errno value on failure
//
    pub size): *const *const *const *const int (send)(struct device dev, void buf, size_t,
//
// @recv: Receives a proxy message from ME FW.
// @dev: device struct corresponding to the mei device
// @buf: message buffer to contain the received message
// @size: size of the buffer
// Return: bytes received on success, negative errno value on failure
//
    pub size): *mut *mut *mut *mut int (recv)(struct device dev, void buf, size_t,
}

//
// struct i915_gsc_proxy_component - Used for communication between i915 and
// MEI drivers for GSC proxy services
// @mei_dev: device that provide the GSC proxy service.
// @ops: Ops implemented by GSC proxy driver, used by i915 driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gsc_proxy_component {
    pub mei_dev: *mut device,
    pub ops: *const i915_gsc_proxy_component_ops,
}
