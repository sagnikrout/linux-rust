//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pci_hotplug.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// PCI HotPlug Core Functions
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
//
// All rights reserved.
//
// Send feedback to <kristen.c.accardi@intel.com>
//
// struct hotplug_slot_ops -the callbacks that the hotplug pci core can use
// @enable_slot: Called when the user wants to enable a specific pci slot
// @disable_slot: Called when the user wants to disable a specific pci slot
// @set_attention_status: Called to set the specific slot's attention LED to
// the specified value
// @hardware_test: Called to run a specified hardware test on the specified
// slot.
// @get_power_status: Called to get the current power status of a slot.
// @get_attention_status: Called to get the current attention status of a slot.
// @get_latch_status: Called to get the current latch status of a slot.
// @get_adapter_status: Called to get see if an adapter is present in the slot or not.
// @reset_slot: Optional interface to allow override of a bus reset for the
// slot for cases where a secondary bus reset can result in spurious
// hotplug events or where a slot can be reset independent of the bus.
//
// The table of function pointers that is passed to the hotplug pci core by a
// hotplug pci driver.  These functions are called by the hotplug pci core when
// the user wants to do something to a specific slot (query it for information,
// set an LED, enable / disable power, etc.)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hotplug_slot_ops {
    pub slot): *mut *mut int (enable_slot) (struct hotplug_slot,
    pub slot): *mut *mut int (disable_slot) (struct hotplug_slot,
    pub value): *mut *mut *mut int (set_attention_status) (struct hotplug_slot slot, u8,
    pub value): *mut *mut *mut int (hardware_test) (struct hotplug_slot slot, u32,
    pub value): *mut *mut *mut int (get_power_status) (struct hotplug_slot slot, u8,
    pub value): *mut *mut *mut int (get_attention_status) (struct hotplug_slot slot, u8,
    pub value): *mut *mut *mut int (get_latch_status) (struct hotplug_slot slot, u8,
    pub value): *mut *mut *mut int (get_adapter_status) (struct hotplug_slot slot, u8,
    pub probe): *mut *mut *mut int (reset_slot) (struct hotplug_slot slot, bool,
}

//
// struct hotplug_slot - used to register a physical slot with the hotplug pci core
// @ops: pointer to the &struct hotplug_slot_ops to be used for this slot
// @pci_slot: represents a physical slot
// @owner: The module owner of this structure
// @mod_name: The module name (KBUILD_MODNAME) of this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hotplug_slot {
    pub ops: *const hotplug_slot_ops,
// Variables below this are for use only by the hotplug pci core.
    pub pci_slot: *mut pci_slot,
    pub owner: *mut module,
    pub mod_name: *const c_char,
}

extern "C" {
    pub fn pci_slot_name(_arg: slot->pci_slot) -> return;
}
extern "C" {
    pub fn pci_hp_add(slot: *mut hotplug_slot) -> c_int;
}
extern "C" {
    pub fn pci_hp_del(slot: *mut hotplug_slot);
}
extern "C" {
    pub fn pci_hp_destroy(slot: *mut hotplug_slot);
}
extern "C" {
    pub fn pci_hp_deregister(slot: *mut hotplug_slot);
}
// use a define to avoid include chaining to get THIS_MODULE & friends

extern "C" {
    pub fn pciehp_is_native(bridge: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn acpi_get_hp_hw_control_from_firmware(bridge: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn shpchp_is_native(bridge: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn acpi_pci_check_ejectable(pbus: *mut pci_bus, handle: acpi_handle) -> c_int;
}
extern "C" {
    pub fn acpi_pci_detect_ejectable(handle: acpi_handle) -> c_int;
}

