//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/qat_algs_send.h
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
// Copyright(c) 2022 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_instance_backlog {
    pub list: list_head,
    pub /: *mut *mut spinlock_t lock; / protects backlog list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_alg_req {
    pub fw_req: *mut u32,
    pub tx_ring: *mut adf_etr_ring_data,
    pub base: *mut crypto_async_request,
    pub list: list_head,
    pub backlog: *mut qat_instance_backlog,
}

extern "C" {
    pub fn qat_alg_send_message(req: *mut qat_alg_req) -> c_int;
}
extern "C" {
    pub fn qat_alg_send_backlog(backlog: *mut qat_instance_backlog);
}
