//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/serial_base.h
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
// Serial core related functions, serial port device drivers do not need this.
//
// Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
// Author: Tony Lindgren <tony@atomide.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_ctrl_device {
    pub dev: device,
    pub port_ida: ida,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_port_device {
    pub dev: device,
    pub port: *mut uart_port,
    pub tx_enabled:1: c_uint,
}

extern "C" {
    pub fn serial_base_ctrl_init() -> c_int;
}
extern "C" {
    pub fn serial_base_ctrl_exit();
}
extern "C" {
    pub fn serial_base_port_init() -> c_int;
}
extern "C" {
    pub fn serial_base_port_exit();
}
extern "C" {
    pub fn serial_base_port_startup(port: *mut uart_port);
}
extern "C" {
    pub fn serial_base_port_shutdown(port: *mut uart_port);
}
extern "C" {
    pub fn serial_base_driver_register(driver: *mut device_driver) -> c_int;
}
extern "C" {
    pub fn serial_base_driver_unregister(driver: *mut device_driver);
}
extern "C" {
    pub fn serial_base_ctrl_device_remove(ctrl_dev: *mut serial_ctrl_device);
}
extern "C" {
    pub fn serial_base_port_device_remove(port_dev: *mut serial_port_device);
}
extern "C" {
    pub fn serial_ctrl_register_port(drv: *mut uart_driver, port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn serial_ctrl_unregister_port(drv: *mut uart_driver, port: *mut uart_port);
}
extern "C" {
    pub fn serial_core_register_port(drv: *mut uart_driver, port: *mut uart_port) -> c_int;
}
extern "C" {
    pub fn serial_core_unregister_port(drv: *mut uart_driver, port: *mut uart_port);
}

