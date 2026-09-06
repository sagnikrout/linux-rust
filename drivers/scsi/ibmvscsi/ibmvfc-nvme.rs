//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/ibmvscsi/ibmvfc-nvme.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// ibmvfc-nvme.h -- IBM Power Virtual Fibre Channel NVMeoF HBA driver
//
// Written By: Tyrel Datwyler <tyreld@linux.ibm.com>, IBM Corporation
//
// Copyright (C) IBM Corporation, 2026
//

pub const IBMVFC_NVME: c_int = 0;
pub const IBMVFC_NVME_HW_QUEUES: c_int = 8;
pub const IBMVFC_MAX_NVME_QUEUES: c_int = 16;
pub const IBMVFC_NVME_CHANNELS: c_int = 8;
pub const IBMVFC_FC4_LS_TIMEOUT: c_int = 15;
pub const IBMVFC_FC4_LS_CANCEL_TIMEOUT: c_int = 45;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ibmvfc_nvme_qhandle {
    pub qidx: c_uint,
    pub cpu_id: u16,
    pub index: c_ulong,
    pub queue: *mut ibmvfc_queue,
}

extern "C" {
    pub fn ibmvfc_nvme_register_remoteport(tgt: *mut ibmvfc_target) -> c_int;
}
extern "C" {
    pub fn ibmvfc_nvme_unregister_remoteport(tgt: *mut ibmvfc_target);
}
extern "C" {
    pub fn ibmvfc_nvme_register(vhost: *mut ibmvfc_host) -> c_int;
}
extern "C" {
    pub fn ibmvfc_nvme_unregister(vhost: *mut ibmvfc_host);
}
