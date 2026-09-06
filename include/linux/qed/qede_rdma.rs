//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qede_rdma.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qedr NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qede_rdma_event {
    QEDE_UP,
    QEDE_DOWN,
    QEDE_CHANGE_ADDR,
    QEDE_CLOSE,
    QEDE_CHANGE_MTU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_rdma_event_work {
    pub list: list_head,
    pub work: work_struct,
    pub ptr: *mut c_void,
    pub event: qede_rdma_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedr_driver {
    pub name: [c_uchar; 32],
    pub ): *mut net_device,
    pub ): *mut *mut void (remove)(struct qedr_dev,
    pub qede_rdma_event): *mut *mut *mut void (notify)(struct qedr_dev , enum,
}

// APIs for RDMA driver to register callback handlers,
// which will be invoked when device is added, removed, ifup, ifdown
//
extern "C" {
    pub fn qede_rdma_register_driver(drv: *mut qedr_driver) -> c_int;
}
extern "C" {
    pub fn qede_rdma_unregister_driver(drv: *mut qedr_driver);
}
extern "C" {
    pub fn qede_rdma_supported(dev: *mut qede_dev) -> bool;
}

extern "C" {
    pub fn qede_rdma_dev_add(dev: *mut qede_dev, recovery: bool) -> c_int;
}
extern "C" {
    pub fn qede_rdma_dev_event_open(dev: *mut qede_dev);
}
extern "C" {
    pub fn qede_rdma_dev_event_close(dev: *mut qede_dev);
}
extern "C" {
    pub fn qede_rdma_dev_remove(dev: *mut qede_dev, recovery: bool);
}
extern "C" {
    pub fn qede_rdma_event_changeaddr(edr: *mut qede_dev);
}
extern "C" {
    pub fn qede_rdma_event_change_mtu(edev: *mut qede_dev);
}

