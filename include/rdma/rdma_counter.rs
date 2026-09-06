//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/rdma_counter.h
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
// Copyright (c) 2019 Mellanox Technologies. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auto_mode_param {
    pub qp_type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_counter_mode {
    pub mode: rdma_nl_counter_mode,
    pub mask: rdma_nl_counter_mask,
    pub param: auto_mode_param,
    pub bind_opcnt: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_port_counter {
    pub mode: rdma_counter_mode,
    pub hstats: *mut rdma_hw_stats,
    pub num_counters: c_uint,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_counter {
    pub res: rdma_restrack_entry,
    pub device: *mut ib_device,
    pub id: u32,
    pub kref: kref,
    pub mode: rdma_counter_mode,
    pub lock: mutex,
    pub stats: *mut rdma_hw_stats,
    pub port: u32,
}

extern "C" {
    pub fn rdma_counter_init(dev: *mut ib_device);
}
extern "C" {
    pub fn rdma_counter_release(dev: *mut ib_device);
}
extern "C" {
    pub fn rdma_counter_bind_qp_auto(qp: *mut ib_qp, port: u32) -> c_int;
}
extern "C" {
    pub fn rdma_counter_unbind_qp(qp: *mut ib_qp, port: u32, force: bool) -> c_int;
}
extern "C" {
    pub fn rdma_counter_query_stats(counter: *mut rdma_counter) -> c_int;
}
extern "C" {
    pub fn rdma_counter_get_hwstat_value(dev: *mut ib_device, port: u32, index: u32) -> u64;
}
