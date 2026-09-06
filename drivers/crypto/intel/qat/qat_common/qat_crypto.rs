//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/qat_crypto.h
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
pub struct qat_crypto_instance {
    pub sym_tx: *mut adf_etr_ring_data,
    pub sym_rx: *mut adf_etr_ring_data,
    pub pke_tx: *mut adf_etr_ring_data,
    pub pke_rx: *mut adf_etr_ring_data,
    pub accel_dev: *mut adf_accel_dev,
    pub list: list_head,
    pub state: c_ulong,
    pub id: c_int,
    pub refctr: core::sync::atomic::AtomicI32,
    pub backlog: qat_instance_backlog,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_crypto_request {
    pub req: icp_qat_fw_la_bulk_req,
    pub aead_ctx: *mut qat_alg_aead_ctx,
    pub skcipher_ctx: *mut qat_alg_skcipher_ctx,
}
