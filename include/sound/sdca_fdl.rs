//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sdca_fdl.h
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

//
// struct fdl_state - FDL state structure to keep data between interrupts
// @begin: Completion indicating the start of an FDL download cycle.
// @done: Completion indicating the end of an FDL download cycle.
// @timeout: Delayed work used for timing out UMP transactions.
// @lock: Mutex to protect between the timeout work and IRQ handlers.
// @interrupt: Pointer to the interrupt struct to which this FDL is attached.
// @set: Pointer to the FDL set currently being downloaded.
// @file_index: Index of the current file being processed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdl_state {
    pub begin: completion,
    pub done: completion,
    pub timeout: delayed_work,
    pub lock: mutex,
    pub interrupt: *mut sdca_interrupt,
    pub set: *mut sdca_fdl_set,
    pub file_index: c_int,
}

pub const SDCA_CTL_XU_FDLH_COMPLETE: c_int = 0;

pub const SDCA_CTL_XU_FDLD_COMPLETE: c_int = 0;

extern "C" {
    pub fn sdca_fdl_alloc_state(interrupt: *mut sdca_interrupt) -> c_int;
}
extern "C" {
    pub fn sdca_fdl_free_state(interrupt: *mut sdca_interrupt);
}
extern "C" {
    pub fn sdca_fdl_process(interrupt: *mut sdca_interrupt) -> c_int;
}

