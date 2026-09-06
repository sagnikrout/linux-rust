//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/interconnect-provider.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2018, Linaro Ltd.
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//

//
// struct icc_node_data - icc node data
//
// @node: icc node
// @tag: tag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_node_data {
    pub node: *mut icc_node,
    pub tag: u32,
}

//
// struct icc_onecell_data - driver data for onecell interconnect providers
//
// @num_nodes: number of nodes in this device
// @nodes: array of pointers to the nodes in this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_onecell_data {
    pub num_nodes: c_uint,
    pub __counted_by(num_nodes): *mut *mut icc_node nodes[],
}

//
// struct icc_provider - interconnect provider (controller) entity that might
// provide multiple interconnect controls
//
// @provider_list: list of the registered interconnect providers
// @nodes: internal list of the interconnect provider nodes
// @set: pointer to device specific set operation function
// @aggregate: pointer to device specific aggregate operation function
// @pre_aggregate: pointer to device specific function that is called
// before the aggregation begins (optional)
// @get_bw: pointer to device specific function to get current bandwidth
// @xlate: provider-specific callback for mapping nodes from phandle arguments
// @xlate_extended: vendor-specific callback for mapping node data from phandle arguments
// @dev: the device this interconnect provider belongs to
// @users: count of active users
// @inter_set: whether inter-provider pairs will be configured with @set
// @data: pointer to private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_provider {
    pub provider_list: list_head,
    pub nodes: list_head,
    pub dst): *mut *mut *mut int (set)(struct icc_node src, struct icc_node,
    pub agg_peak): *mut *mut u32 peak_bw, u32 agg_avg, u32,
    pub node): *mut *mut void (pre_aggregate)(struct icc_node,
    pub peak): *mut *mut *mut *mut int (get_bw)(struct icc_node node, u32 avg, u32,
    pub data): *const *const *const *const icc_node (xlate)(of_phandle_args spec, void,
    pub data): *mut c_void,
    pub dev: *mut device,
    pub users: c_int,
    pub inter_set: bool,
    pub data: *mut c_void,
}

//
// struct icc_node - entity that is part of the interconnect topology
//
// @id: platform specific node id
// @name: node name used in debugfs
// @links: a list of targets pointing to where we can go next when traversing
// @num_links: number of links to other interconnect nodes
// @provider: points to the interconnect provider of this node
// @node_list: the list entry in the parent provider's "nodes" list
// @search_list: list used when walking the nodes graph
// @reverse: pointer to previous node when walking the nodes graph
// @is_traversed: flag that is used when walking the nodes graph
// @req_list: a list of QoS constraint requests associated with this node
// @avg_bw: aggregated value of average bandwidth requests from all consumers
// @peak_bw: aggregated value of peak bandwidth requests from all consumers
// @init_avg: average bandwidth value that is read from the hardware during init
// @init_peak: peak bandwidth value that is read from the hardware during init
// @data: pointer to private data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_node {
    pub id: c_int,
    pub name: *const c_char,
    pub links: *mut icc_node,
    pub num_links: usize,
    pub provider: *mut icc_provider,
    pub node_list: list_head,
    pub search_list: list_head,
    pub reverse: *mut icc_node,
    pub is_traversed:1: u8,
    pub req_list: hlist_head,
    pub avg_bw: u32,
    pub peak_bw: u32,
    pub init_avg: u32,
    pub init_peak: u32,
    pub data: *mut c_void,
}

extern "C" {
    pub fn icc_node_destroy(id: c_int);
}
extern "C" {
    pub fn icc_node_set_name(node: *mut icc_node, provider: *const icc_provider, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn icc_link_nodes(src_node: *mut icc_node, dst_node: *mut icc_node) -> c_int;
}
extern "C" {
    pub fn icc_link_create(node: *mut icc_node, dst_id: c_int) -> c_int;
}
extern "C" {
    pub fn icc_node_add(node: *mut icc_node, provider: *mut icc_provider);
}
extern "C" {
    pub fn icc_node_del(node: *mut icc_node);
}
extern "C" {
    pub fn icc_nodes_remove(provider: *mut icc_provider) -> c_int;
}
extern "C" {
    pub fn icc_provider_init(provider: *mut icc_provider);
}
extern "C" {
    pub fn icc_provider_register(provider: *mut icc_provider) -> c_int;
}
extern "C" {
    pub fn icc_provider_deregister(provider: *mut icc_provider);
}
extern "C" {
    pub fn icc_sync_state(dev: *mut device);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}

