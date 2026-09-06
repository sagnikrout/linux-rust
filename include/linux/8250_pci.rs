//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/8250_pci.h
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
// Definitions for PCI support.
//
pub const FL_BASE_MASK: c_uint = 0x0007;
pub const FL_BASE0: c_uint = 0x0000;
pub const FL_BASE1: c_uint = 0x0001;
pub const FL_BASE2: c_uint = 0x0002;
pub const FL_BASE3: c_uint = 0x0003;
pub const FL_BASE4: c_uint = 0x0004;

// Use successive BARs (PCI base address registers),
pub const FL_BASE_BARS: c_uint = 0x0008;
// do not assign an irq
pub const FL_NOIRQ: c_uint = 0x0080;
// Use the Base address register size to cap number of ports
pub const FL_REGION_SZ_CAP: c_uint = 0x0100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciserial_board {
    pub flags: c_uint,
    pub num_ports: c_uint,
    pub base_baud: c_uint,
    pub uart_offset: c_uint,
    pub reg_shift: c_uint,
    pub first_offset: c_uint,
}

extern "C" {
    pub fn pciserial_remove_ports(priv: *mut serial_private);
}
extern "C" {
    pub fn pciserial_suspend_ports(priv: *mut serial_private);
}
extern "C" {
    pub fn pciserial_resume_ports(priv: *mut serial_private);
}
