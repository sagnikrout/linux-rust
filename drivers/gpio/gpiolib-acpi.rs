//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpio/gpiolib-acpi.h
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
// ACPI helpers for GPIO API
//
// Copyright (C) 2012,2019 Intel Corporation
//

extern "C" {
    pub fn acpi_gpiochip_add(chip: *mut gpio_chip);
}
extern "C" {
    pub fn acpi_gpiochip_remove(chip: *mut gpio_chip);
}
extern "C" {
    pub fn acpi_gpiochip_request_interrupts(chip: *mut gpio_chip);
}
extern "C" {
    pub fn acpi_gpiochip_free_interrupts(chip: *mut gpio_chip);
}
extern "C" {
    pub fn acpi_gpio_count(fwnode: *const fwnode_handle, con_id: *const c_char) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

extern "C" {
    pub fn acpi_gpio_process_deferred_list(list: *mut list_head);
}
extern "C" {
    pub fn acpi_gpio_add_to_deferred_list(list: *mut list_head) -> bool;
}
extern "C" {
    pub fn acpi_gpio_remove_from_deferred_list(list: *mut list_head);
}
extern "C" {
    pub fn acpi_gpio_need_run_edge_events_on_boot() -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_gpio_ignore_list {
    ACPI_GPIO_IGNORE_WAKE,
    ACPI_GPIO_IGNORE_INTERRUPT,
}
