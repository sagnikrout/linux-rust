//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/intel/pmt/class.h
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

// PMT access types
pub const ACCESS_BARID: c_int = 2;
pub const ACCESS_LOCAL: c_int = 3;
// PMT discovery base address/offset register layout

#[repr(C)]
#[derive(Copy, Clone)]
pub struct telem_endpoint {
    pub dev: *mut device,
    pub header: telem_header,
    pub cb: *mut pmt_callbacks,
    pub base: *mut void __iomem,
    pub present: bool,
    pub kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pmt_header {
    pub base_offset: u32,
    pub size: u32,
    pub guid: u32,
    pub access_type: u8,
    pub telem_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pmt_entry {
    pub ep: *mut telem_endpoint,
    pub pcidev: *mut pci_dev,
    pub header: intel_pmt_header,
    pub disc_header: [u32; PMT_DISC_DWORDS],
    pub pmt_bin_attr: bin_attribute,
    pub attr_grp: *const attribute_group,
    pub kobj: *mut kobject,
    pub disc_table: *mut void __iomem,
    pub base: *mut void __iomem,
    pub cb: *mut pmt_callbacks,
    pub base_addr: c_ulong,
    pub size: usize,
    pub feature_flags: u64,
    pub guid: u32,
    pub /: *mut *mut u32 num_rmids; / Number of Resource Monitoring IDs,
    pub devid: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pmt_namespace {
    pub name: *const c_char,
    pub xa: *mut xarray,
    pub dev): *mut device,
    pub entry): *mut intel_pmt_entry,
    pub entry): *mut intel_pmt_entry,
    pub entry): *mut intel_pmt_entry,
}

extern "C" {
    pub fn intel_pmt_is_early_client_hw(dev: *mut device) -> bool;
}

extern "C" {
    pub fn intel_pmt_get_features(entry: *mut intel_pmt_entry);
}

