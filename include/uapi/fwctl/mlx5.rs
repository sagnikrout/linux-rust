//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/fwctl/mlx5.h
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
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//
// These are definitions for the command interface for mlx5 HW. mlx5 FW has a
// User Context mechanism which allows the FW to understand a security scope.
// FWCTL binds each FD to a FW user context and then places the User Context ID
// (UID) in each command header. The created User Context has a capability set
// that is appropriate for FWCTL's security model.
//
// Command formation should use a copy of the structs in mlx5_ifc.h following
// the Programmers Reference Manual. A open release is available here:
//
// https://network.nvidia.com/files/doc-2020/ethernet-adapters-programming-manual.pdf
//
// The device_type for this file is FWCTL_DEVICE_TYPE_MLX5.
//

//
// struct fwctl_info_mlx5 - ioctl(FWCTL_INFO) out_device_data
// @uid: The FW UID this FD is bound to. Each command header will force
// this value.
// @uctx_caps: The FW capabilities that are enabled for the uid.
//
// Return basic information about the FW interface available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_info_mlx5 {
    pub uid: __u32,
    pub uctx_caps: __u32,
}
