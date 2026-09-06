//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/sgx/encl.h
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
// Copyright(c) 2016-20 Intel Corporation.
//
// Contains the software defined data structures for enclaves.
//

// 'desc' bits holding the offset in the VA (version array) page.

// 'desc' bit marking that the page is being reclaimed.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_encl_page {
    pub desc: c_ulong,
    pub vm_max_prot_bits:8: c_ulong,
    pub type:16: sgx_page_type,
    pub epc_page: *mut sgx_epc_page,
    pub encl: *mut sgx_encl,
    pub va_page: *mut sgx_va_page,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgx_encl_flags {
    SGX_ENCL_IOCTL		= BIT(0),
    SGX_ENCL_DEBUG		= BIT(1),
    SGX_ENCL_CREATED	= BIT(2),
    SGX_ENCL_INITIALIZED	= BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_encl_mm {
    pub encl: *mut sgx_encl,
    pub mm: *mut mm_struct,
    pub list: list_head,
    pub mmu_notifier: mmu_notifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_encl {
    pub base: c_ulong,
    pub size: c_ulong,
    pub flags: c_ulong,
    pub page_cnt: c_uint,
    pub secs_child_cnt: c_uint,
    pub lock: mutex,
    pub page_array: xarray,
    pub secs: sgx_encl_page,
    pub attributes: c_ulong,
    pub attributes_mask: c_ulong,
    pub cpumask: cpumask_t,
    pub backing: *mut file,
    pub refcount: kref,
    pub va_pages: list_head,
    pub mm_list_version: c_ulong,
    pub mm_list: list_head,
    pub mm_lock: spinlock_t,
    pub srcu: srcu_struct,
}

pub const SGX_VA_SLOT_COUNT: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_va_page {
    pub epc_page: *mut sgx_epc_page,
    pub SGX_VA_SLOT_COUNT): DECLARE_BITMAP(slots,,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgx_backing {
    pub contents: *mut page,
    pub pcmd: *mut page,
    pub pcmd_offset: c_ulong,
}

// vma = result;
extern "C" {
    pub fn current_is_ksgxd() -> bool;
}
extern "C" {
    pub fn sgx_encl_release(ref: *mut kref);
}
extern "C" {
    pub fn sgx_encl_mm_add(encl: *mut sgx_encl, mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn sgx_encl_put_backing(backing: *mut sgx_backing);
}
extern "C" {
    pub fn sgx_zap_enclave_ptes(encl: *mut sgx_encl, addr: c_ulong);
}
extern "C" {
    pub fn sgx_alloc_va_slot(va_page: *mut sgx_va_page) -> c_uint;
}
extern "C" {
    pub fn sgx_free_va_slot(va_page: *mut sgx_va_page, offset: c_uint);
}
extern "C" {
    pub fn sgx_va_page_full(va_page: *mut sgx_va_page) -> bool;
}
extern "C" {
    pub fn sgx_encl_free_epc_page(page: *mut sgx_epc_page);
}
extern "C" {
    pub fn sgx_encl_shrink(encl: *mut sgx_encl, va_page: *mut sgx_va_page);
}
