//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_sriov_types.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// VFID - Virtual Function Identifier
// @n: VF number
//
// Helper macro to represent Virtual Function (VF) Identifier.
// VFID(0) is used as alias to the PFID that represents Physical Function.
//
// Note: According to PCI spec, SR-IOV VF's numbers are 1-based (VF1, VF2, ...).
//

//
// enum xe_sriov_mode - SR-IOV mode
// @XE_SRIOV_MODE_NONE: bare-metal mode (non-virtualized)
// @XE_SRIOV_MODE_PF: SR-IOV Physical Function (PF) mode
// @XE_SRIOV_MODE_VF: SR-IOV Virtual Function (VF) mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_sriov_mode {
//
// Note: We don't use default enum value 0 to allow catch any too early
// attempt of checking the SR-IOV mode prior to the actual mode probe.
//
    XE_SRIOV_MODE_NONE = 1,
    XE_SRIOV_MODE_PF,
    XE_SRIOV_MODE_VF,
}
