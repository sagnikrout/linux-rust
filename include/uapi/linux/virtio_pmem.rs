//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_pmem.h
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


// SPDX-License-Identifier: (GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause
//
// Definitions for virtio-pmem devices.
//
// Copyright (C) 2019 Red Hat, Inc.
//
// Author(s): Pankaj Gupta <pagupta@redhat.com>
//

// Feature bits
// guest physical address range will be indicated as shared memory region 0
pub const VIRTIO_PMEM_F_SHMEM_REGION: c_int = 0;
// shmid of the shared memory region corresponding to the pmem
pub const VIRTIO_PMEM_SHMEM_REGION_ID: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pmem_config {
    pub start: __le64,
    pub size: __le64,
}

pub const VIRTIO_PMEM_REQ_TYPE_FLUSH: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pmem_resp {
// Host return status corresponding to flush request
    pub ret: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pmem_req {
// command type
    pub type: __le32,
}
