//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/achware.h
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
// Name: achware.h -- hardware specific interfaces
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Values for the _SST predefined method
pub const ACPI_SST_INDICATOR_OFF: c_int = 0;
pub const ACPI_SST_WORKING: c_int = 1;
pub const ACPI_SST_WAKING: c_int = 2;
pub const ACPI_SST_SLEEPING: c_int = 3;
pub const ACPI_SST_SLEEP_CONTEXT: c_int = 4;
//
// hwacpi - high level functions
//
extern "C" {
    pub fn acpi_hw_set_mode(mode: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_get_mode() -> u32;
}
//
// hwregs - ACPI Register I/O
//
extern "C" {
    pub fn acpi_hw_read(value: *mut u64, reg: *mut acpi_generic_address) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_write(value: u64, reg: *mut acpi_generic_address) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_write_pm1_control(pm1a_control: u32, pm1b_control: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_register_read(register_id: u32, return_value: *mut u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_register_write(register_id: u32, value: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_clear_acpi_status() -> acpi_status;
}
//
// hwsleep - sleep/wake support (Legacy sleep registers)
//
extern "C" {
    pub fn acpi_hw_legacy_sleep(sleep_state: u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_legacy_wake_prep(sleep_state: u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_legacy_wake(sleep_state: u8) -> acpi_status;
}
//
// hwesleep - sleep/wake support (Extended FADT-V5 sleep registers)
//
extern "C" {
    pub fn acpi_hw_execute_sleep_method(method_name: *mut c_char, integer_argument: u32);
}
extern "C" {
    pub fn acpi_hw_extended_sleep(sleep_state: u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_extended_wake_prep(sleep_state: u8) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_extended_wake(sleep_state: u8) -> acpi_status;
}
//
// hwvalid - Port I/O with validation
//
extern "C" {
    pub fn acpi_hw_read_port(address: acpi_io_address, value: *mut u32, width: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_write_port(address: acpi_io_address, value: u32, width: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_validate_io_block(address: u64, bit_width: u32, count: u32) -> acpi_status;
}
//
// hwgpe - GPE support
//
extern "C" {
    pub fn acpi_hw_gpe_read(value: *mut u64, reg: *mut acpi_gpe_address) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_gpe_write(value: u64, reg: *mut acpi_gpe_address) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_get_gpe_register_bit(gpe_event_info: *mut acpi_gpe_event_info) -> u32;
}
extern "C" {
    pub fn acpi_hw_clear_gpe(gpe_event_info: *mut acpi_gpe_event_info) -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_enable_all_runtime_gpes() -> acpi_status;
}
extern "C" {
    pub fn acpi_hw_check_all_gpes(gpe_skip_device: acpi_handle, gpe_skip_number: u32) -> u8;
}

//
// hwpci - PCI configuration support
//

