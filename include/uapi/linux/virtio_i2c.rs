//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_i2c.h
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


// SPDX-License-Identifier: GPL-2.0-or-later WITH Linux-syscall-note
//
// Definitions for virtio I2C Adpter
//
// Copyright (c) 2021 Intel Corporation. All rights reserved.
//

// Virtio I2C Feature bits
pub const VIRTIO_I2C_F_ZERO_LENGTH_REQUEST: c_int = 0;
// The bit 0 of the @virtio_i2c_out_hdr.@flags, used to group the requests

// The bit 1 of the @virtio_i2c_out_hdr.@flags, used to mark a buffer as read

//
// struct virtio_i2c_out_hdr - the virtio I2C message OUT header
// @addr: the controlled device address
// @padding: used to pad to full dword
// @flags: used for feature extensibility
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_i2c_out_hdr {
    pub addr: __le16,
    pub padding: __le16,
    pub flags: __le32,
}

//
// struct virtio_i2c_in_hdr - the virtio I2C message IN header
// @status: the processing result from the backend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_i2c_in_hdr {
    pub status: __u8,
}

// The final status written by the device
pub const VIRTIO_I2C_MSG_OK: c_int = 0;
pub const VIRTIO_I2C_MSG_ERR: c_int = 1;
