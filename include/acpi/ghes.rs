//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/ghes.h
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
// One struct ghes is created for each generic hardware error source.
// It provides the context for APEI hardware error timer/IRQ/SCI/NMI
// handler.
//
// estatus: memory buffer for error status block, allocated during
// HEST parsing.
//
pub const GHES_EXITING: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghes {
    pub generic: *mut acpi_hest_generic,
    pub generic_v2: *mut acpi_hest_generic_v2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghes_estatus_node {
    pub llnode: llist_node,
    pub generic: *mut acpi_hest_generic,
    pub ghes: *mut ghes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghes_estatus_cache {
    pub estatus_len: u32,
    pub count: core::sync::atomic::AtomicI32,
    pub generic: *mut acpi_hest_generic,
    pub time_in: c_ulonglong,
    pub rcu: rcu_head,
}

//
// ghes_register_vendor_record_notifier - register a notifier for vendor
// records that the kernel would otherwise ignore.
// @nb: pointer to the notifier_block structure of the event handler.
//
// return 0 : SUCCESS, non-zero : FAIL
//
extern "C" {
    pub fn ghes_register_vendor_record_notifier(nb: *mut notifier_block) -> c_int;
}
//
// ghes_unregister_vendor_record_notifier - unregister the previously
// registered vendor record notifier.
// @nb: pointer to the notifier_block structure of the vendor record handler.
//
extern "C" {
    pub fn ghes_unregister_vendor_record_notifier(nb: *mut notifier_block);
}
//
// devm_ghes_register_vendor_record_notifier - device-managed vendor
// record notifier registration.
// @dev: device that owns the notifier lifetime
// @nb: pointer to the notifier_block structure of the vendor record handler
//
// Return: 0 on success, negative errno on failure.
//
extern "C" {
    pub fn ghes_estatus_pool_region_free(addr: c_ulong, size: u32);
}

extern "C" {
    pub fn ghes_estatus_pool_init(num_ghes: c_uint) -> c_int;
}
extern "C" {
    pub fn sizeof(acpi_hest_generic_data_v300: struct) -> return;
}
extern "C" {
    pub fn sizeof(acpi_hest_generic_data: struct) -> return;
}

extern "C" {
    pub fn ghes_notify_sea() -> c_int;
}

extern "C" {
    pub fn ghes_register_report_chain(nb: *mut notifier_block);
}
extern "C" {
    pub fn ghes_unregister_report_chain(nb: *mut notifier_block);
}
