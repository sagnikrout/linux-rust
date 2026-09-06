//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/core/core_priv.h
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
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
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

// Total number of ports combined across all struct ib_devices's
pub const RDMA_MAX_PORTS: c_int = 8192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkey_index_qp_list {
    pub pkey_index_list: list_head,
    pub pkey_index: u16,
// Lock to hold while iterating the qp_list.
    pub qp_list_lock: spinlock_t,
    pub qp_list: list_head,
}

//
// struct rdma_dev_net - rdma net namespace metadata for a net
// @nl_sock:	Pointer to netlink socket
// @net:	Pointer to owner net namespace
// @id:		xarray id to identify the net namespace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_dev_net {
    pub nl_sock: *mut sock,
    pub net: possible_net_t,
    pub id: u32,
}

extern "C" {
    pub fn net_generic(_arg: net, _arg: rdma_dev_net_id) -> return;
}
extern "C" {
    pub fn ib_device_rename(ibdev: *mut ib_device, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn ib_device_set_dim(ibdev: *mut ib_device, use_dim: u8) -> c_int;
}
extern "C" {
    pub fn ib_device_enable_gid_updates(device: *mut ib_device);
}
extern "C" {
    pub fn ib_device_disable_gid_updates(device: *mut ib_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_client_nl_info {
    pub nl_msg: *mut sk_buff,
    pub cdev: *mut device,
    pub port: u32,
    pub abi: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cache_gid_default_mode {
    IB_CACHE_GID_DEFAULT_MODE_SET,
    IB_CACHE_GID_DEFAULT_MODE_DELETE
}

extern "C" {
    pub fn ib_cache_gid_parse_type_str(buf: *const c_char) -> c_int;
}
extern "C" {
    pub fn roce_gid_mgmt_init() -> c_int;
}
extern "C" {
    pub fn roce_gid_mgmt_cleanup();
}
extern "C" {
    pub fn roce_gid_type_mask_support(ib_dev: *mut ib_device, port: u32) -> c_ulong;
}
extern "C" {
    pub fn ib_cache_setup_one(device: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn ib_cache_cleanup_one(device: *mut ib_device);
}
extern "C" {
    pub fn ib_cache_release_one(device: *mut ib_device);
}
extern "C" {
    pub fn ib_dispatch_event_clients(event: *mut ib_event);
}

extern "C" {
    pub fn ib_device_register_rdmacg(device: *mut ib_device);
}
extern "C" {
    pub fn ib_device_unregister_rdmacg(device: *mut ib_device);
}

extern "C" {
    pub fn netdev_has_upper_dev_all_rcu(_arg: dev, _arg: upper) -> return;
}
extern "C" {
    pub fn addr_init() -> c_int;
}
extern "C" {
    pub fn addr_cleanup();
}
extern "C" {
    pub fn ib_mad_init() -> c_int;
}
extern "C" {
    pub fn ib_mad_cleanup();
}
extern "C" {
    pub fn ib_sa_init() -> c_int;
}
extern "C" {
    pub fn ib_sa_cleanup();
}
extern "C" {
    pub fn rdma_nl_init();
}
extern "C" {
    pub fn rdma_nl_exit();
}

extern "C" {
    pub fn ib_security_release_port_pkey_list(device: *mut ib_device);
}
extern "C" {
    pub fn ib_create_qp_security(qp: *mut ib_qp, dev: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn ib_destroy_qp_security_begin(sec: *mut ib_qp_security);
}
extern "C" {
    pub fn ib_destroy_qp_security_abort(sec: *mut ib_qp_security);
}
extern "C" {
    pub fn ib_destroy_qp_security_end(sec: *mut ib_qp_security);
}
extern "C" {
    pub fn ib_open_shared_qp_security(qp: *mut ib_qp, dev: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn ib_close_shared_qp_security(sec: *mut ib_qp_security);
}
extern "C" {
    pub fn ib_mad_agent_security_cleanup(agent: *mut ib_mad_agent);
}
extern "C" {
    pub fn ib_mad_enforce_security(map: *mut ib_mad_agent_private, pkey_index: u16) -> c_int;
}
extern "C" {
    pub fn ib_mad_agent_security_change();
}

// RDMA device netlink
extern "C" {
    pub fn nldev_init();
}
extern "C" {
    pub fn nldev_exit();
}
extern "C" {
    pub fn ib_qp_usecnt_inc(qp: *mut ib_qp);
}
extern "C" {
    pub fn ib_qp_usecnt_dec(qp: *mut ib_qp);
}
extern "C" {
    pub fn ib_free_port_attrs(coredev: *mut ib_core_device);
}
extern "C" {
    pub fn ib_setup_port_attrs(coredev: *mut ib_core_device) -> c_int;
}
extern "C" {
    pub fn ib_device_release_hw_stats(data: *mut hw_stats_device_data);
}
extern "C" {
    pub fn ib_setup_device_attrs(ibdev: *mut ib_device) -> c_int;
}
extern "C" {
    pub fn rdma_compatdev_set(enable: u8) -> c_int;
}
extern "C" {
    pub fn rdma_nl_net_init(rnet: *mut rdma_dev_net) -> c_int;
}
extern "C" {
    pub fn rdma_nl_net_exit(rnet: *mut rdma_dev_net);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_umap_priv {
    pub vma: *mut vm_area_struct,
    pub list: list_head,
    pub entry: *mut rdma_user_mmap_entry,
}

extern "C" {
    pub fn ib_cq_pool_cleanup(dev: *mut ib_device);
}
extern "C" {
    pub fn rdma_nl_get_privileged_qkey() -> bool;
}
