//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fpga/fpga-bridge.h
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
// struct fpga_bridge_ops - ops for low level FPGA bridge drivers
// @enable_show: returns the FPGA bridge's status
// @enable_set: set an FPGA bridge as enabled or disabled
// @fpga_bridge_remove: set FPGA into a specific state during driver remove
// @groups: optional attribute groups.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_bridge_ops {
    pub bridge): *mut *mut int (enable_show)(struct fpga_bridge,
    pub enable): *mut *mut *mut int (enable_set)(struct fpga_bridge bridge, bool,
    pub bridge): *mut *mut void (fpga_bridge_remove)(struct fpga_bridge,
    pub groups: *const attribute_group,
}

//
// struct fpga_bridge_info - collection of parameters an FPGA Bridge
// @name: fpga bridge name
// @br_ops: pointer to structure of fpga bridge ops
// @priv: fpga bridge private data
//
// fpga_bridge_info contains parameters for the register function. These
// are separated into an info structure because they some are optional
// others could be added to in the future. The info structure facilitates
// maintaining a stable API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_bridge_info {
    pub name: *const c_char,
    pub br_ops: *const fpga_bridge_ops,
    pub priv: *mut c_void,
}

//
// struct fpga_bridge - FPGA bridge structure
// @name: name of low level FPGA bridge
// @dev: FPGA bridge device
// @mutex: enforces exclusive reference to bridge
// @br_ops: pointer to struct of FPGA bridge ops
// @br_ops_owner: module containing the br_ops
// @info: fpga image specific information
// @node: FPGA bridge list node
// @priv: low level driver private date
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpga_bridge {
    pub name: *const c_char,
    pub dev: device,
    pub /: *mut *mut mutex mutex; / for exclusive reference to bridge,
    pub br_ops: *const fpga_bridge_ops,
    pub br_ops_owner: *mut module,
    pub info: *mut fpga_image_info,
    pub node: list_head,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn fpga_bridge_put(bridge: *mut fpga_bridge);
}
extern "C" {
    pub fn fpga_bridge_enable(bridge: *mut fpga_bridge) -> c_int;
}
extern "C" {
    pub fn fpga_bridge_disable(bridge: *mut fpga_bridge) -> c_int;
}
extern "C" {
    pub fn fpga_bridges_enable(bridge_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn fpga_bridges_disable(bridge_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn fpga_bridges_put(bridge_list: *mut list_head);
}

extern "C" {
    pub fn fpga_bridge_unregister(br: *mut fpga_bridge);
}
