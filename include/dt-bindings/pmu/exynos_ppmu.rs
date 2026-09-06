//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pmu/exynos_ppmu.h
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
// Samsung Exynos PPMU event types for counting in regs
//
// Copyright (c) 2019, Samsung Electronics
// Author: Lukasz Luba <l.luba@partner.samsung.com>
//
pub const PPMU_RO_BUSY_CYCLE_CNT: c_uint = 0x0;
pub const PPMU_WO_BUSY_CYCLE_CNT: c_uint = 0x1;
pub const PPMU_RW_BUSY_CYCLE_CNT: c_uint = 0x2;
pub const PPMU_RO_REQUEST_CNT: c_uint = 0x3;
pub const PPMU_WO_REQUEST_CNT: c_uint = 0x4;
pub const PPMU_RO_DATA_CNT: c_uint = 0x5;
pub const PPMU_WO_DATA_CNT: c_uint = 0x6;
pub const PPMU_RO_LATENCY: c_uint = 0x12;
pub const PPMU_WO_LATENCY: c_uint = 0x16;
pub const PPMU_V2_RO_DATA_CNT: c_uint = 0x4;
pub const PPMU_V2_WO_DATA_CNT: c_uint = 0x5;
pub const PPMU_V2_EVT3_RW_DATA_CNT: c_uint = 0x22;
