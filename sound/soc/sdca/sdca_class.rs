//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/sdca/sdca_class.h
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
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//
// Copyright (C) 2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdca_class_drv {
    pub dev: *mut device,
    pub dev_regmap: *mut regmap,
    pub sdw: *mut sdw_slave,
    pub irq_info: *mut sdca_interrupt_info,
    pub regmap_lock: mutex,
// Serialise function initialisations
    pub init_lock: mutex,
    pub boot_work: work_struct,
}
