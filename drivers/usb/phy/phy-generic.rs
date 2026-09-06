//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/phy/phy-generic.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_phy_generic {
    pub phy: usb_phy,
    pub dev: *mut device,
    pub clk: *mut clk,
    pub vcc: *mut regulator,
    pub gpiod_reset: *mut gpio_desc,
    pub gpiod_vbus: *mut gpio_desc,
    pub vbus_draw: *mut regulator,
    pub vbus_draw_enabled: bool,
    pub mA: c_ulong,
    pub vbus: c_uint,
}

extern "C" {
    pub fn usb_gen_phy_init(phy: *mut usb_phy) -> c_int;
}
extern "C" {
    pub fn usb_gen_phy_shutdown(phy: *mut usb_phy);
}
extern "C" {
    pub fn usb_phy_gen_create_phy(dev: *mut device, nop: *mut usb_phy_generic) -> c_int;
}
