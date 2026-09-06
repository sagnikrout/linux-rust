//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/pmc/ssram_telemetry.h
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
// Intel PMC SSRAM Telemetry PCI Driver Header File
//
// Copyright (c) 2024, Intel Corporation.
//
// struct pmc_ssram_telemetry - Structure to keep pmc info in ssram device
// @devid:		device id of the pmc device
// @base_addr:		contains PWRM base address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmc_ssram_telemetry {
    pub devid: u16,
    pub base_addr: u64,
}
