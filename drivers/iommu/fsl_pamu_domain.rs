//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/fsl_pamu_domain.h
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
// Copyright (C) 2013 Freescale Semiconductor, Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_dma_domain {
// list of devices associated with the domain
    pub devices: list_head,
    pub stash_id: u32,
    pub iommu_domain: iommu_domain,
    pub domain_lock: spinlock_t,
}

// domain-device relationship
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_domain_info {
    pub /: *mut *mut list_head link; / link to domain siblings,
    pub dev: *mut device,
    pub liodn: u32,
    pub /: *mut *mut *mut fsl_dma_domain domain; / pointer to domain,
}
