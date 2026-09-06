//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/intel/ipu6/ipu6-isys-queue.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2013--2024 Intel Corporation

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_queue {
    pub vbq: vb2_queue,
    pub node: list_head,
    pub /: *mut *mut spinlock_t lock; / Protects active and incoming lists,
    pub active: list_head,
    pub incoming: list_head,
    pub fw_output: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_buffer {
    pub head: list_head,
    pub str2mmio_flag: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_video_buffer {
    pub vb_v4l2: vb2_v4l2_buffer,
    pub ib: ipu6_isys_buffer,
    pub dma_addr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu6_isys_buffer_list {
    pub head: list_head,
    pub nbufs: c_uint,
}

extern "C" {
    pub fn ipu6_isys_queue_init(aq: *mut ipu6_isys_queue) -> c_int;
}
