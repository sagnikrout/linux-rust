//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serdev.h
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
// Copyright (C) 2016-2017 Linaro Ltd., Rob Herring <robh@kernel.org>
//

//
// serdev device structures
//
// struct serdev_device_ops - Callback operations for a serdev device
// @receive_buf:	Function called with data received from device;
// returns number of bytes accepted; may sleep.
// @write_wakeup:	Function called when ready to transmit more data; must
// not sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdev_device_ops {
    pub size_t): *const *const *const *const size_t (receive_buf)(struct serdev_device , u8 ,,
    pub ): *mut *mut void (write_wakeup)(struct serdev_device,
}

//
// struct serdev_device - Basic representation of an serdev device
// @dev:	Driver model representation of the device.
// @nr:		Device number on serdev bus.
// @ctrl:	serdev controller managing this device.
// @ops:	Device operations.
// @write_comp:	Completion used by serdev_device_write() internally
// @write_lock:	Lock to serialize access when writing data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdev_device {
    pub dev: device,
    pub nr: c_int,
    pub ctrl: *mut serdev_controller,
    pub ops: *const serdev_device_ops,
    pub write_comp: completion,
    pub write_lock: mutex,
}

//
// struct serdev_device_driver - serdev slave device driver
// @driver:	serdev device drivers should initialize name field of this
// structure.
// @probe:	binds this driver to a serdev device.
// @remove:	unbinds this driver from the serdev device.
// @shutdown:	shut down this serdev device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdev_device_driver {
    pub driver: device_driver,
    pub ): *mut *mut int (probe)(struct serdev_device,
    pub ): *mut *mut void (remove)(struct serdev_device,
    pub ): *mut *mut void (shutdown)(struct serdev_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum serdev_parity {
    SERDEV_PARITY_NONE,
    SERDEV_PARITY_EVEN,
    SERDEV_PARITY_ODD,
}

//
// serdev controller structures
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdev_controller_ops {
    pub size_t): *const *const *const *const ssize_t (write_buf)(struct serdev_controller , u8 ,,
    pub ): *mut *mut void (write_flush)(struct serdev_controller,
    pub ): *mut *mut int (open)(struct serdev_controller,
    pub ): *mut *mut void (close)(struct serdev_controller,
    pub bool): *mut *mut *mut void (set_flow_control)(struct serdev_controller ,,
    pub serdev_parity): *mut *mut *mut int (set_parity)(struct serdev_controller , enum,
    pub int): *mut *mut *mut unsigned int (set_baudrate)(struct serdev_controller , unsigned,
    pub long): *mut *mut *mut void (wait_until_sent)(struct serdev_controller ,,
    pub ): *mut *mut int (get_tiocm)(struct serdev_controller,
    pub int): *mut *mut *mut int (set_tiocm)(struct serdev_controller , unsigned int, unsigned,
    pub break_state): *mut *mut *mut int (break_ctl)(struct serdev_controller ctrl, unsigned int,
}

//
// struct serdev_controller - interface to the serdev controller
// @dev:	Driver model representation of the device.
// @host:	Serial port hardware controller device
// @nr:		number identifier for this controller/bus.
// @serdev:	Pointer to slave device for this controller.
// @ops:	Controller operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serdev_controller {
    pub dev: device,
    pub host: *mut device,
    pub nr: c_uint,
    pub serdev: *mut serdev_device,
    pub ops: *const serdev_controller_ops,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &serdev->dev) -> return;
}
//
// serdev_device_put() - decrement serdev device refcount
// @serdev:	serdev device.
//
// serdev_controller_put() - decrement controller refcount
// @ctrl:	serdev controller.
//
extern "C" {
    pub fn serdev_device_add(: *mut serdev_device) -> c_int;
}
extern "C" {
    pub fn serdev_device_remove(: *mut serdev_device);
}
extern "C" {
    pub fn serdev_controller_add(: *mut serdev_controller) -> c_int;
}
extern "C" {
    pub fn serdev_controller_remove(: *mut serdev_controller);
}

extern "C" {
    pub fn serdev_device_open(: *mut serdev_device) -> c_int;
}
extern "C" {
    pub fn serdev_device_close(: *mut serdev_device);
}
extern "C" {
    pub fn devm_serdev_device_open(: *mut device, : *mut serdev_device) -> c_int;
}
extern "C" {
    pub fn serdev_device_set_baudrate(: *mut serdev_device, int: unsigned) -> c_uint;
}
extern "C" {
    pub fn serdev_device_set_flow_control(: *mut serdev_device, _arg: bool);
}
extern "C" {
    pub fn serdev_device_write_buf(: *mut serdev_device, : *const u8, _arg: usize) -> c_int;
}
extern "C" {
    pub fn serdev_device_wait_until_sent(: *mut serdev_device, _arg: c_long);
}
extern "C" {
    pub fn serdev_device_get_tiocm(: *mut serdev_device) -> c_int;
}
extern "C" {
    pub fn serdev_device_set_tiocm(: *mut serdev_device, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn serdev_device_break_ctl(serdev: *mut serdev_device, break_state: c_int) -> c_int;
}
extern "C" {
    pub fn serdev_device_write_wakeup(: *mut serdev_device);
}
extern "C" {
    pub fn serdev_device_write(: *mut serdev_device, : *const u8, _arg: usize, _arg: c_long) -> isize;
}
extern "C" {
    pub fn serdev_device_write_flush(: *mut serdev_device);
}
//
// serdev device driver functions
//
extern "C" {
    pub fn __serdev_device_driver_register(: *mut serdev_device_driver, : *mut module) -> c_int;
}

//
// serdev_device_driver_unregister() - unregister an serdev client driver
// @sdrv:	the driver to unregister
//

// Macro flag: #define serdev_device_driver_register(x)
// Macro flag: #define serdev_device_driver_unregister(x)

extern "C" {
    pub fn serdev_device_set_tiocm(_arg: serdev, _arg: TIOCM_RTS, _arg: 0) -> return;
}
extern "C" {
    pub fn serdev_device_set_tiocm(_arg: serdev, _arg: 0, _arg: TIOCM_RTS) -> return;
}
//
// serdev hooks into TTY core
//

extern "C" {
    pub fn serdev_tty_port_unregister(port: *mut tty_port) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

