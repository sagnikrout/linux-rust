//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/internals.h
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
// Copyright (C) 2018 Cadence Design Systems Inc.
//
// Author: Boris Brezillon <boris.brezillon@bootlin.com>
//

extern "C" {
    pub fn i3c_bus_rpm_get(bus: *mut i3c_bus) -> int __must_check;
}
extern "C" {
    pub fn i3c_bus_rpm_put(bus: *mut i3c_bus);
}
extern "C" {
    pub fn i3c_bus_rpm_ibi_allowed(bus: *mut i3c_bus) -> bool;
}
extern "C" {
    pub fn i3c_bus_normaluse_lock(bus: *mut i3c_bus);
}
extern "C" {
    pub fn i3c_bus_normaluse_unlock(bus: *mut i3c_bus);
}
extern "C" {
    pub fn i3c_dev_setdasa_locked(dev: *mut i3c_dev_desc) -> c_int;
}
extern "C" {
    pub fn i3c_dev_disable_ibi_locked(dev: *mut i3c_dev_desc) -> c_int;
}
extern "C" {
    pub fn i3c_dev_enable_ibi_locked(dev: *mut i3c_dev_desc) -> c_int;
}
extern "C" {
    pub fn i3c_dev_free_ibi_locked(dev: *mut i3c_dev_desc);
}
//
// i3c_writel_fifo - Write data buffer to 32bit FIFO
// @addr: FIFO Address to write to
// @buf: Pointer to the data bytes to write
// @nbytes: Number of bytes to write
//
// writesl() instead of writel() to keep FIFO
// byteorder on big-endian targets
//
// i3c_readl_fifo - Read data buffer from 32bit FIFO
// @addr: FIFO Address to read from
// @buf: Pointer to the buffer to store read bytes
// @nbytes: Number of bytes to read
//
// readsl() instead of readl() to keep FIFO
// byteorder on big-endian targets
//
extern "C" {
    pub fn container_of(_arg: i3cbus, i3c_master_controller: struct, _arg: bus) -> return;
}
