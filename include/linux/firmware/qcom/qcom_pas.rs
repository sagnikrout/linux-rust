//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/qcom/qcom_pas.h
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
// Copyright (c) 2010-2015, 2018-2019 The Linux Foundation. All rights reserved.
// Copyright (C) 2015 Linaro Ltd.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_pas_context {
    pub dev: *mut device,
    pub pas_id: u32,
    pub mem_phys: phys_addr_t,
    pub mem_size: usize,
    pub ptr: *mut c_void,
    pub phys: dma_addr_t,
    pub size: isize,
    pub use_tzmem: bool,
}

extern "C" {
    pub fn qcom_pas_is_available() -> bool;
}
extern "C" {
    pub fn qcom_pas_mem_setup(pas_id: u32, addr: phys_addr_t, size: phys_addr_t) -> c_int;
}
extern "C" {
    pub fn qcom_pas_auth_and_reset(pas_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_pas_prepare_and_auth_reset(ctx: *mut qcom_pas_context) -> c_int;
}
extern "C" {
    pub fn qcom_pas_set_remote_state(state: u32, pas_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_pas_shutdown(pas_id: u32) -> c_int;
}
extern "C" {
    pub fn qcom_pas_supported(pas_id: u32) -> bool;
}
extern "C" {
    pub fn qcom_pas_metadata_release(ctx: *mut qcom_pas_context);
}
