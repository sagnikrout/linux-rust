//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/pmu.h
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
// Copyright(c) 2023 Huawei
// CXL Specification rev 3.0 Setion 8.2.7 (CPMU Register Interface)
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_pmu_type {
    CXL_PMU_MEMDEV,
}

pub const CXL_PMU_REGMAP_SIZE: c_uint = 0xe00 /* Table 8-32 CXL 3.0 specification */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_pmu {
    pub dev: device,
    pub base: *mut void __iomem,
    pub assoc_id: c_int,
    pub index: c_int,
    pub type: cxl_pmu_type,
}

