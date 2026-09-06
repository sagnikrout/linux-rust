//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/opa_smi.h
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
// Copyright (c) 2014 Intel Corporation.  All rights reserved.
//

pub const OPA_SMP_LID_DATA_SIZE: c_int = 2016;
pub const OPA_SMP_DR_DATA_SIZE: c_int = 1872;
pub const OPA_SMP_MAX_PATH_HOPS: c_int = 64;
pub const OPA_MAX_VLS: c_int = 32;
pub const OPA_MAX_SLS: c_int = 32;
pub const OPA_MAX_SCS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_smp {
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub status: __be16,
    pub hop_ptr: u8,
    pub hop_cnt: u8,
    pub tid: __be64,
    pub attr_id: __be16,
    pub resv: __be16,
    pub attr_mod: __be32,
    pub mkey: __be64,
    pub data: [u8; OPA_SMP_LID_DATA_SIZE],
    pub lid: },
    pub dr_slid: __be32,
    pub dr_dlid: __be32,
    pub initial_path: [u8; OPA_SMP_MAX_PATH_HOPS],
    pub return_path: [u8; OPA_SMP_MAX_PATH_HOPS],
    pub reserved: [u8; 8],
    pub data: [u8; OPA_SMP_DR_DATA_SIZE],
    pub dr: },
    pub route: },
    pub __packed: },
// Subnet management attributes
// ...

// ...

// ...

// ...
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_node_description {
    pub data: [u8; 64],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_node_info {
    pub base_version: u8,
    pub class_version: u8,
    pub node_type: u8,
    pub num_ports: u8,
    pub reserved: __be32,
    pub system_image_guid: __be64,
    pub node_guid: __be64,
    pub port_guid: __be64,
    pub partition_cap: __be16,
    pub device_id: __be16,
    pub revision: __be32,
    pub local_port_num: u8,
    pub /: *mut *mut u8 vendor_id[3]; / network byte order,
    pub __packed: },
pub const OPA_PARTITION_TABLE_BLK_SIZE: c_int = 32;
    pub )smp): *mut return ib_get_smp_direction((struct ib_smp,
    pub smp->route.dr.data: return,
    pub smp->route.lid.data: return,
    pub sizeof(smp->route.dr.data): return,
    pub sizeof(smp->route.lid.data): return,
    pub sizeof(smp->route.dr.data): *mut *mut return sizeof(smp) -,
    pub sizeof(smp->route.lid.data): *mut *mut return sizeof(smp) -,
