//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_graph.h
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
// OF graph binding parsing helpers
//
// Copyright (C) 2012 - 2013 Samsung Electronics Co., Ltd.
// Author: Sylwester Nawrocki <s.nawrocki@samsung.com>
//
// Copyright (C) 2012 Renesas Electronics Corp.
// Author: Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

//
// struct of_endpoint - the OF graph endpoint data structure
// @port: identifier (value of reg property) of a port this endpoint belongs to
// @id: identifier (value of reg property) of this endpoint
// @local_node: pointer to device_node of this endpoint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_endpoint {
    pub port: c_uint,
    pub id: c_uint,
    pub local_node: *const device_node,
}

//
// for_each_endpoint_of_node - iterate over every endpoint in a device node
// @parent: parent device node containing ports and endpoints
// @child: loop variable pointing to the current endpoint node
//
// When breaking out of the loop, of_node_put(child) has to be called manually.
//

//
// for_each_of_graph_port - iterate over every port in a device or ports node
// @parent: parent device or ports node containing port
// @child: loop variable pointing to the current port node
//
// When breaking out of the loop, and continue to use the @child, you need to
// use return_ptr(@child) or no_free_ptr(@child) not to call __free() for it.
//

//
// for_each_of_graph_port_endpoint - iterate over every endpoint in a port node
// @parent: parent port node
// @child: loop variable pointing to the current endpoint node
//
// When breaking out of the loop, and continue to use the @child, you need to
// use return_ptr(@child) or no_free_ptr(@child) not to call __free() for it.
//

extern "C" {
    pub fn of_graph_is_present(node: *const device_node) -> bool;
}
extern "C" {
    pub fn of_graph_get_endpoint_count(np: *const device_node) -> c_uint;
}
extern "C" {
    pub fn of_graph_get_port_count(np: *mut device_node) -> c_uint;
}

