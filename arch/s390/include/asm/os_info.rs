//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/os_info.h
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
// OS info memory interface
//
// Copyright IBM Corp. 2012
// Author(s): Michael Holzheu <holzheu@linux.vnet.ibm.com>
//

pub const OS_INFO_VERSION_MAJOR: c_int = 1;
pub const OS_INFO_VERSION_MINOR: c_int = 1;
pub const OS_INFO_MAGIC: c_uint = 0x4f53494e464f535aULL /* OSINFOSZ */;
pub const OS_INFO_VMCOREINFO: c_int = 0;
pub const OS_INFO_REIPL_BLOCK: c_int = 1;
pub const OS_INFO_FLAGS_ENTRY: c_int = 2;
pub const OS_INFO_RESERVED: c_int = 3;
pub const OS_INFO_IDENTITY_BASE: c_int = 4;
pub const OS_INFO_KASLR_OFFSET: c_int = 5;
pub const OS_INFO_KASLR_OFF_PHYS: c_int = 6;
pub const OS_INFO_VMEMMAP: c_int = 7;
pub const OS_INFO_AMODE31_START: c_int = 8;
pub const OS_INFO_AMODE31_END: c_int = 9;
pub const OS_INFO_IMAGE_START: c_int = 10;
pub const OS_INFO_IMAGE_END: c_int = 11;
pub const OS_INFO_IMAGE_PHYS: c_int = 12;
pub const OS_INFO_MAX: c_int = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_info_entry {
    pub addr: u64,
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct os_info {
    pub magic: u64,
    pub csum: u32,
    pub version_major: u16,
    pub version_minor: u16,
    pub crashkernel_addr: u64,
    pub crashkernel_size: u64,
    pub entry: [os_info_entry; OS_INFO_MAX],
    pub reserved: [u8; 3804],
    pub __packed: },
    pub os_info_init(void): c_void,
    pub len): *mut *mut void os_info_entry_add_data(int nr, void ptr, u64,
    pub val): void os_info_entry_add_val(int nr, u64,
    pub size): void os_info_crashkernel_add(unsigned long base, unsigned long,
    pub os_info): *mut u32 os_info_csum(struct os_info,

    pub size): *mut *mut void os_info_old_entry(int nr, unsigned long,
    pub size: c_ulong,
    pub &size): return (unsigned long)os_info_old_entry(nr,,

    pub NULL: return,

