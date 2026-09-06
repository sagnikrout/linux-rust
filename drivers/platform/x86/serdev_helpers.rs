//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/serdev_helpers.h
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
// In some cases UART attached devices which require an in kernel driver,
// e.g. UART attached Bluetooth HCIs are described in the ACPI tables
// by an ACPI device with a broken or missing UartSerialBusV2() resource.
//
// This causes the kernel to create a /dev/ttyS# char-device for the UART
// instead of creating an in kernel serdev-controller + serdev-device pair
// for the in kernel driver.
//
// The quirk handling in acpi_quirk_skip_serdev_enumeration() makes the kernel
// create a serdev-controller device for these UARTs instead of a /dev/ttyS#.
//
// Instantiating the actual serdev-device to bind to is up to pdx86 code,
// this header provides a helper for getting the serdev-controller device.
//

// Walk host -> uart-ctrl -> port -> serdev-ctrl
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
// get_first_physical_node() returns a weak ref
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
// This puts our reference on parent and returns a ref on the ctrl
extern "C" {
    pub fn get_serdev_controller_from_parent(_arg: parent, _arg: serial_ctrl_port, _arg: serdev_ctrl_name) -> return;
}
