//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/vdpa_sim/vdpa_sim.h
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
// Copyright (c) 2020, Red Hat Inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_virtqueue {
    pub vring: vringh,
    pub in_iov: vringh_kiov,
    pub out_iov: vringh_kiov,
    pub head: c_ushort,
    pub ready: bool,
    pub desc_addr: u64,
    pub device_addr: u64,
    pub driver_addr: u64,
    pub num: u32,
    pub private: *mut c_void,
    pub data): *mut *mut irqreturn_t (cb)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim_dev_attr {
    pub mgmt_dev: *mut vdpa_mgmt_dev,
    pub name: *const c_char,
    pub supported_features: u64,
    pub alloc_size: usize,
    pub config_size: usize,
    pub nvqs: c_int,
    pub id: u32,
    pub ngroups: u32,
    pub nas: u32,
    pub vdpasim): *mut *mut void (work_fn)(struct vdpasim,
    pub config): *mut *mut *mut void (get_config)(struct vdpasim vdpasim, void,
    pub config): *const *const *const void (set_config)(struct vdpasim vdpasim, void,
    pub extack): *mut netlink_ext_ack,
    pub vdpasim): *mut *mut void (free)(struct vdpasim,
}

// State of each vdpasim device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdpasim {
    pub vdpa: vdpa_device,
    pub vqs: *mut vdpasim_virtqueue,
    pub worker: *mut kthread_worker,
    pub work: kthread_work,
    pub mm_bound: *mut mm_struct,
    pub dev_attr: vdpasim_dev_attr,
// mutex to synchronize virtqueue state
    pub mutex: mutex,
// virtio config according to device type
    pub config: *mut c_void,
    pub iommu: *mut vhost_iotlb,
    pub iommu_pt: *mut bool,
    pub status: u32,
    pub generation: u32,
    pub features: u64,
    pub groups: u32,
    pub running: bool,
    pub pending_kick: bool,
// spinlock to synchronize iommu table
    pub iommu_lock: spinlock_t,
}

extern "C" {
    pub fn vdpasim_schedule_work(vdpasim: *mut vdpasim);
}
// TODO: cross-endian support
extern "C" {
    pub fn __virtio16_to_cpu(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio16(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio32_to_cpu(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio32(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
extern "C" {
    pub fn __virtio64_to_cpu(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio64(_arg: vdpasim_is_little_endian(vdpasim), _arg: val) -> return;
}
