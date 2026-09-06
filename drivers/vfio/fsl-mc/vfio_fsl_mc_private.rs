//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vfio/fsl-mc/vfio_fsl_mc_private.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright 2013-2016 Freescale Semiconductor Inc.
// Copyright 2016,2019-2020 NXP
//
pub const VFIO_FSL_MC_OFFSET_SHIFT: c_int = 40;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_fsl_mc_irq {
    pub flags: u32,
    pub count: u32,
    pub trigger: *mut eventfd_ctx,
    pub name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_fsl_mc_region {
    pub flags: u32,
    pub type: u32,
    pub addr: u64,
    pub size: resource_size_t,
    pub ioaddr: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vfio_fsl_mc_device {
    pub vdev: vfio_device,
    pub mc_dev: *mut fsl_mc_device,
    pub nb: notifier_block,
    pub regions: *mut vfio_fsl_mc_region,
    pub igate: mutex,
    pub mc_irqs: *mut vfio_fsl_mc_irq,
}

extern "C" {
    pub fn vfio_fsl_mc_irqs_cleanup(vdev: *mut vfio_fsl_mc_device);
}
