//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/dsa.h
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
pub const DSA_MAX_PORTS: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_chip_data {
//
// Reference to network devices
//
    pub netdev: [*mut device; DSA_MAX_PORTS],
// set to size of eeprom if supported by the switch
    pub eeprom_len: c_int,
//
// The names of the switch's ports.  Use "cpu" to
// designate the switch port that the cpu is connected to,
// "dsa" to indicate that this port is a DSA link to
// another switch, NULL to indicate the port is unused,
// or any other string to indicate this is a physical port.
//
    pub port_names: [*mut c_char; DSA_MAX_PORTS],
}
