//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/hd.h
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
// Greybus Host Device
//
// Copyright 2014-2015 Google Inc.
// Copyright 2014-2015 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_hd_driver {
    pub hd_priv_size: usize,
    pub flags): c_ulong,
    pub cport_id): *mut *mut *mut void (cport_release)(struct gb_host_device hd, u16,
    pub flags): c_ulong,
    pub cport_id): *mut *mut *mut int (cport_disable)(struct gb_host_device hd, u16,
    pub cport_id): *mut *mut *mut int (cport_connected)(struct gb_host_device hd, u16,
    pub cport_id): *mut *mut *mut int (cport_flush)(struct gb_host_device hd, u16,
    pub timeout): u8 phase, unsigned int,
    pub timeout): size_t peer_space, unsigned int,
    pub cport_id): *mut *mut *mut int (cport_clear)(struct gb_host_device hd, u16,
    pub gfp_mask): *mut *mut gb_message message, gfp_t,
    pub message): *mut *mut void (message_cancel)(struct gb_message,
    pub cport_id): *mut *mut *mut int (latency_tag_enable)(struct gb_host_device hd, u16,
    pub cport_id): *mut *mut *mut int (latency_tag_disable)(struct gb_host_device hd, u16,
    pub async): bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_host_device {
    pub dev: device,
    pub bus_id: c_int,
    pub driver: *const gb_hd_driver,
    pub modules: list_head,
    pub connections: list_head,
    pub cport_id_map: ida,
// Number of CPorts supported by the UniPro IP
    pub num_cports: usize,
// Host device buffer constraints
    pub buffer_size_max: usize,
    pub svc: *mut gb_svc,
// Private data for the host driver
    pub __aligned(sizeof(s64)): unsigned long hd_priv[],
}

extern "C" {
    pub fn gb_hd_cport_reserve(hd: *mut gb_host_device, cport_id: u16) -> c_int;
}
extern "C" {
    pub fn gb_hd_cport_release_reserved(hd: *mut gb_host_device, cport_id: u16);
}
extern "C" {
    pub fn gb_hd_cport_release(hd: *mut gb_host_device, cport_id: u16);
}
extern "C" {
    pub fn gb_hd_add(hd: *mut gb_host_device) -> c_int;
}
extern "C" {
    pub fn gb_hd_del(hd: *mut gb_host_device);
}
extern "C" {
    pub fn gb_hd_shutdown(hd: *mut gb_host_device);
}
extern "C" {
    pub fn gb_hd_put(hd: *mut gb_host_device);
}
extern "C" {
    pub fn gb_hd_init() -> c_int;
}
extern "C" {
    pub fn gb_hd_exit();
}
