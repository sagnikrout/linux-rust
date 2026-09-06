//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/qat_compression.h
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
// Copyright(c) 2022 Intel Corporation

pub const QAT_COMP_MAX_SKID: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_compression_instance {
    pub dc_tx: *mut adf_etr_ring_data,
    pub dc_rx: *mut adf_etr_ring_data,
    pub accel_dev: *mut adf_accel_dev,
    pub list: list_head,
    pub state: c_ulong,
    pub id: c_int,
    pub refctr: core::sync::atomic::AtomicI32,
    pub backlog: qat_instance_backlog,
    pub dc_data: *mut adf_dc_data,
}
