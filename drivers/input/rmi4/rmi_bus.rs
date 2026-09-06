//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/rmi4/rmi_bus.h
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
// Copyright (c) 2011-2016 Synaptics Incorporated
// Copyright (c) 2011 Unixphere
//

//
// The interrupt source count in the function descriptor can represent up to
// 6 interrupt sources in the normal manner.
//
pub const RMI_FN_MAX_IRQS: c_int = 6;
//
// struct rmi_function - represents the implementation of an RMI4
// function for a particular device (basically, a driver for that RMI4 function)
//
// @fd: The function descriptor of the RMI function
// @rmi_dev: Pointer to the RMI device associated with this function container
// @dev: The device associated with this particular function.
//
// @num_of_irqs: The number of irqs needed by this function
// @irq_pos: The position in the irq bitfield this function holds
// @irq_mask: For convenience, can be used to mask IRQ bits off during ATTN
// interrupt handling.
// @irqs: assigned virq numbers (up to num_of_irqs)
//
// @node: entry in device's list of functions
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_function {
    pub fd: rmi_function_descriptor,
    pub rmi_dev: *mut rmi_device,
    pub dev: device,
    pub node: list_head,
    pub num_of_irqs: c_uint,
    pub irq: [c_int; RMI_FN_MAX_IRQS],
    pub irq_pos: c_uint,
    pub irq_mask: [c_ulong; ],
}

extern "C" {
    pub fn rmi_is_function_device(dev: *mut device) -> bool;
}
extern "C" {
    pub fn rmi_register_function(: *mut rmi_function) -> int __must_check;
}
extern "C" {
    pub fn rmi_unregister_function(: *mut rmi_function);
}
//
// struct rmi_function_handler - driver routines for a particular RMI function.
//
// @func: The RMI function number
// @reset: Called when a reset of the touch sensor is detected.  The routine
// should perform any out-of-the-ordinary reset handling that might be
// necessary.  Restoring of touch sensor configuration registers should be
// handled in the config() callback, below.
// @config: Called when the function container is first initialized, and
// after a reset is detected.  This routine should write any necessary
// configuration settings to the device.
// @attention: Called when the IRQ(s) for the function are set by the touch
// sensor.
// @suspend: Should perform any required operations to suspend the particular
// function.
// @resume: Should perform any required operations to resume the particular
// function.
//
// All callbacks are expected to return 0 on success, error code on failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_function_handler {
    pub driver: device_driver,
    pub func: u8,
    pub fn): *mut *mut int (probe)(struct rmi_function,
    pub fn): *mut *mut void (remove)(struct rmi_function,
    pub fn): *mut *mut int (config)(struct rmi_function,
    pub fn): *mut *mut int (reset)(struct rmi_function,
    pub ctx): *mut *mut irqreturn_t (attention)(int irq, void,
    pub fn): *mut *mut int (suspend)(struct rmi_function,
    pub fn): *mut *mut int (resume)(struct rmi_function,
}

extern "C" {
    pub fn rmi_unregister_function_handler(: *mut rmi_function_handler);
}

extern "C" {
    pub fn rmi_is_physical_device(dev: *mut device) -> bool;
}
//
// rmi_reset - reset a RMI4 device
// @d: Pointer to an RMI device
//
// Calls for a reset of each function implemented by a specific device.
// Returns 0 on success or a negative error code.
//
// rmi_read - read a single byte
// @d: Pointer to an RMI device
// @addr: The address to read from
// @buf: The read buffer
//
// Reads a single byte of data using the underlying transport protocol
// into memory pointed by @buf. It returns 0 on success or a negative
// error code.
//
// rmi_read_block - read a block of bytes
// @d: Pointer to an RMI device
// @addr: The start address to read from
// @buf: The read buffer
// @len: Length of the read buffer
//
// Reads a block of byte data using the underlying transport protocol
// into memory pointed by @buf. It returns 0 on success or a negative
// error code.
//
// rmi_write - write a single byte
// @d: Pointer to an RMI device
// @addr: The address to write to
// @data: The data to write
//
// Writes a single byte using the underlying transport protocol. It
// returns zero on success or a negative error code.
//
// rmi_write_block - write a block of bytes
// @d: Pointer to an RMI device
// @addr: The start address to write to
// @buf: The write buffer
// @len: Length of the write buffer
//
// Writes a block of byte data from buf using the underlaying transport
// protocol.  It returns the amount of bytes written or a negative error code.
//
extern "C" {
    pub fn rmi_for_each_dev(data: *mut c_void, dev: *mut *mut int (func)(struct device, data): *mut c_void) -> c_int;
}

extern "C" {
    pub fn rmi_dbg(flags: c_int, dev: *mut device, fmt: *const c_char, ...);
}
