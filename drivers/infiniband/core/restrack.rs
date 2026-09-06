//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/restrack.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2017-2019 Mellanox Technologies. All rights reserved.
//

//
// struct rdma_restrack_root - main resource tracking management
// entity, per-device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_restrack_root {
//
// @xa: Array of XArray structure to hold restrack entries.
//
    pub xa: xarray,
//
// @next_id: Next ID to support cyclic allocation
//
    pub next_id: u32,
}

extern "C" {
    pub fn rdma_restrack_init(dev: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn rdma_restrack_clean(dev: *mut ib_device);
}
extern "C" {
    pub fn rdma_restrack_add(res: *mut rdma_restrack_entry);
}
extern "C" {
    pub fn rdma_restrack_abort_del(res: *mut rdma_restrack_entry);
}
extern "C" {
    pub fn rdma_restrack_del(res: *mut rdma_restrack_entry);
}
extern "C" {
    pub fn rdma_restrack_sync(res: *mut rdma_restrack_entry);
}
extern "C" {
    pub fn rdma_restrack_begin_del(res: *mut rdma_restrack_entry);
}
extern "C" {
    pub fn rdma_restrack_commit_del(res: *mut rdma_restrack_entry);
}
