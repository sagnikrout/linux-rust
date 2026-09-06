//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_cfg_services.h
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
// Copyright(c) 2023 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_base_services {
    SVC_ASYM = 0,
    SVC_SYM,
    SVC_DC,
    SVC_DECOMP,
    SVC_BASE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_extended_services {
    SVC_DCC = SVC_BASE_COUNT,
    SVC_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_composed_services {
    SVC_SYM_ASYM = SVC_COUNT,
    SVC_SYM_DC,
    SVC_ASYM_DC,
}

extern "C" {
    pub fn adf_get_service_enabled(accel_dev: *mut adf_accel_dev) -> c_int;
}
extern "C" {
    pub fn adf_get_service_mask(accel_dev: *mut adf_accel_dev, mask: *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn adf_srv_to_cfg_svc_type(svc: adf_base_services) -> adf_cfg_service_type;
}
extern "C" {
    pub fn adf_is_service_enabled(accel_dev: *mut adf_accel_dev, svc: adf_base_services) -> bool;
}
