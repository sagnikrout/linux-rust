//! Automatically rewritten from C to Rust
//! Source: drivers/tty/serial/serial_ctrl.c
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
// Serial core controller driver
//
// Copyright (C) 2023 Texas Instruments Incorporated - https://www.ti.com
// Author: Tony Lindgren <tony@atomide.com>
//
// This driver manages the serial core controller struct device instances.
// The serial core controller devices are children of the physical serial
// port device.
//

#[no_mangle]
unsafe extern "C" fn serial_ctrl_probe(dev: *mut device) -> c_int {
    static int serial_ctrl_probe(struct device *dev)
    {
    pm_runtime_enable(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn serial_ctrl_remove(dev: *mut device) -> c_int {
    static int serial_ctrl_remove(struct device *dev)
    {
    pm_runtime_disable(dev);
    return 0;
    }
//
// Serial core controller device init functions. Note that the physical
// serial port device driver may not have completed probe at this point.
//
#[no_mangle]
pub unsafe extern "C" fn serial_ctrl_register_port(drv: *mut uart_driver, port: *mut uart_port) -> c_int {
    int serial_ctrl_register_port(struct uart_driver *drv, struct uart_port *port)
    {
    return serial_core_register_port(drv, port);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_ctrl_unregister_port(drv: *mut uart_driver, port: *mut uart_port) {
    void serial_ctrl_unregister_port(struct uart_driver *drv, struct uart_port *port)
    {
    serial_core_unregister_port(drv, port);
    }
    static struct device_driver serial_ctrl_driver = {
    .name = "ctrl",
    .suppress_bind_attrs = true,
    .probe = serial_ctrl_probe,
    .remove = serial_ctrl_remove,
    };
#[no_mangle]
pub unsafe extern "C" fn serial_base_ctrl_init() -> c_int {
    int serial_base_ctrl_init(void)
    {
    return serial_base_driver_register(&serial_ctrl_driver);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_base_ctrl_exit() {
    void serial_base_ctrl_exit(void)
    {
    serial_base_driver_unregister(&serial_ctrl_driver);
    }
    MODULE_AUTHOR("Tony Lindgren <tony@atomide.com>");
    MODULE_DESCRIPTION("Serial core controller driver");
    MODULE_LICENSE("GPL");
