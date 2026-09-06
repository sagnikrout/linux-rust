//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-heap.h
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
// DMABUF Heaps Allocation Infrastructure
//
// Copyright (C) 2011 Google, Inc.
// Copyright (C) 2019 Linaro Ltd.
//

//
// struct dma_heap_ops - ops to operate on a given heap
// @allocate:	allocate dmabuf and return struct dma_buf ptr
//
// allocate returns dmabuf on success, ERR_PTR(-errno) on error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_heap_ops {
    pub heap_flags): u64,
}

//
// struct dma_heap_export_info - information needed to export a new dmabuf heap
// @name:	used for debugging/device-node name
// @ops:	ops struct for this heap
// @priv:	heap exporter private data
//
// Information needed to export a new dmabuf heap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_heap_export_info {
    pub name: *const c_char,
    pub ops: *const dma_heap_ops,
    pub priv: *mut c_void,
}
