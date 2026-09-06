//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/interface.h
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
// Greybus Interface Block code
//
// Copyright 2014 Google Inc.
// Copyright 2014 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gb_interface_type {
    GB_INTERFACE_TYPE_INVALID = 0,
    GB_INTERFACE_TYPE_UNKNOWN,
    GB_INTERFACE_TYPE_DUMMY,
    GB_INTERFACE_TYPE_UNIPRO,
    GB_INTERFACE_TYPE_GREYBUS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_interface {
    pub dev: device,
    pub control: *mut gb_control,
    pub bundles: list_head,
    pub module_node: list_head,
    pub manifest_descs: list_head,
    pub /: *mut *mut u8 interface_id; / Physical location within the Endo,
    pub device_id: u8,
    pub /: *mut *mut u8 features; / Feature flags set in the manifest,
    pub type: gb_interface_type,
    pub ddbl1_manufacturer_id: u32,
    pub ddbl1_product_id: u32,
    pub vendor_id: u32,
    pub product_id: u32,
    pub serial_number: u64,
    pub hd: *mut gb_host_device,
    pub module: *mut gb_module,
    pub quirks: c_ulong,
    pub mutex: mutex,
    pub disconnected: bool,
    pub ejected: bool,
    pub removed: bool,
    pub active: bool,
    pub enabled: bool,
    pub mode_switch: bool,
    pub dme_read: bool,
    pub mode_switch_work: work_struct,
    pub mode_switch_completion: completion,
}

extern "C" {
    pub fn gb_interface_activate(intf: *mut gb_interface) -> c_int;
}
extern "C" {
    pub fn gb_interface_deactivate(intf: *mut gb_interface);
}
extern "C" {
    pub fn gb_interface_enable(intf: *mut gb_interface) -> c_int;
}
extern "C" {
    pub fn gb_interface_disable(intf: *mut gb_interface);
}
extern "C" {
    pub fn gb_interface_add(intf: *mut gb_interface) -> c_int;
}
extern "C" {
    pub fn gb_interface_del(intf: *mut gb_interface);
}
extern "C" {
    pub fn gb_interface_put(intf: *mut gb_interface);
}
extern "C" {
    pub fn gb_interface_request_mode_switch(intf: *mut gb_interface) -> c_int;
}
