//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/vdpa/vdpa_user/iova_domain.h
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
// MMU-based software IOTLB.
//
// Copyright (C) 2020-2021 Bytedance Inc. and/or its affiliates. All rights reserved.
//
// Author: Xie Yongji <xieyongji@bytedance.com>
//

pub const IOVA_START_PFN: c_int = 1;
pub const BOUNCE_MAP_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_bounce_map {
    pub bounce_page: *mut page,
    pub user_bounce_page: *mut page,
    pub orig_phys: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vduse_iova_domain {
    pub stream_iovad: iova_domain,
    pub consistent_iovad: iova_domain,
    pub bounce_maps: *mut vduse_bounce_map,
    pub bounce_size: usize,
    pub iova_limit: c_ulong,
    pub bounce_map: c_int,
    pub iotlb: *mut vhost_iotlb,
    pub iotlb_lock: spinlock_t,
    pub file: *mut file,
    pub user_bounce_pages: bool,
    pub bounce_lock: rwlock_t,
}

extern "C" {
    pub fn vduse_domain_reset_bounce_map(domain: *mut vduse_iova_domain);
}
extern "C" {
    pub fn vduse_domain_remove_user_bounce_pages(domain: *mut vduse_iova_domain);
}
extern "C" {
    pub fn vduse_domain_destroy(domain: *mut vduse_iova_domain);
}
extern "C" {
    pub fn vduse_domain_init() -> c_int;
}
extern "C" {
    pub fn vduse_domain_exit();
}
