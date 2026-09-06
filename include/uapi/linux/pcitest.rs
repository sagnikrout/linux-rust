//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pcitest.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// pcitest.h - PCI test uapi defines
//
// Copyright (C) 2017 Texas Instruments
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const PCITEST_IRQ_TYPE_INTX: c_int = 0;
pub const PCITEST_IRQ_TYPE_MSI: c_int = 1;
pub const PCITEST_IRQ_TYPE_MSIX: c_int = 2;
pub const PCITEST_IRQ_TYPE_AUTO: c_int = 3;
pub const PCITEST_FLAGS_USE_DMA: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_endpoint_test_xfer_param {
    pub size: c_ulong,
    pub flags: c_uchar,
}
