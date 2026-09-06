//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/pci/pds/vfio_dev.h
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
// Copyright(c) 2023 Advanced Micro Devices, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pds_vfio_pci_device {
    pub vfio_coredev: vfio_pci_core_device,
    pub save_file: *mut pds_vfio_lm_file,
    pub restore_file: *mut pds_vfio_lm_file,
    pub dirty: pds_vfio_dirty,
    pub /: *mut *mut mutex state_mutex; / protect migration state,
    pub state: vfio_device_mig_state,
    pub nb: notifier_block,
    pub vf_id: c_int,
    pub client_id: u16,
}
