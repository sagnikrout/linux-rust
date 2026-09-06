//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/platform/vfio_platform_private.h
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
// Copyright (C) 2013 - Virtual Open Systems
// Author: Antonios Motakis <a.motakis@virtualopensystems.com>
//

pub const VFIO_PLATFORM_OFFSET_SHIFT: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_platform_irq {
    pub flags: u32,
    pub count: u32,
    pub hwirq: c_int,
    pub name: *mut c_char,
    pub trigger: *mut eventfd_ctx,
    pub masked: bool,
    pub lock: spinlock_t,
    pub unmask: *mut virqfd,
    pub mask: *mut virqfd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_platform_region {
    pub addr: u64,
    pub size: resource_size_t,
    pub flags: u32,
    pub type: u32,
pub const VFIO_PLATFORM_REGION_TYPE_MMIO: c_int = 1;
pub const VFIO_PLATFORM_REGION_TYPE_PIO: c_int = 2;
    pub ioaddr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_platform_device {
    pub vdev: vfio_device,
    pub regions: *mut vfio_platform_region,
    pub num_regions: u32,
    pub irqs: *mut vfio_platform_irq,
    pub num_irqs: u32,
    pub igate: mutex,
    pub compat: *const c_char,
    pub acpihid: *const c_char,
    pub reset_module: *mut module,
    pub device: *mut device,
//
// These fields should be filled by the bus specific binder
//
    pub opaque: *mut c_void,
    pub name: *const c_char,
    pub flags: u32,
// callbacks to discover device resources
    pub i): *mut *mut *mut (get_resource)(struct vfio_platform_device vdev, int,
    pub i): *mut *mut *mut int (get_irq)(struct vfio_platform_device vdev, int,
    pub vdev): *mut *mut int (of_reset)(struct vfio_platform_device,
    pub reset_required: bool,
}

extern "C" {
    pub fn int(vdev: *mut *mut vfio_platform_reset_fn_t)(struct vfio_platform_device) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_platform_reset_node {
    pub link: list_head,
    pub compat: *mut c_char,
    pub owner: *mut module,
    pub of_reset: vfio_platform_reset_fn_t,
}

extern "C" {
    pub fn vfio_platform_init_common(vdev: *mut vfio_platform_device) -> c_int;
}
extern "C" {
    pub fn vfio_platform_release_common(vdev: *mut vfio_platform_device);
}
extern "C" {
    pub fn vfio_platform_open_device(core_vdev: *mut vfio_device) -> c_int;
}
extern "C" {
    pub fn vfio_platform_close_device(core_vdev: *mut vfio_device);
}
extern "C" {
    pub fn vfio_platform_irq_init(vdev: *mut vfio_platform_device) -> c_int;
}
extern "C" {
    pub fn vfio_platform_irq_cleanup(vdev: *mut vfio_platform_device);
}
extern "C" {
    pub fn __vfio_platform_register_reset(n: *mut vfio_platform_reset_node);
}

