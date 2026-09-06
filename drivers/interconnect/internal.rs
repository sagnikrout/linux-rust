//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/interconnect/internal.h
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
// Interconnect framework internal structs
//
// Copyright (c) 2019, Linaro Ltd.
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//
// struct icc_req - constraints that are attached to each node
// @req_node: entry in list of requests for the particular @node
// @node: the interconnect node to which this constraint applies
// @dev: reference to the device that sets the constraints
// @enabled: indicates whether the path with this request is enabled
// @tag: path tag (optional)
// @avg_bw: an integer describing the average bandwidth in kBps
// @peak_bw: an integer describing the peak bandwidth in kBps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_req {
    pub req_node: hlist_node,
    pub node: *mut icc_node,
    pub dev: *mut device,
    pub enabled: bool,
    pub tag: u32,
    pub avg_bw: u32,
    pub peak_bw: u32,
}

//
// struct icc_path - interconnect path structure
// @name: a string name of the path (useful for ftrace)
// @num_nodes: number of hops (nodes)
// @reqs: array of the requests applicable to this path of nodes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_path {
    pub name: *const c_char,
    pub num_nodes: usize,
    pub __counted_by(num_nodes): icc_req reqs[],
}

extern "C" {
    pub fn icc_debugfs_client_init(icc_dir: *mut dentry) -> c_int;
}
