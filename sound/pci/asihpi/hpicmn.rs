//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/asihpi/hpicmn.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// a function that takes an adapter obj and returns an int
extern "C" {
    pub fn adapter_int_func(pao: *mut hpi_adapter_obj, message: u32) -> typedef int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_adapter_obj {
    pub /: *mut *mut hpi_pci pci; / PCI info - bus#,dev#,address etc,
    pub /: *mut *mut u16 type; / 0x6644 == ASI6644 etc,
    pub index: u16,
    pub dsp_lock: hpios_spinlock,
    pub dsp_crashed: u16,
    pub has_control_cache: u16,
    pub priv: *mut c_void,
    pub irq_query_and_clear: *mut adapter_int_func,
    pub instream_host_buffer_status: *mut hpi_hostbuffer_status,
    pub outstream_host_buffer_status: *mut hpi_hostbuffer_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpi_control_cache {
// indicates whether the structures are initialized
    pub init: u16,
    pub adap_idx: u16,
    pub control_count: u32,
    pub cache_size_in_bytes: u32,
// pointer to DSP's control cache.
    pub p_cache: *mut u8,
// pointer to allocated memory of lookup pointers.
    pub __counted_by(control_count): *mut *mut hpi_control_cache_info p_info[],
}

extern "C" {
    pub fn hpi_add_adapter(pao: *mut hpi_adapter_obj) -> u16;
}
extern "C" {
    pub fn hpi_delete_adapter(pao: *mut hpi_adapter_obj);
}
extern "C" {
    pub fn hpi_free_control_cache(p_cache: *mut hpi_control_cache);
}
// pC, struct hpi_message *phm, struct hpi_response *phr);
extern "C" {
    pub fn hpi_validate_response(phm: *mut hpi_message, phr: *mut hpi_response) -> u16;
}
