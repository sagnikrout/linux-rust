//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vhost_iotlb.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_iotlb_map {
    pub rb: rb_node,
    pub link: list_head,
    pub start: u64,
    pub last: u64,
    pub size: u64,
    pub addr: u64,
pub const VHOST_MAP_RO: c_uint = 0x1;
pub const VHOST_MAP_WO: c_uint = 0x2;
pub const VHOST_MAP_RW: c_uint = 0x3;
    pub perm: u32,
    pub flags_padding: u32,
    pub __subtree_last: u64,
    pub opaque: *mut c_void,
}

pub const VHOST_IOTLB_FLAG_RETIRE: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_iotlb {
    pub root: rb_root_cached,
    pub list: list_head,
    pub limit: c_uint,
    pub nmaps: c_uint,
    pub flags: c_uint,
}

extern "C" {
    pub fn vhost_iotlb_del_range(iotlb: *mut vhost_iotlb, start: u64, last: u64);
}
extern "C" {
    pub fn vhost_iotlb_free(iotlb: *mut vhost_iotlb);
}
extern "C" {
    pub fn vhost_iotlb_reset(iotlb: *mut vhost_iotlb);
}
