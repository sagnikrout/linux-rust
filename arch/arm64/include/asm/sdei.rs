//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/sdei.h
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
// Copyright (C) 2017 Arm Ltd.
// Values for sdei_exit_mode
pub const SDEI_EXIT_HVC: c_int = 0;
pub const SDEI_EXIT_SMC: c_int = 1;

// Software Delegated Exception entry point from firmware
// and its CONFIG_UNMAP_KERNEL_AT_EL0 trampoline
// Abort a running handler. Context is discarded.
extern "C" {
    pub fn __sdei_handler_abort();
}
//
// The above entry point does the minimum to call C code. This function does
// anything else, before calling the driver.
//
extern "C" {
    pub fn sdei_arch_get_entry_point(conduit: c_int) -> c_ulong;
}

