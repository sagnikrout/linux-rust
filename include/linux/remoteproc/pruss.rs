//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/remoteproc/pruss.h
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
// PRU-ICSS Subsystem user interfaces
//
// Copyright (C) 2015-2022 Texas Instruments Incorporated - http://www.ti.com
// Suman Anna <s-anna@ti.com>
//

//
// enum pruss_pru_id - PRU core identifiers
// @PRUSS_PRU0: PRU Core 0.
// @PRUSS_PRU1: PRU Core 1.
// @PRUSS_NUM_PRUS: Total number of PRU Cores available.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pruss_pru_id {
    PRUSS_PRU0 = 0,
    PRUSS_PRU1,
    PRUSS_NUM_PRUS,
}

//
// enum pru_ctable_idx - Configurable Constant table index identifiers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pru_ctable_idx {
    PRU_C24 = 0,
    PRU_C25,
    PRU_C26,
    PRU_C27,
    PRU_C28,
    PRU_C29,
    PRU_C30,
    PRU_C31,
}

extern "C" {
    pub fn pru_rproc_put(rproc: *mut rproc);
}
extern "C" {
    pub fn pru_rproc_set_ctable(rproc: *mut rproc, c: pru_ctable_idx, addr: u32) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

