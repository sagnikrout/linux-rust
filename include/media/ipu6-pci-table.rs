//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/ipu6-pci-table.h
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
// Copyright (C) 2024 Intel Corporation
//

pub const PCI_DEVICE_ID_INTEL_IPU6: c_uint = 0x9a19;
pub const PCI_DEVICE_ID_INTEL_IPU6SE: c_uint = 0x4e19;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_ADLP: c_uint = 0x465d;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_ADLN: c_uint = 0x462e;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_RPLP: c_uint = 0xa75d;
pub const PCI_DEVICE_ID_INTEL_IPU6EP_MTL: c_uint = 0x7d19;
