//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pnv-pci.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2014 IBM Corp.
//

extern "C" {
    pub fn pnv_pci_get_slot_id(np: *mut device_node, id: *mut u64) -> c_int;
}
extern "C" {
    pub fn pnv_pci_get_device_tree(phandle: u32, buf: *mut c_void, len: u64) -> c_int;
}
extern "C" {
    pub fn pnv_pci_get_presence_state(id: u64, state: *mut u8) -> c_int;
}
extern "C" {
    pub fn pnv_pci_get_power_state(id: u64, state: *mut u8) -> c_int;
}
extern "C" {
    pub fn pnv_opal_pci_msi_eoi(d: *mut irq_data) -> i64;
}
extern "C" {
    pub fn is_pnv_opal_msi(chip: *mut irq_chip) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnv_php_slot {
    pub slot: hotplug_slot,
    pub id: u64,
    pub name: *mut c_char,
    pub slot_no: c_int,
    pub flags: c_uint,
pub const PNV_PHP_FLAG_BROKEN_PDC: c_uint = 0x1;
    pub kref: kref,
pub const PNV_PHP_STATE_INITIALIZED: c_int = 0;
pub const PNV_PHP_STATE_REGISTERED: c_int = 1;
pub const PNV_PHP_STATE_POPULATED: c_int = 2;
pub const PNV_PHP_STATE_OFFLINE: c_int = 3;
    pub state: c_int,
    pub irq: c_int,
    pub wq: *mut workqueue_struct,
    pub dn: *mut device_node,
    pub pdev: *mut pci_dev,
    pub bus: *mut pci_bus,
    pub power_state_check: bool,
    pub attention_state: u8,
    pub fdt: *mut c_void,
    pub dt: *mut c_void,
    pub ocs: of_changeset,
    pub parent: *mut pnv_php_slot,
    pub children: list_head,
    pub link: list_head,
}
