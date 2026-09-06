//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/sgx/sgx.h
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

pub const SGX_MAX_EPC_SECTIONS: c_int = 8;
pub const SGX_EEXTEND_BLOCK_SIZE: c_int = 256;
pub const SGX_NR_TO_SCAN: c_int = 16;
pub const SGX_NR_LOW_PAGES: c_int = 32;
pub const SGX_NR_HIGH_PAGES: c_int = 64;
// Pages, which are being tracked by the page reclaimer.

// Pages on free list

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_epc_page {
    pub section: c_uint,
    pub flags: u16,
    pub poison: u16,
    pub owner: *mut sgx_encl_page,
    pub list: list_head,
}

//
// Contains the tracking data for NUMA nodes having EPC pages. Most importantly,
// the free page list local to the node is stored here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_numa_node {
    pub free_page_list: list_head,
    pub sgx_poison_page_list: list_head,
    pub size: c_ulong,
    pub lock: spinlock_t,
}

//
// The firmware can define multiple chunks of EPC to the different areas of the
// physical memory e.g. for memory areas of the each node. This structure is
// used to store EPC pages for one EPC section and virtual memory area where
// the pages have been mapped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_epc_section {
    pub phys_addr: c_ulong,
    pub virt_addr: *mut c_void,
    pub pages: *mut sgx_epc_page,
    pub node: *mut sgx_numa_node,
}

extern "C" {
    pub fn sgx_free_epc_page(page: *mut sgx_epc_page);
}
extern "C" {
    pub fn sgx_reclaim_direct();
}
extern "C" {
    pub fn sgx_mark_page_reclaimable(page: *mut sgx_epc_page);
}
extern "C" {
    pub fn sgx_unmark_page_reclaimable(page: *mut sgx_epc_page) -> c_int;
}
extern "C" {
    pub fn sgx_ipi_cb(info: *mut c_void);
}

extern "C" {
    pub fn sgx_vepc_init() -> int __init;
}

extern "C" {
    pub fn sgx_inc_usage_count() -> c_int;
}
extern "C" {
    pub fn sgx_dec_usage_count();
}
extern "C" {
    pub fn sgx_update_lepubkeyhash(lepubkeyhash: *mut u64);
}
