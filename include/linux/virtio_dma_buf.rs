//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_dma_buf.h
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
// dma-bufs for virtio exported objects
//
// Copyright (C) 2020 Google, Inc.
//

//
// struct virtio_dma_buf_ops - operations possible on exported object dma-buf
// @ops: the base dma_buf_ops. ops.attach MUST be virtio_dma_buf_attach.
// @device_attach: [optional] callback invoked by virtio_dma_buf_attach during
// all attach operations.
// @get_uuid: [required] callback to get the uuid of the exported object.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_dma_buf_ops {
    pub ops: dma_buf_ops,
    pub attach): *mut dma_buf_attachment,
    pub uuid): *mut *mut *mut int (get_uuid)(struct dma_buf dma_buf, uuid_t,
}

extern "C" {
    pub fn is_virtio_dma_buf(dma_buf: *mut dma_buf) -> bool;
}
extern "C" {
    pub fn virtio_dma_buf_get_uuid(dma_buf: *mut dma_buf, uuid: *mut uuid_t) -> c_int;
}
