//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/rmi4/rmi_driver.h
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
// Copyright (c) 2011-2016 Synaptics Incorporated
// Copyright (c) 2011 Unixphere
//

pub const SYNAPTICS_VENDOR_ID: c_uint = 0x06cb;

pub const PDT_PROPERTIES_LOCATION: c_uint = 0x00EF;
pub const BSR_LOCATION: c_uint = 0x00FE;
pub const RMI_PDT_PROPS_HAS_BSR: c_uint = 0x02;
pub const NAME_BUFFER_SIZE: c_int = 256;
pub const RMI_PDT_ENTRY_SIZE: c_int = 6;
pub const RMI_PDT_FUNCTION_VERSION_MASK: c_uint = 0x60;
pub const RMI_PDT_INT_SOURCE_COUNT_MASK: c_uint = 0x07;
pub const PDT_START_SCAN_LOCATION: c_uint = 0x00e9;
pub const PDT_END_SCAN_LOCATION: c_uint = 0x0005;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pdt_entry {
    pub page_start: u16,
    pub query_base_addr: u8,
    pub command_base_addr: u8,
    pub control_base_addr: u8,
    pub data_base_addr: u8,
    pub interrupt_source_count: u8,
    pub function_version: u8,
    pub function_number: u8,
}

// describes a single packet register
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_register_desc_item {
    pub reg_size: u32,
    pub reg: u16,
    pub num_subpackets: u16,
    pub RMI_REG_DESC_SUBPACKET_BITS): DECLARE_BITMAP(subpacket_map,,
}

//
// describes the packet registers for a particular type
// (ie query, control, data)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_register_descriptor {
    pub struct_size: c_ulong,
    pub num_registers: u16,
    pub registers: *mut rmi_register_desc_item,
}

//
// Calculate the total size of all of the registers described in the
// descriptor.
//
extern "C" {
    pub fn rmi_register_desc_calc_size(rdesc: *mut rmi_register_descriptor) -> usize;
}
extern "C" {
    pub fn rmi_is_physical_driver(: *const device_driver) -> bool;
}
extern "C" {
    pub fn rmi_register_physical_driver() -> c_int;
}
extern "C" {
    pub fn rmi_unregister_physical_driver();
}
extern "C" {
    pub fn rmi_free_function_list(rmi_dev: *mut rmi_device);
}
extern "C" {
    pub fn rmi_enable_sensor(rmi_dev: *mut rmi_device) -> c_int;
}
extern "C" {
    pub fn rmi_probe_interrupts(data: *mut rmi_driver_data) -> c_int;
}
extern "C" {
    pub fn rmi_enable_irq(rmi_dev: *mut rmi_device, clear_wake: bool);
}
extern "C" {
    pub fn rmi_disable_irq(rmi_dev: *mut rmi_device, enable_wake: bool);
}
extern "C" {
    pub fn rmi_init_functions(data: *mut rmi_driver_data) -> c_int;
}

extern "C" {
    pub fn rmi_f03_commit_buttons(fn: *mut rmi_function);
}

extern "C" {
    pub fn rmi_f34_create_sysfs(rmi_dev: *mut rmi_device) -> c_int;
}
extern "C" {
    pub fn rmi_f34_remove_sysfs(rmi_dev: *mut rmi_device);
}

