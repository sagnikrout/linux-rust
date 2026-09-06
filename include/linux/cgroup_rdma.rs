//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cgroup_rdma.h
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
// Copyright (C) 2016 Parav Pandit <pandit.parav@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdmacg_resource_type {
    RDMACG_RESOURCE_HCA_HANDLE,
    RDMACG_RESOURCE_HCA_OBJECT,
    RDMACG_RESOURCE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_cgroup {
    pub css: cgroup_subsys_state,
//
// head to keep track of all resource pools
// that belongs to this cgroup.
//
    pub rpools: list_head,
// Handles for rdma.events[.local]
    pub events_file: cgroup_file,
    pub events_local_file: cgroup_file,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdmacg_device {
    pub dev_node: list_head,
    pub rpools: list_head,
    pub name: *mut c_char,
    pub index: u32,
}

//
// APIs for RDMA/IB stack to publish when a device wants to
// participate in resource accounting
//
extern "C" {
    pub fn rdmacg_register_device(device: *mut rdmacg_device);
}
extern "C" {
    pub fn rdmacg_unregister_device(device: *mut rdmacg_device);
}
// APIs for RDMA/IB stack to charge/uncharge pool specific resources

