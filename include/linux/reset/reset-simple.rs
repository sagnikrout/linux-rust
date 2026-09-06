//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/reset/reset-simple.h
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
// Simple Reset Controller ops
//
// Based on Allwinner SoCs Reset Controller driver
//
// Copyright 2013 Maxime Ripard
//
// Maxime Ripard <maxime.ripard@free-electrons.com>
//

//
// struct reset_simple_data - driver data for simple reset controllers
// @lock: spinlock to protect registers during read-modify-write cycles
// @membase: memory mapped I/O register range
// @rcdev: reset controller device base structure
// @active_low: if true, bits are cleared to assert the reset. Otherwise, bits
// are set to assert the reset. Note that this says nothing about
// the voltage level of the actual reset line.
// @status_active_low: if true, bits read back as cleared while the reset is
// asserted. Otherwise, bits read back as set while the
// reset is asserted.
// @reset_us: Minimum delay in microseconds needed that needs to be
// waited for between an assert and a deassert to reset the
// device. If multiple consumers with different delay
// requirements are connected to this controller, it must
// be the largest minimum delay. 0 means that such a delay is
// unknown and the reset operation is unsupported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_simple_data {
    pub lock: spinlock_t,
    pub membase: *mut void __iomem,
    pub rcdev: reset_controller_dev,
    pub active_low: bool,
    pub status_active_low: bool,
    pub reset_us: c_uint,
}
