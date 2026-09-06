//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/emulex/benet/be_roce.h
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
// Copyright (C) 2005 - 2016 Broadcom
// All rights reserved.
//
// Contact Information:
// linux-drivers@emulex.com
//
// Emulex
// 3333 Susan Street
// Costa Mesa, CA 92626
//

pub const BE_ROCE_ABI_VERSION: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum be_interrupt_mode {
    BE_INTERRUPT_MODE_MSIX	= 0,
    BE_INTERRUPT_MODE_INTX	= 1,
    BE_INTERRUPT_MODE_MSI	= 2,
}

pub const MAX_MSIX_VECTORS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct be_dev_info {
    pub db: *mut u8 __iomem,
    pub unmapped_db: u64,
    pub db_page_size: u32,
    pub db_total_size: u32,
    pub dpp_unmapped_addr: u64,
    pub dpp_unmapped_len: u32,
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub mac_addr: [u8; ETH_ALEN],
    pub dev_family: u32,
    pub intr_mode: be_interrupt_mode,
    pub num_vectors: c_int,
    pub start_vector: c_int,
    pub vector_list: [u32; MAX_MSIX_VECTORS],
    pub msix: },
}

// ocrdma driver register's the callback functions with nic driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocrdma_driver {
    pub name: [c_uchar; 32],
    pub be_abi_version: u32,
    pub dev_info): *mut *mut *mut ocrdma_dev (add) (be_dev_info,
    pub ): *mut *mut void (remove) (struct ocrdma_dev,
    pub new_state): *mut *mut *mut void (state_change_handler) (struct ocrdma_dev , u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum be_roce_event {
    BE_DEV_SHUTDOWN = 2
}

// APIs for RoCE driver to register callback handlers,
// which will be invoked when device is added, removed, ifup, ifdown
//
extern "C" {
    pub fn be_roce_register_driver(drv: *mut ocrdma_driver) -> c_int;
}
extern "C" {
    pub fn be_roce_unregister_driver(drv: *mut ocrdma_driver);
}
// API for RoCE driver to issue mailbox commands
