//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/rpaphp.h
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
// PCI Hot Plug Controller Driver for RPA-compliant PPC64 platform.
//
// Copyright (C) 2003 Linda Xie <lxie@us.ibm.com>
//
// All rights reserved.
//
// Send feedback to <lxie@us.ibm.com>,
//

pub const DR_INDICATOR: c_int = 9002;
pub const DR_ENTITY_SENSE: c_int = 9003;
pub const POWER_ON: c_int = 100;
pub const POWER_OFF: c_int = 0;
pub const LED_OFF: c_int = 0;

// Sensor values from rtas_get-sensor

// slot states
pub const NOT_VALID: c_int = 3;
pub const NOT_CONFIGURED: c_int = 2;
pub const CONFIGURED: c_int = 1;
pub const EMPTY: c_int = 0;
// DRC constants
pub const MAX_DRC_NAME_LEN: c_int = 64;
//
// struct slot - slot information for each *physical* slot
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub rpaphp_slot_list: list_head,
    pub state: c_int,
    pub index: u32,
    pub type: u32,
    pub power_domain: u32,
    pub attention_status: u8,
    pub name: *mut c_char,
    pub dn: *mut device_node,
    pub bus: *mut pci_bus,
    pub pci_devs: *mut list_head,
    pub hotplug_slot: hotplug_slot,
}

extern "C" {
    pub fn container_of(_arg: hotplug_slot, slot: struct, _arg: hotplug_slot) -> return;
}
// function prototypes
// rpaphp_pci.c
extern "C" {
    pub fn rpaphp_enable_slot(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn rpaphp_get_sensor_state(slot: *mut slot, state: *mut c_int) -> c_int;
}
// rpaphp_core.c
extern "C" {
    pub fn rpaphp_add_slot(dn: *mut device_node) -> c_int;
}
// rpaphp_slot.c
extern "C" {
    pub fn dealloc_slot_struct(slot: *mut slot);
}
extern "C" {
    pub fn rpaphp_register_slot(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn rpaphp_deregister_slot(slot: *mut slot) -> c_int;
}
