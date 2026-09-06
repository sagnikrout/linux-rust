//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-smbus.h
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
// i2c-smbus.h - SMBus extensions to the I2C protocol
//
// Copyright (C) 2010-2019 Jean Delvare <jdelvare@suse.de>
//

//
// i2c_smbus_alert_setup - platform data for the smbus_alert i2c client
// @irq: IRQ number, if the smbus_alert driver should take care of interrupt
// handling
//
// If irq is not specified, the smbus_alert driver doesn't take care of
// interrupt handling. In that case it is up to the I2C bus driver to either
// handle the interrupts or to poll for alerts.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_smbus_alert_setup {
    pub irq: c_int,
}

extern "C" {
    pub fn i2c_handle_smbus_alert(ara: *mut i2c_client) -> c_int;
}

extern "C" {
    pub fn i2c_free_slave_host_notify_device(client: *mut i2c_client);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

extern "C" {
    pub fn i2c_register_spd_write_disable(adap: *mut i2c_adapter);
}
extern "C" {
    pub fn i2c_register_spd_write_enable(adap: *mut i2c_adapter);
}

