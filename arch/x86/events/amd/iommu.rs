//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/events/amd/iommu.h
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
// Copyright (C) 2013 Advanced Micro Devices, Inc.
//
// Author: Steven Kinney <Steven.Kinney@amd.com>
// Author: Suravee Suthikulpanit <Suraveee.Suthikulpanit@amd.com>
//
// iommu pc mmio region register indexes
pub const IOMMU_PC_COUNTER_REG: c_uint = 0x00;
pub const IOMMU_PC_COUNTER_SRC_REG: c_uint = 0x08;
pub const IOMMU_PC_PASID_MATCH_REG: c_uint = 0x10;
pub const IOMMU_PC_DOMID_MATCH_REG: c_uint = 0x18;
pub const IOMMU_PC_DEVID_MATCH_REG: c_uint = 0x20;
pub const IOMMU_PC_COUNTER_REPORT_REG: c_uint = 0x28;
// maximum specified bank/counters
pub const PC_MAX_SPEC_BNKS: c_int = 64;
pub const PC_MAX_SPEC_CNTRS: c_int = 16;
