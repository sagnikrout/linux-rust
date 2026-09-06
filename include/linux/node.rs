//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/node.h
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
// include/linux/node.h - generic node definition
//
// This is mainly for topological representation. We define the
// basic 'struct node' here, which can be embedded in per-arch
// definitions of processors.
//
// Basic handling of the devices is done in drivers/base/node.c
// and system devices are handled in drivers/base/sys.c.
//
// Nodes are exported via driverfs in the class/node/devices
// directory.
//

//
// struct access_coordinate - generic performance coordinates container
//
// @read_bandwidth:	Read bandwidth in MB/s
// @write_bandwidth:	Write bandwidth in MB/s
// @read_latency:	Read latency in nanoseconds
// @write_latency:	Write latency in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct access_coordinate {
    pub read_bandwidth: c_uint,
    pub write_bandwidth: c_uint,
    pub read_latency: c_uint,
    pub write_latency: c_uint,
}

//
// ACCESS_COORDINATE_LOCAL correlates to ACCESS CLASS 0
// - access_coordinate between target node and nearest initiator node
// ACCESS_COORDINATE_CPU correlates to ACCESS CLASS 1
// - access_coordinate between target node and nearest CPU node
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum access_coordinate_class {
    ACCESS_COORDINATE_LOCAL,
    ACCESS_COORDINATE_CPU,
    ACCESS_COORDINATE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_indexing {
    NODE_CACHE_DIRECT_MAP,
    NODE_CACHE_INDEXED,
    NODE_CACHE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_write_policy {
    NODE_CACHE_WRITE_BACK,
    NODE_CACHE_WRITE_THROUGH,
    NODE_CACHE_WRITE_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cache_mode {
    NODE_CACHE_ADDR_MODE_RESERVED,
    NODE_CACHE_ADDR_MODE_EXTENDED_LINEAR,
}

//
// struct node_cache_attrs - system memory caching attributes
//
// @indexing:		The ways memory blocks may be placed in cache
// @write_policy:	Write back or write through policy
// @size:		Total size of cache in bytes
// @line_size:		Number of bytes fetched on a cache miss
// @level:		The cache hierarchy level
// @address_mode:		The address mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_cache_attrs {
    pub indexing: cache_indexing,
    pub write_policy: cache_write_policy,
    pub size: u64,
    pub line_size: u16,
    pub level: u8,
    pub address_mode: u16,
}

extern "C" {
    pub fn node_add_cache(nid: c_uint, cache_attrs: *mut node_cache_attrs);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node {
    pub dev: device,
    pub access_list: list_head,

    pub cache_attrs: list_head,
    pub cache_dev: *mut device,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_notify {
    pub nid: c_int,
}

extern "C" {
    pub fn register_node_notifier(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_node_notifier(nb: *mut notifier_block);
}
extern "C" {
    pub fn node_notify(val: c_ulong, v: *mut c_void) -> c_int;
}

extern "C" {
    pub fn node_dev_init();
}
// Core of the node registration - only memory hotplug should use this
extern "C" {
    pub fn register_node(nid: c_int) -> c_int;
}
extern "C" {
    pub fn unregister_node(nid: c_int);
}
extern "C" {
    pub fn register_cpu_under_node(cpu: c_uint, nid: c_uint) -> c_int;
}
extern "C" {
    pub fn unregister_cpu_under_node(cpu: c_uint, nid: c_uint) -> c_int;
}
extern "C" {
    pub fn unregister_memory_block_under_nodes(mem_blk: *mut memory_block);
}

