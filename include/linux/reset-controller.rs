//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/reset-controller.h
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
// struct reset_control_ops - reset controller driver callbacks
//
// @reset: for self-deasserting resets, does all necessary
// things to reset the device
// @assert: manually assert the reset line, if supported
// @deassert: manually deassert the reset line, if supported
// @status: return the status of the reset line, if supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_control_ops {
    pub id): *mut *mut *mut int (reset)(struct reset_controller_dev rcdev, unsigned long,
    pub id): *mut *mut *mut int (assert)(struct reset_controller_dev rcdev, unsigned long,
    pub id): *mut *mut *mut int (deassert)(struct reset_controller_dev rcdev, unsigned long,
    pub id): *mut *mut *mut int (status)(struct reset_controller_dev rcdev, unsigned long,
}

//
// struct reset_controller_dev - reset controller entity that might
// provide multiple reset controls
// @ops: a pointer to device specific struct reset_control_ops
// @owner: kernel module of the reset controller driver
// @list: internal list of reset controller devices
// @reset_control_head: head of internal list of requested reset controls
// @dev: corresponding driver model device struct
// @of_node: corresponding device tree node as phandle target
// @of_reset_n_cells: number of cells in reset line specifiers
// @of_xlate: translation function to translate from specifier as found in the
// device tree to id as given to the reset control ops
// @fwnode: firmware node associated with this device
// @fwnode_reset_n_cells: number of cells in reset line specifiers
// @fwnode_xlate: translation function to translate from firmware specifier to
// id as given to the reset control ops, defaults to
// :c:func:`fwnode_reset_simple_xlate`
// @nr_resets: number of reset controls in this reset controller device
// @lock: protects the reset control list from concurrent access
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_controller_dev {
    pub ops: *const reset_control_ops,
    pub owner: *mut module,
    pub list: list_head,
    pub reset_control_head: list_head,
    pub dev: *mut device,
    pub of_node: *mut device_node,
    pub of_reset_n_cells: c_int,
    pub reset_spec): *const of_phandle_args,
    pub fwnode: *mut fwnode_handle,
    pub fwnode_reset_n_cells: c_int,
    pub reset_spec): *const fwnode_reference_args,
    pub nr_resets: c_uint,
    pub lock: mutex,
}

extern "C" {
    pub fn reset_controller_register(rcdev: *mut reset_controller_dev) -> c_int;
}
extern "C" {
    pub fn reset_controller_unregister(rcdev: *mut reset_controller_dev);
}

