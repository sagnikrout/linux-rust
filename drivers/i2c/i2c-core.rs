//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i2c/i2c-core.h
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
// i2c-core.h - interfaces internal to the I2C framework
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_devinfo {
    pub list: list_head,
    pub busnum: c_int,
    pub board_info: i2c_board_info,
}

// board_lock protects board_list and first_dynamic_bus_num.
// only i2c core components are allowed to use these symbols.
//
extern "C" {
    pub fn i2c_check_7bit_addr_validity_strict(addr: c_ushort) -> c_int;
}
//
// We only allow atomic transfers for very late communication, e.g. to access a
// PMIC when powering down. Atomic transfers are a corner case and not for
// generic use!
//

extern "C" {
    pub fn i2c_acpi_register_devices(adap: *mut i2c_adapter);
}
extern "C" {
    pub fn i2c_acpi_get_irq(client: *mut i2c_client, wake_capable: *mut bool) -> c_int;
}

extern "C" {
    pub fn i2c_acpi_install_space_handler(adapter: *mut i2c_adapter) -> c_int;
}
extern "C" {
    pub fn i2c_acpi_remove_space_handler(adapter: *mut i2c_adapter);
}

extern "C" {
    pub fn of_i2c_register_devices(adap: *mut i2c_adapter);
}

extern "C" {
    pub fn i2c_setup_smbus_alert(adap: *mut i2c_adapter) -> c_int;
}

