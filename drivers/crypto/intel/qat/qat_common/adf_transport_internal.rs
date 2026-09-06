//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_transport_internal.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_etr_ring_debug_entry {
    pub ring_name: [c_char; ADF_CFG_MAX_KEY_LEN_IN_BYTES],
    pub debug: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_etr_ring_data {
    pub base_addr: *mut c_void,
    pub inflights: *mut core::sync::atomic::AtomicI32,
    pub callback: adf_callback_fn,
    pub bank: *mut adf_etr_bank_data,
    pub dma_addr: dma_addr_t,
    pub ring_debug: *mut adf_etr_ring_debug_entry,
    pub /: *mut *mut spinlock_t lock; / protects ring data struct,
    pub head: u16,
    pub tail: u16,
    pub threshold: u32,
    pub ring_number: u8,
    pub ring_size: u8,
    pub msg_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_etr_bank_data {
    pub rings: *mut adf_etr_ring_data,
    pub resp_handler: tasklet_struct,
    pub csr_addr: *mut void __iomem,
    pub irq_coalesc_timer: u32,
    pub bank_number: u32,
    pub ring_mask: u16,
    pub irq_mask: u16,
    pub /: *mut *mut spinlock_t lock; / protects bank data struct,
    pub accel_dev: *mut adf_accel_dev,
    pub bank_debug_dir: *mut dentry,
    pub bank_debug_cfg: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adf_etr_data {
    pub banks: *mut adf_etr_bank_data,
    pub debug: *mut dentry,
}

extern "C" {
    pub fn adf_response_handler(bank_addr: uintptr_t);
}

extern "C" {
    pub fn adf_bank_debugfs_add(bank: *mut adf_etr_bank_data) -> c_int;
}
extern "C" {
    pub fn adf_bank_debugfs_rm(bank: *mut adf_etr_bank_data);
}
extern "C" {
    pub fn adf_ring_debugfs_add(ring: *mut adf_etr_ring_data, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn adf_ring_debugfs_rm(ring: *mut adf_etr_ring_data);
}

