//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/uverbs.h
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
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
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

//
// Our lifetime rules for these structs are the following:
//
// struct ib_uverbs_device: One reference is held by the module and
// released in ib_uverbs_remove_one().  Another reference is taken by
// ib_uverbs_open() each time the character special file is opened,
// and released in ib_uverbs_release_file() when the file is released.
//
// struct ib_uverbs_file: One reference is held by the VFS and
// released when the file is closed.  Another reference is taken when
// an asynchronous event queue file is created and released when the
// event file is closed.
//
// struct ib_uverbs_event_queue: Base structure for
// struct ib_uverbs_async_event_file and struct ib_uverbs_completion_event_file.
// One reference is held by the VFS and released when the file is closed.
// For asynchronous event files, another reference is held by the corresponding
// main context file and released when that file is closed.  For completion
// event files, a reference is taken when a CQ is created that uses the file,
// and released when the CQ is destroyed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_device {
    pub refcount: refcount_t,
    pub num_comp_vectors: u32,
    pub comp: completion,
    pub dev: device,
// First group for device attributes, NULL terminated array
    pub groups: [*const attribute_group; 2],
    pub ib_dev: *mut ib_device __rcu,
    pub devnum: c_int,
    pub cdev: cdev,
    pub xrcd_tree: rb_root,
    pub xrcd_tree_mutex: mutex,
    pub disassociate_srcu: srcu_struct,
    pub /: *mut *mut mutex lists_mutex; / protect lists,
    pub uverbs_file_list: list_head,
    pub uapi: *mut uverbs_api,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_event_queue {
    pub lock: spinlock_t,
    pub is_closed: c_int,
    pub poll_wait: wait_queue_head_t,
    pub async_queue: *mut fasync_struct,
    pub event_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_async_event_file {
    pub uobj: ib_uobject,
    pub ev_queue: ib_uverbs_event_queue,
    pub event_handler: ib_event_handler,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_completion_event_file {
    pub uobj: ib_uobject,
    pub ev_queue: ib_uverbs_event_queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_dmabuf_file {
    pub uobj: ib_uobject,
    pub dmabuf: *mut dma_buf,
    pub dmabufs_elm: list_head,
    pub mmap_entry: *mut rdma_user_mmap_entry,
    pub phys_vec: phys_vec,
    pub provider: *mut p2pdma_provider,
    pub kref: kref,
    pub comp: completion,
    pub :1: u8 revoked,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_event {
    pub async: ib_uverbs_async_event_desc,
    pub comp: ib_uverbs_comp_event_desc,
    pub desc: },
    pub list: list_head,
    pub obj_list: list_head,
    pub counter: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_mcast_entry {
    pub list: list_head,
    pub gid: ib_gid,
    pub lid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uevent_object {
    pub uobject: ib_uobject,
    pub event_file: *mut ib_uverbs_async_event_file,
// List member for ib_uverbs_async_event_file list
    pub event_list: list_head,
    pub events_reported: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uxrcd_object {
    pub uobject: ib_uobject,
    pub refcnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_usrq_object {
    pub uevent: ib_uevent_object,
    pub uxrcd: *mut ib_uxrcd_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uqp_object {
    pub uevent: ib_uevent_object,
// lock for mcast list
    pub mcast_lock: mutex,
    pub mcast_list: list_head,
    pub uxrcd: *mut ib_uxrcd_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uwq_object {
    pub uevent: ib_uevent_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ucq_object {
    pub uevent: ib_uevent_object,
    pub comp_list: list_head,
    pub comp_events_reported: u32,
}

extern "C" {
    pub fn ib_uverbs_init_event_queue(ev_queue: *mut ib_uverbs_event_queue);
}
extern "C" {
    pub fn ib_uverbs_init_async_event_file(ev_file: *mut ib_uverbs_async_event_file);
}
extern "C" {
    pub fn ib_uverbs_free_event_queue(event_queue: *mut ib_uverbs_event_queue);
}
extern "C" {
    pub fn ib_uverbs_flow_resources_free(uflow_res: *mut ib_uflow_resources);
}
extern "C" {
    pub fn ib_alloc_ucontext(attrs: *mut uverbs_attr_bundle) -> c_int;
}
extern "C" {
    pub fn ib_init_ucontext(attrs: *mut uverbs_attr_bundle) -> c_int;
}
extern "C" {
    pub fn ib_uverbs_release_uevent(uobj: *mut ib_uevent_object);
}
extern "C" {
    pub fn ib_uverbs_release_file(ref: *mut kref);
}
extern "C" {
    pub fn ib_uverbs_comp_handler(cq: *mut ib_cq, cq_context: *mut c_void);
}
extern "C" {
    pub fn ib_uverbs_cq_event_handler(event: *mut ib_event, context_ptr: *mut c_void);
}
extern "C" {
    pub fn ib_uverbs_qp_event_handler(event: *mut ib_event, context_ptr: *mut c_void);
}
extern "C" {
    pub fn ib_uverbs_wq_event_handler(event: *mut ib_event, context_ptr: *mut c_void);
}
extern "C" {
    pub fn ib_uverbs_srq_event_handler(event: *mut ib_event, context_ptr: *mut c_void);
}
extern "C" {
    pub fn uverbs_dealloc_mw(mw: *mut ib_mw) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bundle_alloc_head {
    pub next: *mut bundle_alloc_head,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bundle_priv {
// Must be first
    pub alloc_head: bundle_alloc_head_hdr,
    pub allocated_mem: *mut bundle_alloc_head,
    pub internal_avail: usize,
    pub internal_used: usize,
    pub radix: *mut radix_tree_root,
    pub radix_slots: *mut void __rcu,
    pub radix_slots_len: c_ulong,
    pub method_key: u32,
    pub user_attrs: *mut ib_uverbs_attr __user,
    pub uattrs: *mut ib_uverbs_attr,
    pub UVERBS_API_ATTR_BKEY_LEN): DECLARE_BITMAP(uobj_finalize,,
    pub UVERBS_API_ATTR_BKEY_LEN): DECLARE_BITMAP(spec_finalize,,
    pub UVERBS_API_ATTR_BKEY_LEN): DECLARE_BITMAP(uobj_hw_obj_valid,,
//
// Must be last. bundle ends in a flex array which overlaps
// internal_buffer.
//
    pub bundle: uverbs_attr_bundle_hdr,
    pub internal_buffer: [u64; 32],
}

extern "C" {
    pub fn ib_uverbs_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uverbs_flow_spec {
    pub hdr: ib_uverbs_flow_spec_hdr,
    pub type: __u32,
    pub size: __u16,
    pub reserved: __u16,
}

//
// ib_uverbs_query_port_resp.port_cap_flags started out as just a copy of the
// PortInfo CapabilityMask, but was extended with unique bits.
//
// All IBA CapabilityMask bits are passed through here, except bit 26,
// which is overridden with IP_BASED_GIDS. This is due to a historical
// mistake in the implementation of IP_BASED_GIDS. Otherwise all other
// bits match the IBA definition across all kernel versions.
//
