//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/iio/buffer.h
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
// industrial I/O buffer definitions needed both in and out of kernel
//

// Flags for iio_dmabuf.flags

pub const IIO_BUFFER_DMABUF_SUPPORTED_FLAGS: c_uint = 0x00000001;
//
// struct iio_dmabuf - Descriptor for a single IIO DMABUF object
// @fd:		file descriptor of the DMABUF object
// @flags:	one or more IIO_BUFFER_DMABUF_* flags
// @bytes_used:	number of bytes used in this DMABUF for the data transfer.
// Should generally be set to the DMABUF's size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_dmabuf {
    pub fd: __u32,
    pub flags: __u32,
    pub bytes_used: __u64,
}

