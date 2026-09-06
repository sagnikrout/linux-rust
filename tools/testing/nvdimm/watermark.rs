//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/nvdimm/watermark.h
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
// Copyright(c) 2018 Intel Corporation. All rights reserved.
extern "C" {
    pub fn pmem_test() -> c_int;
}
extern "C" {
    pub fn libnvdimm_test() -> c_int;
}
extern "C" {
    pub fn acpi_nfit_test() -> c_int;
}
extern "C" {
    pub fn device_dax_test() -> c_int;
}
extern "C" {
    pub fn dax_pmem_test() -> c_int;
}
extern "C" {
    pub fn dax_pmem_core_test() -> c_int;
}
extern "C" {
    pub fn dax_pmem_compat_test() -> c_int;
}
//
// dummy routine for nfit_test to validate it is linking to the properly
// mocked module and not the standard one from the base tree.
//

