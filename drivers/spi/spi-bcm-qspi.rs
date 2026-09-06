//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-bcm-qspi.h
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
// Copyright 2016 Broadcom
//

// BSPI interrupt masks

// MSPI Interrupt masks

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_qspi_soc_intc {
    pub type): *mut *mut *mut void (bcm_qspi_int_ack)(struct bcm_qspi_soc_intc soc_intc, int,
    pub en): bool,
    pub soc_intc): *mut *mut u32 (bcm_qspi_get_int_status)(struct bcm_qspi_soc_intc,
}

// Read controller register
extern "C" {
    pub fn ioread32be(_arg: addr) -> return;
}
extern "C" {
    pub fn readl_relaxed(_arg: addr) -> return;
}
// Write controller register
// The common driver functions to be called by the SoC platform driver
extern "C" {
    pub fn bcm_qspi_remove(pdev: *mut platform_device);
}
// pm_ops used by the SoC platform driver called on PM suspend/resume
