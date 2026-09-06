//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/apple-gmux.h
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
// apple-gmux.h - microcontroller built into dual GPU MacBook Pro & Mac Pro
// Copyright (C) 2015 Lukas Wunner <lukas@wunner.de>
//

//
// gmux port offsets. Many of these are not yet used, but may be in the
// future, and it's useful to have them documented here anyhow.
//
pub const GMUX_PORT_VERSION_MAJOR: c_uint = 0x04;
pub const GMUX_PORT_VERSION_MINOR: c_uint = 0x05;
pub const GMUX_PORT_VERSION_RELEASE: c_uint = 0x06;
pub const GMUX_PORT_SWITCH_DISPLAY: c_uint = 0x10;
pub const GMUX_PORT_SWITCH_GET_DISPLAY: c_uint = 0x11;
pub const GMUX_PORT_INTERRUPT_ENABLE: c_uint = 0x14;
pub const GMUX_PORT_INTERRUPT_STATUS: c_uint = 0x16;
pub const GMUX_PORT_SWITCH_DDC: c_uint = 0x28;
pub const GMUX_PORT_SWITCH_EXTERNAL: c_uint = 0x40;
pub const GMUX_PORT_SWITCH_GET_EXTERNAL: c_uint = 0x41;
pub const GMUX_PORT_DISCRETE_POWER: c_uint = 0x50;
pub const GMUX_PORT_MAX_BRIGHTNESS: c_uint = 0x70;
pub const GMUX_PORT_BRIGHTNESS: c_uint = 0x74;
pub const GMUX_PORT_VALUE: c_uint = 0xc2;
pub const GMUX_PORT_READ: c_uint = 0xd0;
pub const GMUX_PORT_WRITE: c_uint = 0xd4;
pub const GMUX_MMIO_PORT_SELECT: c_uint = 0x0e;
pub const GMUX_MMIO_COMMAND_SEND: c_uint = 0x0f;
pub const GMUX_MMIO_READ: c_uint = 0x00;
pub const GMUX_MMIO_WRITE: c_uint = 0x40;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum apple_gmux_type {
    APPLE_GMUX_TYPE_PIO,
    APPLE_GMUX_TYPE_INDEXED,
    APPLE_GMUX_TYPE_MMIO,
}

//
// If this is 0xff, then gmux must not be present, as the gmux would
// reset it to 0x00, or it would be one of 0x1, 0x4, 0x41, 0x44 if a
// command is currently being processed.
//
// apple_gmux_detect() - detect if gmux is built into the machine
//
// @pnp_dev:     Device to probe or NULL to use the first matching device
// @type_ret: Returns (by reference) the apple_gmux_type of the device
//
// Detect if a supported gmux device is present by actually probing it.
// This avoids the false positives returned on some models by
// apple_gmux_present().
//
// Return: %true if a supported gmux ACPI device is detected and the kernel
// was configured with CONFIG_APPLE_GMUX, %false otherwise.
//
// Invalid version information may indicate either that the gmux
// device isn't present or that it's a new one that uses indexed io.
//
// type_ret = type;
//
// apple_gmux_present() - check if gmux ACPI device is present
//
// Drivers may use this to activate quirks specific to dual GPU MacBook Pros
// and Mac Pros, e.g. for deferred probing, runtime pm and backlight.
//
// Return: %true if gmux ACPI device is present and the kernel was configured
// with CONFIG_APPLE_GMUX, %false otherwise.
//
extern "C" {
    pub fn acpi_dev_found(_arg: GMUX_ACPI_HID) -> return;
}

