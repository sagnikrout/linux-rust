//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/rdma_core.h
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


//
// Copyright (c) 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005, 2006 Cisco Systems.  All rights reserved.
// Copyright (c) 2005-2017 Mellanox Technologies. All rights reserved.
// Copyright (c) 2005 Voltaire, Inc. All rights reserved.
// Copyright (c) 2005 PathScale, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern "C" {
    pub fn uobj_destroy(uobj: *mut ib_uobject, attrs: *mut uverbs_attr_bundle) -> c_int;
}
//
// Get an ib_uobject that corresponds to the given id from ufile, assuming
// the object is from the given type. Lock it to the required access when
// applicable.
// This function could create (access == NEW), destroy (access == DESTROY)
// or unlock (access == READ || access == WRITE) objects if required.
// The action will be finalized only when uverbs_finalize_object or
// uverbs_finalize_objects are called.
//
extern "C" {
    pub fn uverbs_output_written(bundle: *const uverbs_attr_bundle, idx: usize) -> c_int;
}
extern "C" {
    pub fn setup_ufile_idr_uobject(ufile: *mut ib_uverbs_file);
}

//
// This is the runtime description of the uverbs API, used by the syscall
// machinery to validate and dispatch calls.
//
// Depending on ID the slot pointer in the radix tree points at one of these
// structs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_api_ioctl_method {
    pub attrs): *mut *mut int(__rcu handler)(struct uverbs_attr_bundle,
    pub UVERBS_API_ATTR_BKEY_LEN): DECLARE_BITMAP(attr_mandatory,,
    pub bundle_size: u16,
    pub use_stack:1: u8,
    pub driver_method:1: u8,
    pub disabled:1: u8,
    pub has_udata:1: u8,
    pub key_bitmap_len: u8,
    pub destroy_bkey: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_api_write_method {
    pub attrs): *mut *mut int (handler)(struct uverbs_attr_bundle,
    pub disabled:1: u8,
    pub is_ex:1: u8,
    pub has_udata:1: u8,
    pub has_resp:1: u8,
    pub req_size: u8,
    pub resp_size: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_api_attr {
    pub spec: uverbs_attr_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uverbs_api {
// radix tree contains struct uverbs_api_* pointers
    pub radix: radix_tree_root,
    pub driver_id: rdma_driver_id,
    pub num_write: c_uint,
    pub num_write_ex: c_uint,
    pub notsupp_method: uverbs_api_write_method,
    pub write_methods: *const uverbs_api_write_method,
    pub write_ex_methods: *const uverbs_api_write_method,
}

//
// Get an uverbs_api_object that corresponds to the given object_id.
// Note:
// -ENOMSG means that any object is allowed to match during lookup.
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMSG) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn uverbs_disassociate_api_pre(uverbs_dev: *mut ib_uverbs_device);
}
extern "C" {
    pub fn uverbs_disassociate_api(uapi: *mut uverbs_api);
}
extern "C" {
    pub fn uverbs_destroy_api(uapi: *mut uverbs_api);
}
extern "C" {
    pub fn uverbs_user_mmap_disassociate(ufile: *mut ib_uverbs_file);
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
