//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/surface_acpi_notify.h
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
// Interface for Surface ACPI Notify (SAN) driver.
//
// Provides access to discrete GPU notifications sent from ACPI via the SAN
// driver, which are not handled by this driver directly.
//
// Copyright (C) 2019-2020 Maximilian Luz <luzmaximilian@gmail.com>
//

//
// struct san_dgpu_event - Discrete GPU ACPI event.
// @category: Category of the event.
// @target:   Target ID of the event source.
// @command:  Command ID of the event.
// @instance: Instance ID of the event source.
// @length:   Length of the event's payload data (in bytes).
// @payload:  Pointer to the event's payload data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct san_dgpu_event {
    pub category: u8,
    pub target: u8,
    pub command: u8,
    pub instance: u8,
    pub length: u16,
    pub payload: *mut u8,
}

extern "C" {
    pub fn san_client_link(client: *mut device) -> c_int;
}
extern "C" {
    pub fn san_dgpu_notifier_register(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn san_dgpu_notifier_unregister(nb: *mut notifier_block) -> c_int;
}
