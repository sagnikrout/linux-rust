//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dma-heap.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// DMABUF Heaps Userspace API
//
// Copyright (C) 2011 Google, Inc.
// Copyright (C) 2019 Linaro Ltd.
//

//
// DOC: DMABUF Heaps Userspace API
//
// Valid FD_FLAGS are O_CLOEXEC, O_RDONLY, O_WRONLY, O_RDWR

// Currently no heap flags

//
// struct dma_heap_allocation_data - metadata passed from userspace for
// allocations
// @len:		size of the allocation
// @fd:			will be populated with a fd which provides the
// handle to the allocated dma-buf
// @fd_flags:		file descriptor flags used when allocating
// @heap_flags:		flags passed to heap
//
// Provided by userspace as an argument to the ioctl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_heap_allocation_data {
    pub len: __u64,
    pub fd: __u32,
    pub fd_flags: __u32,
    pub heap_flags: __u64,
}

//
// DOC: DMA_HEAP_IOCTL_ALLOC - allocate memory from pool
//
// Takes a dma_heap_allocation_data struct and returns it with the fd field
// populated with the dmabuf handle of the allocation.
//

