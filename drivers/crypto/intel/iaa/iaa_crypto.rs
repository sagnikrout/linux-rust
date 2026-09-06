//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/iaa/iaa_crypto.h
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
// Copyright(c) 2021 Intel Corporation. All rights rsvd.

pub const IAA_COMPLETION_TIMEOUT: c_int = 1000000;
pub const IAA_ANALYTICS_ERROR: c_uint = 0x0a;
pub const IAA_ERROR_DECOMP_BUF_OVERFLOW: c_uint = 0x0b;
pub const IAA_ERROR_COMP_BUF_OVERFLOW: c_uint = 0x19;
pub const IAA_ERROR_WATCHDOG_EXPIRED: c_uint = 0x24;
pub const IAA_COMP_MODES_MAX: c_int = 2;
pub const FIXED_HDR: c_uint = 0x2;
pub const FIXED_HDR_SIZE: c_int = 3;

// Representation of IAA workqueue
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iaa_wq {
    pub list: list_head,
    pub wq: *mut idxd_wq,
    pub ref: c_int,
    pub remove: bool,
    pub iaa_device: *mut iaa_device,
    pub comp_calls: core::sync::atomic::AtomicI64,
    pub comp_bytes: core::sync::atomic::AtomicI64,
    pub decomp_calls: core::sync::atomic::AtomicI64,
    pub decomp_bytes: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iaa_device_compression_mode {
    pub name: *const c_char,
    pub aecs_comp_table: *mut aecs_comp_table_record,
    pub aecs_comp_table_dma_addr: dma_addr_t,
}

// Representation of IAA device with wqs, populated by probe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iaa_device {
    pub list: list_head,
    pub idxd: *mut idxd_device,
    pub compression_modes: [*mut iaa_device_compression_mode; IAA_COMP_MODES_MAX],
    pub n_wq: c_int,
    pub wqs: list_head,
    pub comp_calls: core::sync::atomic::AtomicI64,
    pub comp_bytes: core::sync::atomic::AtomicI64,
    pub decomp_calls: core::sync::atomic::AtomicI64,
    pub decomp_bytes: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wq_table_entry {
    pub wqs: *mut idxd_wq,
    pub max_wqs: c_int,
    pub n_wqs: c_int,
    pub cur_wq: c_int,
}

pub const IAA_AECS_ALIGN: c_int = 32;
//
// Analytics Engine Configuration and State (AECS) contains parameters and
// internal state of the analytics engine.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aecs_comp_table_record {
    pub crc: u32,
    pub xor_checksum: u32,
    pub reserved0: [u32; 5],
    pub num_output_accum_bits: u32,
    pub output_accum: [u8; 256],
    pub ll_sym: [u32; 286],
    pub reserved1: u32,
    pub reserved2: u32,
    pub d_sym: [u32; 30],
    pub reserved_padding: [u32; 2],
    pub __packed: },
    pub iaa_aecs_init_fixed(void): c_int,
    pub iaa_aecs_cleanup_fixed(void): c_void,
    pub mode): *mut *mut typedef int (iaa_dev_comp_init_fn_t) (struct iaa_device_compression_mode,
    pub mode): *mut *mut typedef int (iaa_dev_comp_free_fn_t) (struct iaa_device_compression_mode,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iaa_compression_mode {
    pub name: *const c_char,
    pub ll_table: *mut u32,
    pub ll_table_size: c_int,
    pub d_table: *mut u32,
    pub d_table_size: c_int,
    pub init: iaa_dev_comp_init_fn_t,
    pub free: iaa_dev_comp_free_fn_t,
}

extern "C" {
    pub fn remove_iaa_compression_mode(name: *const c_char);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iaa_mode {
    IAA_MODE_FIXED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iaa_compression_ctx {
    pub mode: iaa_mode,
    pub verify_compress: bool,
    pub async_mode: bool,
    pub use_irq: bool,
}
