//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_range_manager.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT

//
// struct ttm_range_mgr_node
//
// @base: base clase we extend
// @mm_nodes: MM nodes, usually 1
//
// Extending the ttm_resource object to manage an address space allocation with
// one or more drm_mm_nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_range_mgr_node {
    pub base: ttm_resource,
    pub mm_nodes: [drm_mm_node; ],
}

//
// to_ttm_range_mgr_node
//
// @res: the resource to upcast
//
// Upcast the ttm_resource object into a ttm_range_mgr_node object.
//
extern "C" {
    pub fn container_of(_arg: res, ttm_range_mgr_node: struct, _arg: base) -> return;
}
extern "C" {
    pub fn ttm_range_man_init_nocheck(_arg: bdev, _arg: type, _arg: use_tt, _arg: p_size) -> return;
}
extern "C" {
    pub fn ttm_range_man_fini_nocheck(_arg: bdev, _arg: type) -> return;
}
