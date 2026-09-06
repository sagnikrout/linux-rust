//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pmac_low_i2c.h
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
// include/asm-ppc/pmac_low_i2c.h
//
// Copyright (C) 2003 Ben. Herrenschmidt (benh@kernel.crashing.org)
//

// i2c mode (based on the platform functions format)
// RW bit in address
// i2c bus type
// i2c bus features
// can_largesub : supports >1 byte subaddresses (SMU only)
// multibus : device node holds multiple busses, bus number is
// encoded in bits 0xff00 of "reg" of a given device
//
// i2c busses in the system
// Init, called early during boot
extern "C" {
    pub fn pmac_i2c_init() -> c_int;
}
// Lookup an i2c bus for a device-node. The node can be either the bus
// node itself or a device below it. In the case of a multibus, the bus
// node itself is the controller node, else, it's a child of the controller
// node
//
// Get the address for an i2c device. This strips the bus number if
// necessary. The 7 bits address is returned 1 bit right shifted so that the
// direction can be directly ored in
//
extern "C" {
    pub fn pmac_i2c_get_dev_addr(device: *mut device_node) -> u8;
}
// Get infos about a bus
extern "C" {
    pub fn pmac_i2c_get_type(bus: *mut pmac_i2c_bus) -> c_int;
}
extern "C" {
    pub fn pmac_i2c_get_flags(bus: *mut pmac_i2c_bus) -> c_int;
}
extern "C" {
    pub fn pmac_i2c_get_channel(bus: *mut pmac_i2c_bus) -> c_int;
}
// i2c layer adapter helpers
// March a device or bus with an i2c adapter structure, to be used by drivers
// to match device-tree nodes with i2c adapters during adapter discovery
// callbacks
//
// Access functions for platform code
extern "C" {
    pub fn pmac_i2c_open(bus: *mut pmac_i2c_bus, polled: c_int) -> c_int;
}
extern "C" {
    pub fn pmac_i2c_close(bus: *mut pmac_i2c_bus);
}
extern "C" {
    pub fn pmac_i2c_setmode(bus: *mut pmac_i2c_bus, mode: c_int) -> c_int;
}
// Suspend/resume code called by via-pmu directly for now
extern "C" {
    pub fn pmac_pfunc_i2c_suspend();
}
extern "C" {
    pub fn pmac_pfunc_i2c_resume();
}

