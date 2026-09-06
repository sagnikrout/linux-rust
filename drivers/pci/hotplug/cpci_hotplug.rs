//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/cpci_hotplug.h
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
// CompactPCI Hot Plug Core Functions
//
// Copyright (C) 2002 SOMA Networks, Inc.
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM Corp.
//
// All rights reserved.
//
// Send feedback to <scottm@somanetworks.com>
//

// PICMG 2.1 R2.0 HS CSR bits:
pub const HS_CSR_INS: c_uint = 0x0080;
pub const HS_CSR_EXT: c_uint = 0x0040;
pub const HS_CSR_PI: c_uint = 0x0030;
pub const HS_CSR_LOO: c_uint = 0x0008;
pub const HS_CSR_PIE: c_uint = 0x0004;
pub const HS_CSR_EIM: c_uint = 0x0002;
pub const HS_CSR_DHA: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub number: u8,
    pub devfn: c_uint,
    pub bus: *mut pci_bus,
    pub dev: *mut pci_dev,
    pub latch_status:1: c_uint,
    pub adapter_status:1: c_uint,
    pub extracting: c_uint,
    pub hotplug_slot: hotplug_slot,
    pub slot_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpci_hp_controller_ops {
    pub (*query_enum)(void): *mut c_int,
    pub (*enable_irq)(void): *mut c_int,
    pub (*disable_irq)(void): *mut c_int,
    pub dev_id): *mut *mut int (check_irq)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpci_hp_controller {
    pub irq: c_uint,
    pub irq_flags: c_ulong,
    pub devname: *mut c_char,
    pub dev_id: *mut c_void,
    pub name: *mut c_char,
    pub ops: *mut cpci_hp_controller_ops,
}

extern "C" {
    pub fn hotplug_slot_name(_arg: &slot->hotplug_slot) -> return;
}
extern "C" {
    pub fn container_of(_arg: hotplug_slot, slot: struct, _arg: hotplug_slot) -> return;
}
extern "C" {
    pub fn cpci_hp_register_controller(controller: *mut cpci_hp_controller) -> c_int;
}
extern "C" {
    pub fn cpci_hp_unregister_controller(controller: *mut cpci_hp_controller) -> c_int;
}
extern "C" {
    pub fn cpci_hp_register_bus(bus: *mut pci_bus, first: u8, last: u8) -> c_int;
}
extern "C" {
    pub fn cpci_hp_unregister_bus(bus: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn cpci_hp_start() -> c_int;
}
extern "C" {
    pub fn cpci_hp_stop() -> c_int;
}
// Global variables
//
// Internal function prototypes, these functions should not be used by
// board/chassis drivers.
//
extern "C" {
    pub fn cpci_get_attention_status(slot: *mut slot) -> u8;
}
extern "C" {
    pub fn cpci_get_hs_csr(slot: *mut slot) -> u16;
}
extern "C" {
    pub fn cpci_set_attention_status(slot: *mut slot, status: c_int) -> c_int;
}
extern "C" {
    pub fn cpci_check_and_clear_ins(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_check_ext(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_clear_ext(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_led_on(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_led_off(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_configure_slot(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn cpci_unconfigure_slot(slot: *mut slot) -> c_int;
}

extern "C" {
    pub fn cpci_hotplug_init(debug: c_int) -> c_int;
}

