//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pds/pds_common.h
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


// SPDX-License-Identifier: (GPL-2.0 OR Linux-OpenIB) OR BSD-2-Clause
// Copyright(c) 2023 Advanced Micro Devices, Inc.

// the device's internal addressing uses up to 52 bits
pub const PDS_CORE_ADDR_LEN: c_int = 52;

pub const PDS_PAGE_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_driver_type {
    PDS_DRIVER_LINUX   = 1,
    PDS_DRIVER_WIN     = 2,
    PDS_DRIVER_DPDK    = 3,
    PDS_DRIVER_FREEBSD = 4,
    PDS_DRIVER_IPXE    = 5,
    PDS_DRIVER_ESXI    = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pds_core_vif_types {
    PDS_DEV_TYPE_CORE	= 0,
    PDS_DEV_TYPE_VDPA	= 1,
    PDS_DEV_TYPE_VFIO	= 2,
    PDS_DEV_TYPE_ETH	= 3,
    PDS_DEV_TYPE_RDMA	= 4,
    PDS_DEV_TYPE_LM		= 5,
    PDS_DEV_TYPE_FWCTL	= 6,

// new ones added before this line
    PDS_DEV_TYPE_MAX	= 16   /* don't change - used in struct size */
}

extern "C" {
    pub fn pdsc_register_notify(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn pdsc_unregister_notify(nb: *mut notifier_block);
}
extern "C" {
    pub fn pds_client_register(pf: *mut pdsc, devname: *mut c_char) -> c_int;
}
extern "C" {
    pub fn pds_client_unregister(pf: *mut pdsc, client_id: u16) -> c_int;
}
