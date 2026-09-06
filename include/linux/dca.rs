//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dca.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright(c) 2007 - 2009 Intel Corporation. All rights reserved.
//

// DCA Provider API
// DCA Notifier Interface
extern "C" {
    pub fn dca_register_notify(nb: *mut notifier_block);
}
extern "C" {
    pub fn dca_unregister_notify(nb: *mut notifier_block);
}
pub const DCA_PROVIDER_ADD: c_uint = 0x0001;
pub const DCA_PROVIDER_REMOVE: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dca_provider {
    pub node: list_head,
    pub ops: *const dca_ops,
    pub cd: *mut device,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dca_domain {
    pub node: list_head,
    pub dca_providers: list_head,
    pub pci_rc: *mut pci_bus,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dca_ops {
    pub ): *mut *mut *mut int (add_requester) (struct dca_provider , struct device,
    pub ): *mut *mut *mut int (remove_requester) (struct dca_provider , struct device,
    pub cpu): c_int,
    pub ): *mut *mut *mut int (dev_managed) (struct dca_provider , struct device,
}

extern "C" {
    pub fn free_dca_provider(dca: *mut dca_provider);
}
extern "C" {
    pub fn register_dca_provider(dca: *mut dca_provider, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn unregister_dca_provider(dca: *mut dca_provider, dev: *mut device);
}
// Requester API
// Macro flag: #define DCA_GET_TAG_TWO_ARGS
extern "C" {
    pub fn dca_add_requester(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dca_remove_requester(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dca_get_tag(cpu: c_int) -> u8;
}
extern "C" {
    pub fn dca3_get_tag(dev: *mut device, cpu: c_int) -> u8;
}
// internal stuff
extern "C" {
    pub fn dca_sysfs_init() -> int __init;
}
extern "C" {
    pub fn dca_sysfs_exit() -> void __exit;
}
extern "C" {
    pub fn dca_sysfs_add_provider(dca: *mut dca_provider, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn dca_sysfs_remove_provider(dca: *mut dca_provider);
}
extern "C" {
    pub fn dca_sysfs_add_req(dca: *mut dca_provider, dev: *mut device, slot: c_int) -> c_int;
}
extern "C" {
    pub fn dca_sysfs_remove_req(dca: *mut dca_provider, slot: c_int);
}
