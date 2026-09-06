//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/intel/perf.h
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
// perf.h - performance monitor header
//
// Copyright (C) 2021 Intel Corporation
//
// Author: Lu Baolu <baolu.lu@linux.intel.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum latency_type {
    DMAR_LATENCY_INV_IOTLB = 0,
    DMAR_LATENCY_INV_DEVTLB,
    DMAR_LATENCY_INV_IEC,
    DMAR_LATENCY_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum latency_count {
    COUNTS_10e2 = 0,	/* < 0.1us	*/
    COUNTS_10e3,		/* 0.1us ~ 1us	*/
    COUNTS_10e4,		/* 1us ~ 10us	*/
    COUNTS_10e5,		/* 10us ~ 100us	*/
    COUNTS_10e6,		/* 100us ~ 1ms	*/
    COUNTS_10e7,		/* 1ms ~ 10ms	*/
    COUNTS_10e8_plus,	/* 10ms and plus*/
    COUNTS_MIN,
    COUNTS_MAX,
    COUNTS_SUM,
    COUNTS_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct latency_statistic {
    pub enabled: bool,
    pub counter: [u64; COUNTS_NUM],
    pub samples: u64,
}

extern "C" {
    pub fn dmar_latency_enable(iommu: *mut intel_iommu, type: latency_type) -> c_int;
}
extern "C" {
    pub fn dmar_latency_disable(iommu: *mut intel_iommu, type: latency_type);
}
extern "C" {
    pub fn dmar_latency_enabled(iommu: *mut intel_iommu, type: latency_type) -> bool;
}
extern "C" {
    pub fn dmar_latency_snapshot(iommu: *mut intel_iommu, str: *mut c_char, size: usize);
}

