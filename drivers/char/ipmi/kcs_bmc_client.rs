//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/ipmi/kcs_bmc_client.h
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
// Copyright (c) 2021, IBM Corp.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_driver_ops {
    pub kcs_bmc): *mut *mut int (add_device)(struct kcs_bmc_device,
    pub kcs_bmc): *mut *mut int (remove_device)(struct kcs_bmc_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_driver {
    pub entry: list_head,
    pub ops: *const kcs_bmc_driver_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_client_ops {
    pub client): *mut *mut irqreturn_t (event)(struct kcs_bmc_client,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcs_bmc_client {
    pub ops: *const kcs_bmc_client_ops,
    pub dev: *mut kcs_bmc_device,
}

extern "C" {
    pub fn kcs_bmc_register_driver(drv: *mut kcs_bmc_driver);
}
extern "C" {
    pub fn kcs_bmc_unregister_driver(drv: *mut kcs_bmc_driver);
}
extern "C" {
    pub fn kcs_bmc_enable_device(kcs_bmc: *mut kcs_bmc_device, client: *mut kcs_bmc_client) -> c_int;
}
extern "C" {
    pub fn kcs_bmc_disable_device(kcs_bmc: *mut kcs_bmc_device, client: *mut kcs_bmc_client);
}
extern "C" {
    pub fn kcs_bmc_update_event_mask(kcs_bmc: *mut kcs_bmc_device, mask: u8, events: u8);
}
extern "C" {
    pub fn kcs_bmc_read_data(kcs_bmc: *mut kcs_bmc_device) -> u8;
}
extern "C" {
    pub fn kcs_bmc_write_data(kcs_bmc: *mut kcs_bmc_device, data: u8);
}
extern "C" {
    pub fn kcs_bmc_read_status(kcs_bmc: *mut kcs_bmc_device) -> u8;
}
extern "C" {
    pub fn kcs_bmc_write_status(kcs_bmc: *mut kcs_bmc_device, data: u8);
}
extern "C" {
    pub fn kcs_bmc_update_status(kcs_bmc: *mut kcs_bmc_device, mask: u8, val: u8);
}
