//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mux/driver.h
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
// mux/driver.h - definitions for the multiplexer driver interface
//
// Copyright (C) 2017 Axentia Technologies AB
//
// Author: Peter Rosin <peda@axentia.se>
//

//
// struct mux_control_ops -	Mux controller operations for a mux chip.
// @set:			Set the state of the given mux controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_control_ops {
    pub state): *mut *mut *mut int (set)(struct mux_control mux, int,
}

//
// struct mux_control -	Represents a mux controller.
// @lock:		Protects the mux controller state.
// @chip:		The mux chip that is handling this mux controller.
// @cached_state:	The current mux controller state, or -1 if none.
// @states:		The number of mux controller states.
// @idle_state:		The mux controller state to use when inactive, or one
// of MUX_IDLE_AS_IS and MUX_IDLE_DISCONNECT.
// @last_change:	Timestamp of last change
//
// Mux drivers may only change @states and @idle_state, and may only do so
// between allocation and registration of the mux controller. Specifically,
// @cached_state is internal to the mux core and should never be written by
// mux drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_control {
    pub /: *mut *mut semaphore lock; / protects the state of the mux,
    pub chip: *mut mux_chip,
    pub cached_state: c_int,
    pub states: c_uint,
    pub idle_state: c_int,
    pub last_change: ktime_t,
}

//
// struct mux_chip -	Represents a chip holding mux controllers.
// @controllers:	Number of mux controllers handled by the chip.
// @dev:		Device structure.
// @id:			Used to identify the device internally.
// @ops:		Mux controller operations.
// @mux:		Array of mux controllers that are handled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mux_chip {
    pub controllers: c_uint,
    pub dev: device,
    pub id: c_int,
    pub ops: *const mux_control_ops,
    pub __counted_by(controllers): mux_control mux[],
}

//
// mux_chip_priv() - Get the extra memory reserved by mux_chip_alloc().
// @mux_chip: The mux-chip to get the private memory from.
//
// Return: Pointer to the private memory reserved by the allocator.
//
extern "C" {
    pub fn mux_chip_register(mux_chip: *mut mux_chip) -> c_int;
}
extern "C" {
    pub fn mux_chip_unregister(mux_chip: *mut mux_chip);
}
extern "C" {
    pub fn mux_chip_free(mux_chip: *mut mux_chip);
}
extern "C" {
    pub fn devm_mux_chip_register(dev: *mut device, mux_chip: *mut mux_chip) -> c_int;
}
//
// mux_control_get_index() - Get the index of the given mux controller
// @mux: The mux-control to get the index for.
//
// Return: The index of the mux controller within the mux chip the mux
// controller is a part of.
//
