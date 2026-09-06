//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/uverbs_std_types.h
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
// Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved.
//

// Returns _id, or causes a compile error if _id is not a u32.
//
// The uobj APIs should only be used with the write based uAPI to access
// object IDs. The write API must use a u32 for the object handle, which is
// checked by this macro.
//

extern "C" {
    pub fn ERR_CAST(_arg: uobj) -> return;
}

//
// Tell the core code that the write() handler has completed
// initializing the object and that the core should commit or
// abort this object based upon the return code from the write()
// method. Similar to what uverbs_finalize_uobj_create() does for
// ioctl()
//
// ib_dev = attrs->context->device;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uflow_resources {
    pub max: usize,
    pub num: usize,
    pub collection_num: usize,
    pub counters_num: usize,
    pub counters: *mut ib_counters,
    pub collection: *mut ib_flow_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uflow_object {
    pub uobject: ib_uobject,
    pub resources: *mut ib_uflow_resources,
}

extern "C" {
    pub fn ib_uverbs_flow_resources_free(uflow_res: *mut ib_uflow_resources);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_api_object {
    pub type_attrs: *const uverbs_obj_type,
    pub type_class: *const uverbs_obj_type_class,
    pub disabled:1: u8,
    pub id: u32,
}
