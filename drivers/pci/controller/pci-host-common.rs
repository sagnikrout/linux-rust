//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/controller/pci-host-common.h
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
// Common library for PCI host controller drivers
//
// Copyright (C) 2014 ARM Limited
//
// Author: Will Deacon <will.deacon@arm.com>
//

//
// struct pci_host_perst - PERST# GPIO descriptor
// @list: List node for linking multiple PERST# GPIOs
// @desc: GPIO descriptor for PERST# signal
//
// This structure holds a single PERST# GPIO descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_host_perst {
    pub list: list_head,
    pub desc: *mut gpio_desc,
}

//
// struct pci_host_port - Generic Root Port properties
// @list: List node for linking multiple ports
// @perst: List of PERST# GPIO descriptors for this port and its children
//
// This structure contains common properties that can be parsed from
// Root Port device tree nodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_host_port {
    pub list: list_head,
    pub perst: list_head,
}

extern "C" {
    pub fn pci_host_common_delete_ports(data: *mut c_void);
}
extern "C" {
    pub fn pci_host_common_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn pci_host_common_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn pci_host_handle_link_down(port: *mut pci_dev);
}
//
// pci_host_common_link_train_delay - Wait 100 ms if link speed > 5 GT/s
// @max_link_speed: the maximum link speed (2 = 5.0 GT/s, 3 = 8.0 GT/s, ...)
//
// Must be called after Link training completes and before the first
// Configuration Request is sent.
//
