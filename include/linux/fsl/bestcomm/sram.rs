//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl/bestcomm/sram.h
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


//
// Handling of a sram zone for bestcomm
//
// Copyright (C) 2007 Sylvain Munaut <tnt@246tNt.com>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// Structure used internally
// The internals are here for the inline functions
// sake, certainly not for the user to mess with !
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcom_sram {
    pub base_phys: phys_addr_t,
    pub base_virt: *mut c_void,
    pub size: c_uint,
    pub rh: *mut rh_info_t,
    pub lock: spinlock_t,
}

// Public API
extern "C" {
    pub fn bcom_sram_init(sram_node: *mut device_node, owner: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bcom_sram_cleanup();
}
extern "C" {
    pub fn bcom_sram_alloc(size: c_int, align: c_int, phys: *mut phys_addr_t) -> *mut c_void;
}
extern "C" {
    pub fn bcom_sram_free(ptr: *mut c_void);
}
