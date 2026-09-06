//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/dpll/zl3073x/prop.h
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
// struct zl3073x_pin_props - pin properties
// @fwnode: pin firmware node
// @dpll_props: DPLL core pin properties
// @package_label: pin package label
// @esync_control: embedded sync support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zl3073x_pin_props {
    pub fwnode: *mut fwnode_handle,
    pub dpll_props: dpll_pin_properties,
    pub package_label: [c_char; 8],
    pub esync_control: bool,
}

extern "C" {
    pub fn zl3073x_prop_dpll_type_get(zldev: *mut zl3073x_dev, index: u8) -> dpll_type;
}
extern "C" {
    pub fn zl3073x_pin_props_put(props: *mut zl3073x_pin_props);
}
