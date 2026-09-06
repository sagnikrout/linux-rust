//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fwctl.h
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
//
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//

//
// struct fwctl_ops - Driver provided operations
//
// fwctl_unregister() will wait until all excuting ops are completed before it
// returns. Drivers should be mindful to not let their ops run for too long as
// it will block device hot unplug and module unloading.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_ops {
//
// @device_type: The drivers assigned device_type number. This is uABI.
//
    pub device_type: fwctl_device_type,
//
// @uctx_size: The size of the fwctl_uctx struct to allocate. The first
// bytes of this memory will be a fwctl_uctx. The driver can use the
// remaining bytes as its private memory.
//
    pub uctx_size: usize,
//
// @open_uctx: Called when a file descriptor is opened before the uctx
// is ever used.
//
    pub uctx): *mut *mut int (open_uctx)(struct fwctl_uctx,
//
// @close_uctx: Called when the uctx is destroyed, usually when the FD
// is closed.
//
    pub uctx): *mut *mut void (close_uctx)(struct fwctl_uctx,
//
// @info: Implement FWCTL_INFO. Return a kmalloc() memory that is copied
// to out_device_data. On input length indicates the size of the user
// buffer on output it indicates the size of the memory. The driver can
// ignore length on input, the core code will handle everything.
//
    pub length): *mut *mut *mut *mut void (info)(struct fwctl_uctx uctx, size_t,
//
// @fw_rpc: Implement FWCTL_RPC. Deliver rpc_in/in_len to the FW and
// return the response and set out_len. rpc_in can be returned as the
// response pointer. Otherwise the returned pointer is freed with
// kvfree().
//
    pub out_len): *mut *mut void rpc_in, size_t in_len, size_t,
}

//
// struct fwctl_device - Per-driver registration struct
// @dev: The sysfs (class/fwctl/fwctlXX) device
//
// Each driver instance will have one of these structs with the driver private
// data following immediately after. This struct is refcounted, it is freed by
// calling fwctl_put().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_device {
    pub dev: device,
// private:
    pub cdev: cdev,
// Protect uctx_list
    pub uctx_list_lock: mutex,
    pub uctx_list: list_head,
//
// Protect ops, held for write when ops becomes NULL during unregister,
// held for read whenever ops is loaded or an ops function is running.
//
    pub registration_lock: rw_semaphore,
    pub ops: *const fwctl_ops,
}

//
// fwctl_alloc_device - Allocate a fwctl
// @parent: Physical device that provides the FW interface
// @ops: Driver ops to register
// @drv_struct: 'struct driver_fwctl' that holds the struct fwctl_device
// @member: Name of the struct fwctl_device in @drv_struct
//
// This allocates and initializes the fwctl_device embedded in the drv_struct.
// Upon success the pointer must be freed via fwctl_put(). Returns a 'drv_struct
// \*' on success, NULL on error.
//

extern "C" {
    pub fn fwctl_register(fwctl: *mut fwctl_device) -> c_int;
}
extern "C" {
    pub fn fwctl_unregister(fwctl: *mut fwctl_device);
}
//
// struct fwctl_uctx - Per user FD context
// @fwctl: fwctl instance that owns the context
//
// Every FD opened by userspace will get a unique context allocation. Any driver
// private data will follow immediately after.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fwctl_uctx {
    pub fwctl: *mut fwctl_device,
// private:
// Head at fwctl_device::uctx_list
    pub uctx_list_entry: list_head,
}
