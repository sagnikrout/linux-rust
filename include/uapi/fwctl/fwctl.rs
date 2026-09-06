//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/fwctl/fwctl.h
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
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES.
//

pub const FWCTL_TYPE: c_uint = 0x9A;
//
// DOC: General ioctl format
//
// The ioctl interface follows a general format to allow for extensibility. Each
// ioctl is passed a structure pointer as the argument providing the size of
// the structure in the first u32. The kernel checks that any structure space
// beyond what it understands is 0. This allows userspace to use the backward
// compatible portion while consistently using the newer, larger, structures.
//
// ioctls use a standard meaning for common errnos:
//
// - ENOTTY: The IOCTL number itself is not supported at all
// - E2BIG: The IOCTL number is supported, but the provided structure has
// non-zero in a part the kernel does not understand.
// - EOPNOTSUPP: The IOCTL number is supported, and the structure is
// understood, however a known field has a value the kernel does not
// understand or support.
// - EINVAL: Everything about the IOCTL was understood, but a field is not
// correct.
// - ENOMEM: Out of memory.
// - ENODEV: The underlying device has been hot-unplugged and the FD is
// orphaned.
//
// As well as additional errnos, within specific ioctls.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fwctl_device_type {
    FWCTL_DEVICE_TYPE_ERROR = 0,
    FWCTL_DEVICE_TYPE_MLX5 = 1,
    FWCTL_DEVICE_TYPE_CXL = 2,
    FWCTL_DEVICE_TYPE_BNXT = 3,
    FWCTL_DEVICE_TYPE_PDS = 4,
}

//
// struct fwctl_info - ioctl(FWCTL_INFO)
// @size: sizeof(struct fwctl_info)
// @flags: Must be 0
// @out_device_type: Returns the type of the device from enum fwctl_device_type
// @device_data_len: On input the length of the out_device_data memory. On
// output the size of the kernel's device_data which may be larger or
// smaller than the input. Maybe 0 on input.
// @out_device_data: Pointer to a memory of device_data_len bytes. Kernel will
// fill the entire memory, zeroing as required.
//
// Returns basic information about this fwctl instance, particularly what driver
// is being used to define the device_data format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_info {
    pub size: __u32,
    pub flags: __u32,
    pub out_device_type: __u32,
    pub device_data_len: __u32,
    pub out_device_data: __aligned_u64,
}

//
// enum fwctl_rpc_scope - Scope of access for the RPC
//
// Refer to fwctl.rst for a more detailed discussion of these scopes.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fwctl_rpc_scope {
//
// @FWCTL_RPC_CONFIGURATION: Device configuration access scope
//
// Read/write access to device configuration. When configuration
// is written to the device it remains in a fully supported state.
//
    FWCTL_RPC_CONFIGURATION = 0,
//
// @FWCTL_RPC_DEBUG_READ_ONLY: Read only access to debug information
//
// Readable debug information. Debug information is compatible with
// kernel lockdown, and does not disclose any sensitive information. For
// instance exposing any encryption secrets from this information is
// forbidden.
//
    FWCTL_RPC_DEBUG_READ_ONLY = 1,
//
// @FWCTL_RPC_DEBUG_WRITE: Writable access to lockdown compatible debug information
//
// Allows write access to data in the device which may leave a fully
// supported state. This is intended to permit intensive and possibly
// invasive debugging. This scope will taint the kernel.
//
    FWCTL_RPC_DEBUG_WRITE = 2,
//
// @FWCTL_RPC_DEBUG_WRITE_FULL: Write access to all debug information
//
// Allows read/write access to everything. Requires CAP_SYS_RAW_IO, so
// it is not required to follow lockdown principals. If in doubt
// debugging should be placed in this scope. This scope will taint the
// kernel.
//
    FWCTL_RPC_DEBUG_WRITE_FULL = 3,
}

//
// struct fwctl_rpc - ioctl(FWCTL_RPC)
// @size: sizeof(struct fwctl_rpc)
// @scope: One of enum fwctl_rpc_scope, required scope for the RPC
// @in_len: Length of the in memory
// @out_len: Length of the out memory
// @in: Request message in device specific format
// @out: Response message in device specific format
//
// Deliver a Remote Procedure Call to the device FW and return the response. The
// call's parameters and return are marshaled into linear buffers of memory. Any
// errno indicates that delivery of the RPC to the device failed. Return status
// originating in the device during a successful delivery must be encoded into
// out.
//
// The format of the buffers matches the out_device_type from FWCTL_INFO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_rpc {
    pub size: __u32,
    pub scope: __u32,
    pub in_len: __u32,
    pub out_len: __u32,
    pub in: __aligned_u64,
    pub out: __aligned_u64,
}

