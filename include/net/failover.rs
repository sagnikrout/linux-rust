//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/failover.h
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
// Copyright (c) 2018, Intel Corporation.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct failover_ops {
    pub failover_dev): *mut net_device,
    pub failover_dev): *mut net_device,
    pub failover_dev): *mut net_device,
    pub failover_dev): *mut net_device,
    pub failover_dev): *mut net_device,
    pub failover_dev): *mut net_device,
    pub pskb): *mut *mut rx_handler_result_t (slave_handle_frame)(struct sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct failover {
    pub list: list_head,
    pub failover_dev: *mut net_device __rcu,
    pub dev_tracker: netdevice_tracker,
    pub ops: *mut failover_ops __rcu,
}

extern "C" {
    pub fn failover_unregister(failover: *mut failover);
}
extern "C" {
    pub fn failover_slave_unregister(slave_dev: *mut net_device) -> c_int;
}
