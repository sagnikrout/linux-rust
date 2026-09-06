//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ks0108.h
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
// Filename: ks0108.h
// Version: 0.1.0
// Description: ks0108 LCD Controller driver header
//
// Author: Copyright (C) Miguel Ojeda <ojeda@kernel.org>
// Date: 2006-10-31
//
// Write a byte to the data port
extern "C" {
    pub fn ks0108_writedata(byte: c_uchar);
}
// Write a byte to the control port
extern "C" {
    pub fn ks0108_writecontrol(byte: c_uchar);
}
// Set the controller's current display state (0..1)
extern "C" {
    pub fn ks0108_displaystate(state: c_uchar);
}
// Set the controller's current startline (0..63)
extern "C" {
    pub fn ks0108_startline(startline: c_uchar);
}
// Set the controller's current address (0..63)
extern "C" {
    pub fn ks0108_address(address: c_uchar);
}
// Set the controller's current page (0..7)
extern "C" {
    pub fn ks0108_page(page: c_uchar);
}
// Is the module inited?
extern "C" {
    pub fn ks0108_isinited() -> c_uchar;
}
