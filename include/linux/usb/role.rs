//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/role.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_role {
    USB_ROLE_NONE,
    USB_ROLE_HOST,
    USB_ROLE_DEVICE,
}

extern "C" {
    pub fn usb_role(sw: *mut *mut usb_role_switch_get_t)(struct usb_role_switch) -> typedef enum;
}
//
// struct usb_role_switch_desc - USB Role Switch Descriptor
// @fwnode: The device node to be associated with the role switch
// @usb2_port: Optional reference to the host controller port device (USB2)
// @usb3_port: Optional reference to the host controller port device (USB3)
// @udc: Optional reference to the peripheral controller device
// @set: Callback for setting the role
// @get: Callback for getting the role (optional)
// @allow_userspace_control: If true userspace may change the role through sysfs
// @driver_data: Private data pointer
// @name: Name for the switch (optional)
//
// @usb2_port and @usb3_port will point to the USB host port and @udc to the USB
// device controller behind the USB connector with the role switch. If
// @usb2_port, @usb3_port and @udc are included in the description, the
// reference count for them should be incremented by the caller of
// usb_role_switch_register() before registering the switch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_role_switch_desc {
    pub fwnode: *mut fwnode_handle,
    pub usb2_port: *mut device,
    pub usb3_port: *mut device,
    pub udc: *mut device,
    pub set: usb_role_switch_set_t,
    pub get: usb_role_switch_get_t,
    pub allow_userspace_control: bool,
    pub driver_data: *mut c_void,
    pub name: *const c_char,
}

extern "C" {
    pub fn usb_role_switch_set_role(sw: *mut usb_role_switch, role: usb_role) -> c_int;
}
extern "C" {
    pub fn usb_role_switch_get_role(sw: *mut usb_role_switch) -> usb_role;
}
extern "C" {
    pub fn usb_role_switch_put(sw: *mut usb_role_switch);
}
extern "C" {
    pub fn usb_role_switch_unregister(sw: *mut usb_role_switch);
}
extern "C" {
    pub fn usb_role_switch_set_drvdata(sw: *mut usb_role_switch, data: *mut c_void);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

