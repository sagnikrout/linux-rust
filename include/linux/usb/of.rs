//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/of.h
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
// OF helpers for usb devices.
//

extern "C" {
    pub fn of_usb_get_dr_mode_by_phy(np: *mut device_node, arg0: c_int) -> usb_dr_mode;
}
extern "C" {
    pub fn of_usb_host_tpl_support(np: *mut device_node) -> bool;
}
extern "C" {
    pub fn usb_of_get_connect_type(hub: *mut usb_device, port1: c_int) -> usb_port_connect_type;
}
extern "C" {
    pub fn usb_of_has_combined_node(udev: *mut usb_device) -> bool;
}

extern "C" {
    pub fn of_usb_get_phy_mode(np: *mut device_node) -> usb_phy_interface;
}

