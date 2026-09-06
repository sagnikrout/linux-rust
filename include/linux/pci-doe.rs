//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-doe.h
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
// Data Object Exchange
// PCIe r6.0, sec 6.30 DOE
//
// Copyright (C) 2021 Huawei
// Jonathan Cameron <Jonathan.Cameron@huawei.com>
//
// Copyright (C) 2022 Intel Corporation
// Ira Weiny <ira.weiny@intel.com>
//
pub const PCI_DOE_FEATURE_DISCOVERY: c_int = 0;
pub const PCI_DOE_FEATURE_CMA: c_int = 1;
pub const PCI_DOE_FEATURE_SSESSION: c_int = 2;
