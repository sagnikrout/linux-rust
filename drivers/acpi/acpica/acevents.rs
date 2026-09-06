//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/acevents.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acevents.h - Event subcomponent prototypes and defines
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Conditions to trigger post enabling GPE polling:
// It is not sufficient to trigger edge-triggered GPE with specific GPE
// chips, software need to poll once after enabling.
//

//
// evevent
//
extern "C" {
    pub fn acpi_ev_initialize_events() -> acpi_status;
}
extern "C" {
    pub fn acpi_ev_install_xrupt_handlers() -> acpi_status;
}
extern "C" {
    pub fn acpi_ev_fixed_event_detect() -> u32;
}
//
// evmisc
//
extern "C" {
    pub fn acpi_ev_is_notify_object(node: *mut acpi_namespace_node) -> u8;
}
extern "C" {
    pub fn acpi_ev_get_gpe_number_index(gpe_number: u32) -> u32;
}
//
// evglock - Global Lock support
//
extern "C" {
    pub fn acpi_ev_init_global_lock_handler() -> acpi_status;
}
extern "C" {
    pub fn acpi_ev_remove_global_lock_handler() -> acpi_status;
}
//
// evgpe - Low-level GPE support
//
extern "C" {
    pub fn acpi_ev_gpe_detect(gpe_xrupt_list: *mut acpi_gpe_xrupt_info) -> u32;
}
extern "C" {
    pub fn acpi_ev_enable_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
}
// gpe_block);
extern "C" {
    pub fn acpi_ev_finish_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
}
//
// evgpeblk - Upper-level GPE block support
//
// gpe_block))
//
// evgpeinit - GPE initialization and update
//
extern "C" {
    pub fn acpi_ev_gpe_initialize() -> acpi_status;
}
//
// evgpeutil - GPE utilities
//
extern "C" {
    pub fn acpi_ev_delete_gpe_xrupt(gpe_xrupt: *mut acpi_gpe_xrupt_info) -> acpi_status;
}
//
// evhandler - Address space handling
//
// handler_obj);
extern "C" {
    pub fn acpi_ev_install_region_handlers() -> acpi_status;
}
//
// evregion - Operation region support
//
extern "C" {
    pub fn acpi_ev_initialize_op_regions() -> acpi_status;
}
//
// evregini - Region initialization and setup
//
extern "C" {
    pub fn acpi_ev_initialize_region(region_obj: *mut acpi_operand_object) -> acpi_status;
}
extern "C" {
    pub fn acpi_ev_is_pci_root_bridge(node: *mut acpi_namespace_node) -> u8;
}
//
// evsci - SCI (System Control Interrupt) handling/dispatch
//
extern "C" {
    pub fn acpi_ev_gpe_xrupt_handler(context: *mut c_void) -> u32 ACPI_SYSTEM_XFACE;
}
extern "C" {
    pub fn acpi_ev_sci_dispatch() -> u32;
}
extern "C" {
    pub fn acpi_ev_install_sci_handler() -> u32;
}
extern "C" {
    pub fn acpi_ev_remove_all_sci_handlers() -> acpi_status;
}
