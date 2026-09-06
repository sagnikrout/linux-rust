//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/control.h
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
// Greybus CPort control protocol
//
// Copyright 2015 Google Inc.
// Copyright 2015 Linaro Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control {
    pub dev: device,
    pub intf: *mut gb_interface,
    pub connection: *mut gb_connection,
    pub protocol_major: u8,
    pub protocol_minor: u8,
    pub has_bundle_activate: bool,
    pub has_bundle_version: bool,
    pub vendor_string: *mut c_char,
    pub product_string: *mut c_char,
}

extern "C" {
    pub fn gb_control_enable(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_disable(control: *mut gb_control);
}
extern "C" {
    pub fn gb_control_suspend(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_resume(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_add(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_del(control: *mut gb_control);
}
extern "C" {
    pub fn gb_control_put(control: *mut gb_control);
}
extern "C" {
    pub fn gb_control_get_bundle_versions(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_connected_operation(control: *mut gb_control, cport_id: u16) -> c_int;
}
extern "C" {
    pub fn gb_control_disconnected_operation(control: *mut gb_control, cport_id: u16) -> c_int;
}
extern "C" {
    pub fn gb_control_mode_switch_operation(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_mode_switch_prepare(control: *mut gb_control);
}
extern "C" {
    pub fn gb_control_mode_switch_complete(control: *mut gb_control);
}
extern "C" {
    pub fn gb_control_get_manifest_size_operation(intf: *mut gb_interface) -> c_int;
}
extern "C" {
    pub fn gb_control_bundle_suspend(control: *mut gb_control, bundle_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_control_bundle_resume(control: *mut gb_control, bundle_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_control_bundle_deactivate(control: *mut gb_control, bundle_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_control_bundle_activate(control: *mut gb_control, bundle_id: u8) -> c_int;
}
extern "C" {
    pub fn gb_control_interface_suspend_prepare(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_interface_deactivate_prepare(control: *mut gb_control) -> c_int;
}
extern "C" {
    pub fn gb_control_interface_hibernate_abort(control: *mut gb_control) -> c_int;
}
