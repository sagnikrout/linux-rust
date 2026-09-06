//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/acpiphp.h
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
// ACPI PCI Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
// Copyright (C) 2002 Hiroshi Aono (h-aono@ap.jp.nec.com)
// Copyright (C) 2002,2003 Takayoshi Kochi (t-kochi@bq.jp.nec.com)
// Copyright (C) 2002,2003 NEC Corporation
// Copyright (C) 2003-2005 Matthew Wilcox (willy@infradead.org)
// Copyright (C) 2003-2005 Hewlett Packard
//
// All rights reserved.
//
// Send feedback to <gregkh@us.ibm.com>,
// <t-kochi@bq.jp.nec.com>
//

//
// struct slot - slot information for each *physical* slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub hotplug_slot: hotplug_slot,
    pub acpi_slot: *mut acpiphp_slot,
    pub /: *mut *mut unsigned int sun; / ACPI _SUN (Slot User Number) value,
}

extern "C" {
    pub fn hotplug_slot_name(_arg: &slot->hotplug_slot) -> return;
}
extern "C" {
    pub fn container_of(_arg: hotplug_slot, slot: struct, _arg: hotplug_slot) -> return;
}
//
// struct acpiphp_bridge - PCI bridge information
//
// for each bridge device in ACPI namespace
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_bridge {
    pub list: list_head,
    pub slots: list_head,
    pub ref: kref,
    pub context: *mut acpiphp_context,
    pub nr_slots: c_int,
// This bus (host bridge) or Secondary bus (PCI-to-PCI bridge)
    pub pci_bus: *mut pci_bus,
// PCI-to-PCI bridge device
    pub pci_dev: *mut pci_dev,
    pub is_going_away: bool,
}

//
// struct acpiphp_slot - PCI slot information
//
// PCI slot information for each *physical* PCI slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_slot {
    pub node: list_head,
    pub bus: *mut pci_bus,
    pub different: *mut *mut list_head funcs; / one slot may have,
    pub slot: *mut slot,
    pub /: *mut *mut u8 device; / pci device#,
    pub /: *mut *mut u32 flags; / see below,
}

//
// struct acpiphp_func - PCI function information
//
// PCI function information for each object in ACPI namespace
// typically 8 objects per slot (i.e. for each PCI function)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_func {
    pub parent: *mut acpiphp_bridge,
    pub slot: *mut acpiphp_slot,
    pub sibling: list_head,
    pub /: *mut *mut u8 function; / pci function#,
    pub /: *mut *mut u32 flags; / see below,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_context {
    pub hp: acpi_hotplug_context,
    pub func: acpiphp_func,
    pub bridge: *mut acpiphp_bridge,
    pub refcount: c_uint,
}

extern "C" {
    pub fn container_of(_arg: hp, acpiphp_context: struct, _arg: hp) -> return;
}
extern "C" {
    pub fn container_of(_arg: func, acpiphp_context: struct, _arg: func) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_root_context {
    pub hp: acpi_hotplug_context,
    pub root_bridge: *mut acpiphp_bridge,
}

extern "C" {
    pub fn container_of(_arg: hp, acpiphp_root_context: struct, _arg: hp) -> return;
}
//
// struct acpiphp_attention_info - device specific attention registration
//
// ACPI has no generic method of setting/getting attention status
// this allows for device specific driver registration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpiphp_attention_info {
    pub status): *mut *mut *mut int (set_attn)(struct hotplug_slot slot, u8,
    pub status): *mut *mut *mut int (get_attn)(struct hotplug_slot slot, u8,
    pub owner: *mut module,
}

// ACPI _STA method value (ignore bit 4; battery present)

// slot flags

// function flags

// function prototypes
// acpiphp_core.c
extern "C" {
    pub fn acpiphp_register_attention(info: *mut acpiphp_attention_info) -> c_int;
}
extern "C" {
    pub fn acpiphp_unregister_attention(info: *mut acpiphp_attention_info) -> c_int;
}
extern "C" {
    pub fn acpiphp_register_hotplug_slot(slot: *mut acpiphp_slot, sun: c_uint) -> c_int;
}
extern "C" {
    pub fn acpiphp_unregister_hotplug_slot(slot: *mut acpiphp_slot);
}
extern "C" {
    pub fn acpiphp_enable_slot(slot: *mut acpiphp_slot) -> c_int;
}
extern "C" {
    pub fn acpiphp_disable_slot(slot: *mut acpiphp_slot) -> c_int;
}
extern "C" {
    pub fn acpiphp_get_power_status(slot: *mut acpiphp_slot) -> u8;
}
extern "C" {
    pub fn acpiphp_get_latch_status(slot: *mut acpiphp_slot) -> u8;
}
extern "C" {
    pub fn acpiphp_get_adapter_status(slot: *mut acpiphp_slot) -> u8;
}
// variables
