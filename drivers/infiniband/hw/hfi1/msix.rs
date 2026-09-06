//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/hfi1/msix.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2018 - 2020 Intel Corporation.
//

// MSIx interface
extern "C" {
    pub fn msix_initialize(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn msix_request_irqs(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn msix_clean_up_interrupts(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn msix_request_general_irq(dd: *mut hfi1_devdata) -> c_int;
}
extern "C" {
    pub fn msix_request_rcd_irq(rcd: *mut hfi1_ctxtdata) -> c_int;
}
extern "C" {
    pub fn msix_request_sdma_irq(sde: *mut sdma_engine) -> c_int;
}
extern "C" {
    pub fn msix_free_irq(dd: *mut hfi1_devdata, msix_intr: u8);
}
// Netdev interface
extern "C" {
    pub fn msix_netdev_synchronize_irq(dd: *mut hfi1_devdata);
}
extern "C" {
    pub fn msix_netdev_request_rcd_irq(rcd: *mut hfi1_ctxtdata) -> c_int;
}
