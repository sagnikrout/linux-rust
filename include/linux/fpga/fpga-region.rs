//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fpga/fpga-region.h
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
// struct fpga_region_info - collection of parameters an FPGA Region
// @mgr: fpga region manager
// @compat_id: FPGA region id for compatibility check.
// @priv: fpga region private data
// @get_bridges: optional function to get bridges to a list
//
// fpga_region_info contains parameters for the register_full function.
// These are separated into an info structure because they some are optional
// others could be added to in the future. The info structure facilitates
// maintaining a stable API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_region_info {
    pub mgr: *mut fpga_manager,
    pub compat_id: *mut fpga_compat_id,
    pub priv: *mut c_void,
    pub region): *mut *mut int (get_bridges)(struct fpga_region,
}

//
// struct fpga_region - FPGA Region structure
// @dev: FPGA Region device
// @mutex: enforces exclusive reference to region
// @bridge_list: list of FPGA bridges specified in region
// @mgr: FPGA manager
// @info: FPGA image info
// @compat_id: FPGA region id for compatibility check.
// @ops_owner: module containing the get_bridges function
// @priv: private data
// @get_bridges: optional function to get bridges to a list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_region {
    pub dev: device,
    pub /: *mut *mut mutex mutex; / for exclusive reference to region,
    pub bridge_list: list_head,
    pub mgr: *mut fpga_manager,
    pub info: *mut fpga_image_info,
    pub compat_id: *mut fpga_compat_id,
    pub ops_owner: *mut module,
    pub priv: *mut c_void,
    pub region): *mut *mut int (get_bridges)(struct fpga_region,
}

extern "C" {
    pub fn fpga_region_program_fpga(region: *mut fpga_region) -> c_int;
}

extern "C" {
    pub fn fpga_region_unregister(region: *mut fpga_region);
}
