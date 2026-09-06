//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_irq.h
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
// Copyright (C) 2023, Intel Corporation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_irq_entry {
    pub index: c_uint,
    pub /: *mut *mut bool dynamic; / allocation type flag,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_irq_tracker {
    pub entries: xarray,
    pub /: *mut *mut u16 num_entries; / total vectors available,
    pub /: *mut *mut u16 num_static; / preallocated entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_virt_irq_tracker {
    pub /: *mut *mut *mut unsigned long bm; / bitmap to track irq usage,
    pub num_entries: u32,
// First MSIX vector used by SR-IOV VFs. Calculated by subtracting the
// number of MSIX vectors needed for all SR-IOV VFs from the number of
// MSIX vectors allowed on this PF.
//
    pub base: u32,
}

extern "C" {
    pub fn ice_init_interrupt_scheme(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_clear_interrupt_scheme(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_alloc_irq(pf: *mut ice_pf, dyn_only: bool) -> msi_map;
}
extern "C" {
    pub fn ice_free_irq(pf: *mut ice_pf, map: msi_map);
}
extern "C" {
    pub fn ice_virt_get_irqs(pf: *mut ice_pf, needed: u32) -> c_int;
}
extern "C" {
    pub fn ice_virt_free_irqs(pf: *mut ice_pf, index: u32, irqs: u32);
}
