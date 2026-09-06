//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-pwrctrl.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2024 Linaro Ltd.
//

//
// This is a simple framework for solving the issue of PCI devices that require
// certain resources (regulators, GPIOs, clocks) to be enabled before the
// device can actually be detected on the PCI bus.
//
// The idea is to reuse the platform bus to populate OF nodes describing the
// PCI device and its resources, let these platform devices probe and enable
// relevant resources and then trigger a rescan of the PCI bus allowing for the
// same device (with a second associated struct device) to be registered with
// the PCI subsystem.
//
// To preserve a correct hierarchy for PCI power management and device reset,
// we create a device link between the power control platform device (parent)
// and the supplied PCI device (child).
//
// struct pci_pwrctrl - PCI device power control context.
// @dev: Address of the power controlling device.
// @power_on: Callback to power on the power controlling device.
// @power_off: Callback to power off the power controlling device.
//
// An object of this type must be allocated by the PCI power control device and
// passed to the pwrctrl subsystem to trigger a bus rescan and setup a device
// link with the device once it's up.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_pwrctrl {
    pub dev: *mut device,
    pub pwrctrl): *mut *mut int (power_on)(struct pci_pwrctrl,
    pub pwrctrl): *mut *mut int (power_off)(struct pci_pwrctrl,
// private: internal use only
    pub nb: notifier_block,
    pub link: *mut device_link,
    pub work: work_struct,
}

extern "C" {
    pub fn pci_pwrctrl_init(pwrctrl: *mut pci_pwrctrl, dev: *mut device);
}
extern "C" {
    pub fn pci_pwrctrl_device_set_ready(pwrctrl: *mut pci_pwrctrl) -> c_int;
}
extern "C" {
    pub fn pci_pwrctrl_device_unset_ready(pwrctrl: *mut pci_pwrctrl);
}

extern "C" {
    pub fn pci_pwrctrl_create_devices(parent: *mut device) -> c_int;
}
extern "C" {
    pub fn pci_pwrctrl_destroy_devices(parent: *mut device);
}
extern "C" {
    pub fn pci_pwrctrl_power_on_devices(parent: *mut device) -> c_int;
}
extern "C" {
    pub fn pci_pwrctrl_power_off_devices(parent: *mut device);
}

