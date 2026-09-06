//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/stm/stm.h
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
// System Trace Module (STM) infrastructure
// Copyright (c) 2014, Intel Corporation.
//
// STM class implements generic infrastructure for  System Trace Module devices
// as defined in MIPI STPv2 specification.
//

extern "C" {
    pub fn stp_configfs_init() -> c_int;
}
extern "C" {
    pub fn stp_configfs_exit();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stp_master {
    pub nr_free: c_uint,
    pub chan_map: [c_ulong; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_device {
    pub dev: device,
    pub owner: *mut module,
    pub policy: *mut stp_policy,
    pub policy_mutex: mutex,
    pub major: c_int,
    pub sw_nmasters: c_uint,
    pub data: *mut stm_data,
    pub link_mutex: mutex,
    pub link_lock: spinlock_t,
    pub link_list: list_head,
// framing protocol in use
    pub pdrv: *const stm_protocol_driver,
    pub pdrv_node_type: *const config_item_type,
// master allocation
    pub mc_lock: spinlock_t,
    pub masters: [*mut stp_master; ],
}

extern "C" {
    pub fn stp_policy_node_put(policy_node: *mut stp_policy_node);
}
extern "C" {
    pub fn stp_policy_unbind(policy: *mut stp_policy);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_output {
    pub lock: spinlock_t,
    pub master: c_uint,
    pub channel: c_uint,
    pub nr_chans: c_uint,
    pub pdrv_private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_file {
    pub stm: *mut stm_device,
    pub output: stm_output,
}

extern "C" {
    pub fn stm_put_device(stm: *mut stm_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_source_device {
    pub dev: device,
    pub data: *mut stm_source_data,
    pub link_lock: spinlock_t,
    pub link: *mut stm_device __rcu,
    pub link_entry: list_head,
// one output per stm_source device
    pub output: stm_output,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_protocol_driver {
    pub owner: *mut module,
    pub name: *const c_char,
    pub source): *const *const char buf, size_t count, struct stm_source_data,
    pub arg): *mut *mut void (policy_node_init)(void,
    pub output): *mut *mut *mut int (output_open)(void priv, struct stm_output,
    pub output): *mut *mut void (output_close)(struct stm_output,
    pub priv_sz: isize,
    pub policy_attr: *mut configfs_attribute,
}

extern "C" {
    pub fn stm_register_protocol(pdrv: *const stm_protocol_driver) -> c_int;
}
extern "C" {
    pub fn stm_unregister_protocol(pdrv: *const stm_protocol_driver);
}
extern "C" {
    pub fn stm_put_protocol(pdrv: *const stm_protocol_driver);
}
