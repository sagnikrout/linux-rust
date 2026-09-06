//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firmware/qcom/qcom_tzmem.h
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
// Copyright (C) 2023-2024 Linaro Ltd.
//

//
// enum qcom_tzmem_policy - Policy for pool growth.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qcom_tzmem_policy {
//
// @QCOM_TZMEM_POLICY_STATIC: Static pool,
// never grow above initial size.
//
    QCOM_TZMEM_POLICY_STATIC = 1,
//
// @QCOM_TZMEM_POLICY_MULTIPLIER: When out of memory,
// add increment * current size of memory.
//
    QCOM_TZMEM_POLICY_MULTIPLIER,
//
// @QCOM_TZMEM_POLICY_ON_DEMAND: When out of memory
// add as much as is needed until max_size.
//
    QCOM_TZMEM_POLICY_ON_DEMAND,
}

//
// struct qcom_tzmem_pool_config - TZ memory pool configuration.
// @initial_size: Number of bytes to allocate for the pool during its creation.
// @policy: Pool size growth policy.
// @increment: Used with policies that allow pool growth.
// @max_size: Size above which the pool will never grow.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_tzmem_pool_config {
    pub initial_size: usize,
    pub policy: qcom_tzmem_policy,
    pub increment: usize,
    pub max_size: usize,
}

extern "C" {
    pub fn qcom_tzmem_pool_free(pool: *mut qcom_tzmem_pool);
}
extern "C" {
    pub fn qcom_tzmem_free(ptr: *mut c_void);
}
extern "C" {
    pub fn qcom_tzmem_to_phys(ptr: *mut c_void) -> phys_addr_t;
}

extern "C" {
    pub fn qcom_tzmem_shm_bridge_create(paddr: phys_addr_t, size: usize, handle: *mut u64) -> c_int;
}
extern "C" {
    pub fn qcom_tzmem_shm_bridge_delete(handle: u64);
}

