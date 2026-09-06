//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_tl_debugfs.h
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
// Copyright (c) 2023 Intel Corporation.

pub const MAX_COUNT_NAME_SIZE: c_int = 32;

pub const ADF_TL_RP_REGS_FNAME_SIZE: c_int = 16;

//
// enum adf_tl_counter_type - telemetry counter types
// @ADF_TL_COUNTER_UNSUPPORTED: unsupported counter
// @ADF_TL_SIMPLE_COUNT: simple counter
// @ADF_TL_COUNTER_NS: latency counter, value in ns
// @ADF_TL_COUNTER_NS_AVG: accumulated average latency counter, value in ns
// @ADF_TL_COUNTER_MBPS: bandwidth, value in MBps
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_tl_counter_type {
    ADF_TL_COUNTER_UNSUPPORTED,
    ADF_TL_SIMPLE_COUNT,
    ADF_TL_COUNTER_NS,
    ADF_TL_COUNTER_NS_AVG,
    ADF_TL_COUNTER_MBPS,
}

//
// struct adf_tl_dbg_counter - telemetry counter definition
// @name: name of the counter as printed in the report
// @adf_tl_counter_type: type of the counter
// @offset1: offset of 1st register
// @offset2: offset of 2nd optional register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_tl_dbg_counter {
    pub name: *const c_char,
    pub type: adf_tl_counter_type,
    pub offset1: usize,
    pub offset2: usize,
}

// Telemetry counter aggregated values.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_tl_dbg_aggr_values {
    pub curr: u64,
    pub min: u64,
    pub max: u64,
    pub avg: u64,
}

//
// adf_tl_dbgfs_add() - Add telemetry's debug fs entries.
// @accel_dev: Pointer to acceleration device.
//
// Creates telemetry's debug fs folder and attributes in QAT debug fs root.
//
extern "C" {
    pub fn adf_tl_dbgfs_add(accel_dev: *mut adf_accel_dev);
}
//
// adf_tl_dbgfs_rm() - Remove telemetry's debug fs entries.
// @accel_dev: Pointer to acceleration device.
//
// Removes telemetry's debug fs folder and attributes from QAT debug fs root.
//
extern "C" {
    pub fn adf_tl_dbgfs_rm(accel_dev: *mut adf_accel_dev);
}
