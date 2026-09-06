//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci-acpi.h
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
// File		pci-acpi.h
//
// Copyright (C) 2004 Intel
// Copyright (C) Tom Long Nguyen (tom.l.nguyen@intel.com)
//

extern "C" {
    pub fn acpi_remove_pm_notifier(_arg: dev) -> return;
}
extern "C" {
    pub fn acpi_remove_pm_notifier(_arg: dev) -> return;
}
extern "C" {
    pub fn acpi_pci_root_get_mcfg_addr(handle: acpi_handle) -> phys_addr_t;
}
// Find a PCI root bus
extern "C" {
    pub fn ACPI_HANDLE(_arg: pbus->bridge) -> return;
}
// If pbus is a virtual bus, there is no bridge to it
extern "C" {
    pub fn ACPI_HANDLE(_arg: dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_root_info {
    pub root: *mut acpi_pci_root,
    pub bridge: *mut acpi_device,
    pub ops: *mut acpi_pci_root_ops,
    pub resources: list_head,
    pub name: [c_char; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_root_ops {
    pub pci_ops: *mut pci_ops,
    pub info): *mut *mut int (init_info)(struct acpi_pci_root_info,
    pub info): *mut *mut void (release_info)(struct acpi_pci_root_info,
    pub info): *mut *mut int (prepare_resources)(struct acpi_pci_root_info,
}

extern "C" {
    pub fn acpi_pci_probe_root_resources(info: *mut acpi_pci_root_info) -> c_int;
}
extern "C" {
    pub fn acpi_pci_add_bus(bus: *mut pci_bus);
}
extern "C" {
    pub fn acpi_pci_remove_bus(bus: *mut pci_bus);
}

extern "C" {
    pub fn pci_acpi_setup(dev: *mut device, adev: *mut acpi_device);
}
extern "C" {
    pub fn pci_acpi_cleanup(dev: *mut device, adev: *mut acpi_device);
}

extern "C" {
    pub fn acpi_pci_slot_init();
}
extern "C" {
    pub fn acpi_pci_slot_enumerate(bus: *mut pci_bus);
}
extern "C" {
    pub fn acpi_pci_slot_remove(bus: *mut pci_bus);
}

extern "C" {
    pub fn acpiphp_init();
}
extern "C" {
    pub fn acpiphp_enumerate_slots(bus: *mut pci_bus);
}
extern "C" {
    pub fn acpiphp_remove_slots(bus: *mut pci_bus);
}
extern "C" {
    pub fn acpiphp_check_host_bridge(adev: *mut acpi_device);
}

// _DSM Definitions for PCI
pub const DSM_PCI_PRESERVE_BOOT_CONFIG: c_uint = 0x05;
pub const DSM_PCI_DEVICE_NAME: c_uint = 0x07;
pub const DSM_PCI_POWER_ON_RESET_DELAY: c_uint = 0x08;
pub const DSM_PCI_DEVICE_READINESS_DURATIONS: c_uint = 0x09;

extern "C" {
    pub fn pci_acpi_add_edr_notifier(pdev: *mut pci_dev);
}
extern "C" {
    pub fn pci_acpi_remove_edr_notifier(pdev: *mut pci_dev);
}

extern "C" {
    pub fn pci_acpi_set_companion_lookup_hook(): *mut *mut *mut acpi_device (func)(pci_dev) -> c_int;
}
extern "C" {
    pub fn pci_acpi_clear_companion_lookup_hook();
}

