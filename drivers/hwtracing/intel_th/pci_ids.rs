//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/intel_th/pci_ids.h
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
// Intel(R) Trace Hub driver debugging
//
// Copyright (C) 2025 Intel Corporation.
//
pub const PCI_DEVICE_ID_INTEL_NPK_CML: c_uint = 0x02a6;
pub const PCI_DEVICE_ID_INTEL_NPK_CML_PCH: c_uint = 0x06a6;
pub const PCI_DEVICE_ID_INTEL_NPK_GNR: c_uint = 0x0963;
pub const PCI_DEVICE_ID_INTEL_NPK_BXT: c_uint = 0x0a80;
pub const PCI_DEVICE_ID_INTEL_NPK_CDF: c_uint = 0x18e1;
pub const PCI_DEVICE_ID_INTEL_NPK_DNV: c_uint = 0x19e1;
pub const PCI_DEVICE_ID_INTEL_NPK_BXT_B: c_uint = 0x1a8e;
pub const PCI_DEVICE_ID_INTEL_NPK_EBG: c_uint = 0x1bcc;
pub const PCI_DEVICE_ID_INTEL_NPK_GLK: c_uint = 0x318e;
pub const PCI_DEVICE_ID_INTEL_NPK_GNR_SOC: c_uint = 0x3256;
pub const PCI_DEVICE_ID_INTEL_NPK_SPR: c_uint = 0x3456;
pub const PCI_DEVICE_ID_INTEL_NPK_ICL_PCH: c_uint = 0x34a6;
pub const PCI_DEVICE_ID_INTEL_NPK_TGL_PCH_H: c_uint = 0x43a6;
pub const PCI_DEVICE_ID_INTEL_NPK_EHL_CPU: c_uint = 0x4529;
pub const PCI_DEVICE_ID_INTEL_NPK_ICL_NNPI: c_uint = 0x45c5;
pub const PCI_DEVICE_ID_INTEL_NPK_ADL_CPU: c_uint = 0x466f;
pub const PCI_DEVICE_ID_INTEL_NPK_EHL: c_uint = 0x4b26;
pub const PCI_DEVICE_ID_INTEL_NPK_RKL: c_uint = 0x4c19;
pub const PCI_DEVICE_ID_INTEL_NPK_JSL_PCH: c_uint = 0x4da6;
pub const PCI_DEVICE_ID_INTEL_NPK_JSL_CPU: c_uint = 0x4e29;
pub const PCI_DEVICE_ID_INTEL_NPK_ADL_P: c_uint = 0x51a6;
pub const PCI_DEVICE_ID_INTEL_NPK_ADL_M: c_uint = 0x54a6;
pub const PCI_DEVICE_ID_INTEL_NPK_APL: c_uint = 0x5a8e;
pub const PCI_DEVICE_ID_INTEL_NPK_NVL_PCH: c_uint = 0x6e26;
pub const PCI_DEVICE_ID_INTEL_NPK_ARL: c_uint = 0x7724;
pub const PCI_DEVICE_ID_INTEL_NPK_RPL_S: c_uint = 0x7a26;
pub const PCI_DEVICE_ID_INTEL_NPK_ADL: c_uint = 0x7aa6;
pub const PCI_DEVICE_ID_INTEL_NPK_MTL_P: c_uint = 0x7e24;
pub const PCI_DEVICE_ID_INTEL_NPK_MTL_S: c_uint = 0x7f26;
pub const PCI_DEVICE_ID_INTEL_NPK_ICL_CPU: c_uint = 0x8a29;
pub const PCI_DEVICE_ID_INTEL_NPK_TGL_CPU: c_uint = 0x9a33;
pub const PCI_DEVICE_ID_INTEL_NPK_0: c_uint = 0x9d26;
pub const PCI_DEVICE_ID_INTEL_NPK_CNL_LP: c_uint = 0x9da6;
pub const PCI_DEVICE_ID_INTEL_NPK_TGL_PCH: c_uint = 0xa0a6;
pub const PCI_DEVICE_ID_INTEL_NPK_1: c_uint = 0xa126;
pub const PCI_DEVICE_ID_INTEL_NPK_LBG_PCH: c_uint = 0xa1a6;
pub const PCI_DEVICE_ID_INTEL_NPK_LBG_PCH_2: c_uint = 0xa226;
pub const PCI_DEVICE_ID_INTEL_NPK_KBL_PCH: c_uint = 0xa2a6;
pub const PCI_DEVICE_ID_INTEL_NPK_CNL_H: c_uint = 0xa326;
pub const PCI_DEVICE_ID_INTEL_NPK_CML_PCH_V: c_uint = 0xa3a6;
pub const PCI_DEVICE_ID_INTEL_NPK_RPL_S_CPU: c_uint = 0xa76f;
pub const PCI_DEVICE_ID_INTEL_NPK_LNL: c_uint = 0xa824;
pub const PCI_DEVICE_ID_INTEL_NPK_MTL_S_CPU: c_uint = 0xae24;
pub const PCI_DEVICE_ID_INTEL_NPK_NVL_P: c_uint = 0xd224;
pub const PCI_DEVICE_ID_INTEL_NPK_NVL_H: c_uint = 0xd324;
pub const PCI_DEVICE_ID_INTEL_NPK_NVL_S: c_uint = 0xd424;
pub const PCI_DEVICE_ID_INTEL_NPK_PTL_H: c_uint = 0xe324;
pub const PCI_DEVICE_ID_INTEL_NPK_PTL_PU: c_uint = 0xe424;
