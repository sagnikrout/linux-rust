//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/cadence/pcie-cadence-host-common.h
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
// Cadence PCIe Host controller driver.
//
// Copyright (c) 2017 Cadence
// Author: Cyrille Pitchen <cyrille.pitchen@free-electrons.com>
//

extern "C" {
    pub fn bool(: *mut *mut cdns_pcie_linkup_func)(struct cdns_pcie) -> typedef;
}
extern "C" {
    pub fn cdns_pcie_host_training_complete(pcie: *mut cdns_pcie) -> c_int;
}
extern "C" {
    pub fn cdns_pcie_retrain(pcie: *mut cdns_pcie, pcie_linkup_func: cdns_pcie_linkup_func) -> c_int;
}
