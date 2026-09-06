//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/otg.h
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
// USB OTG (On The Go) defines
//
// These APIs may be used between USB controllers.  USB device drivers
// (for either host or peripheral roles) don't use these calls; they
// continue to use just usb_device and usb_gadget.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_otg {
    pub default_a: u8,
    pub phy: *mut phy,
// old usb_phy interface
    pub usb_phy: *mut usb_phy,
    pub host: *mut usb_bus,
    pub gadget: *mut usb_gadget,
    pub state: usb_otg_state,
// bind/unbind the host controller
    pub host): *mut *mut *mut int (set_host)(struct usb_otg otg, struct usb_bus,
// bind/unbind the peripheral controller
    pub gadget): *mut usb_gadget,
// effective for A-peripheral, ignored for B devices
    pub enabled): *mut *mut *mut int (set_vbus)(struct usb_otg otg, bool,
// for B devices only:  start session with A-Host
    pub otg): *mut *mut int (start_srp)(struct usb_otg,
// start or continue HNP role switch
    pub otg): *mut *mut int (start_hnp)(struct usb_otg,
}

//
// struct usb_otg_caps - describes the otg capabilities of the device
// @otg_rev: The OTG revision number the device is compliant with, it's
// in binary-coded decimal (i.e. 2.0 is 0200H).
// @hnp_support: Indicates if the device supports HNP.
// @srp_support: Indicates if the device supports SRP.
// @adp_support: Indicates if the device supports ADP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_otg_caps {
    pub otg_rev: u16,
    pub hnp_support: bool,
    pub srp_support: bool,
    pub adp_support: bool,
}

// Context: can sleep
// for HCDs
// for usb peripheral controller drivers
// Context: can sleep
// for OTG controller drivers (and maybe other stuff)
extern "C" {
    pub fn usb_bus_start_enum(bus: *mut usb_bus, port_num: unsigned) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_dr_mode {
    USB_DR_MODE_UNKNOWN,
    USB_DR_MODE_HOST,
    USB_DR_MODE_PERIPHERAL,
    USB_DR_MODE_OTG,
}

//
// usb_get_dr_mode - Get dual role mode for given device
// @dev: Pointer to the given device
//
// The function gets phy interface string from property 'dr_mode',
// and returns the corresponding enum usb_dr_mode
//
extern "C" {
    pub fn usb_get_dr_mode(dev: *mut device) -> usb_dr_mode;
}
extern "C" {
    pub fn usb_get_role_switch_default_mode(dev: *mut device) -> usb_dr_mode;
}
