//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nvram.h
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
// NVRAM definitions and access functions.
//

//
// Set oops header version to distinguish between old and new format header.
// lnx,oops-log partition max size is 4000, header version > 4000 will
// help in identifying new header.
//
pub const OOPS_HDR_VERSION: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct err_log_info {
    pub error_type: __be32,
    pub seq_num: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvram_os_partition {
    pub name: *const c_char,
    pub /: *mut *mut int req_size; / desired size, in bytes,
    pub /: *mut *mut int min_size; / minimum acceptable size (0 means req_size),
    pub /: *mut *mut long size; / size of data portion (excluding err_log_info),
    pub /: *mut *mut long index; / offset of data portion of partition,
    pub /: *mut *mut bool os_partition; / partition initialized by OS, not FW,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oops_log_info {
    pub version: __be16,
    pub report_length: __be16,
    pub timestamp: __be64,
    pub __attribute__((packed)): },
    pub oops_log_partition: extern struct nvram_os_partition,

    pub rtas_log_partition: extern struct nvram_os_partition,
    pub err_seq): unsigned int err_type, unsigned int,
    pub err_seq): *mut *mut unsigned int  err_type, unsigned int,
    pub nvram_clear_error_log(void): extern int,
    pub pSeries_nvram_init(void): extern int,

    pub mmio_nvram_init(void): extern int,

    pub -ENODEV: return,

    pub nvram_scan_partitions(void): extern int __init,
    pub min_size): int req_size, int,
    pub exceptions[]): *const c_char,
    pub data_index): extern int nvram_get_partition_size(loff_t,
    pub out_size): *const *const extern loff_t nvram_find_partition(char name, int sig, int,
// Return partition offset in nvram
    pub partition): extern int pmac_get_partition(int,
// Direct access to XPRAM on PowerMacs
    pub xpaddr): extern u8 pmac_xpram_read(int,
    pub data): extern void pmac_xpram_write(int xpaddr, u8,
// Initialize NVRAM OS partition
    pub part): *mut extern int __init nvram_init_os_partition(struct nvram_os_partition,
// Initialize NVRAM oops partition
    pub rtas_partition_exists): extern void __init nvram_init_oops_partition(int,
// Read a NVRAM partition
    pub error_log_cnt): *mut c_uint,
// Write to NVRAM OS partition
    pub error_log_cnt): c_uint,
