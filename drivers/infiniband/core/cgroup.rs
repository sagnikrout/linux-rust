//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/cgroup.c
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
// Copyright (C) 2016 Parav Pandit <pandit.parav@gmail.com>
//

//
// ib_device_register_rdmacg - register with rdma cgroup.
// @device: device to register to participate in resource
// accounting by rdma cgroup.
//
// Register with the rdma cgroup. Should be called before
// exposing rdma device to user space applications to avoid
// resource accounting leak.
//
#[no_mangle]
pub unsafe extern "C" fn ib_device_register_rdmacg(device: *mut ib_device) {
    void ib_device_register_rdmacg(struct ib_device *device)
    {
    device.cg_device.name = device.name;
    device.cg_device.index = device.index;
    rdmacg_register_device(&device.cg_device);
    }
//
// ib_device_unregister_rdmacg - unregister with rdma cgroup.
// @device: device to unregister.
//
// Unregister with the rdma cgroup. Should be called after
// all the resources are deallocated, and after a stage when any
// other resource allocation by user application cannot be done
// for this device to avoid any leak in accounting.
//
#[no_mangle]
pub unsafe extern "C" fn ib_device_unregister_rdmacg(device: *mut ib_device) {
    void ib_device_unregister_rdmacg(struct ib_device *device)
    {
    rdmacg_unregister_device(&device.cg_device);
    }
    int ib_rdmacg_try_charge(struct ib_rdmacg_object *cg_obj,
    struct ib_device *device,
    enum rdmacg_resource_type resource_index)
    {
    return rdmacg_try_charge(&cg_obj.cg, &device.cg_device,
    resource_index);
    }
    EXPORT_SYMBOL(ib_rdmacg_try_charge);
    void ib_rdmacg_uncharge(struct ib_rdmacg_object *cg_obj,
    struct ib_device *device,
    enum rdmacg_resource_type resource_index)
    {
    rdmacg_uncharge(cg_obj.cg, &device.cg_device,
    resource_index);
    }
    EXPORT_SYMBOL(ib_rdmacg_uncharge);
