//! Automatically rewritten from C to Rust
//! Source: drivers/usb/core/notify.c
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
// All the USB notify logic
//
// (C) Copyright 2005 Greg Kroah-Hartman <gregkh@suse.de>
//
// notifier functions originally based on those in kernel/sys.c
// but fixed up to not be so broken.
//
// Released under the GPLv2 only.
//

    static BLOCKING_NOTIFIER_HEAD(usb_notifier_list);
//
// usb_register_notify - register a notifier callback whenever a usb change happens
// @nb: pointer to the notifier block for the callback events.
//
// These changes are either USB devices or busses being added or removed.
//
#[no_mangle]
pub unsafe extern "C" fn usb_register_notify(nb: *mut notifier_block) {
    void usb_register_notify(struct notifier_block *nb)
    {
    blocking_notifier_chain_register(&usb_notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(usb_register_notify);
//
// usb_unregister_notify - unregister a notifier callback
// @nb: pointer to the notifier block for the callback events.
//
// usb_register_notify() must have been previously called for this function
// to work properly.
//
#[no_mangle]
pub unsafe extern "C" fn usb_unregister_notify(nb: *mut notifier_block) {
    void usb_unregister_notify(struct notifier_block *nb)
    {
    blocking_notifier_chain_unregister(&usb_notifier_list, nb);
    }
    EXPORT_SYMBOL_GPL(usb_unregister_notify);
#[no_mangle]
pub unsafe extern "C" fn usb_notify_add_device(udev: *mut usb_device) {
    void usb_notify_add_device(struct usb_device *udev)
    {
    blocking_notifier_call_chain(&usb_notifier_list, USB_DEVICE_ADD, udev);
    }
#[no_mangle]
pub unsafe extern "C" fn usb_notify_remove_device(udev: *mut usb_device) {
    void usb_notify_remove_device(struct usb_device *udev)
    {
    blocking_notifier_call_chain(&usb_notifier_list,
    USB_DEVICE_REMOVE, udev);
    }
#[no_mangle]
pub unsafe extern "C" fn usb_notify_add_bus(ubus: *mut usb_bus) {
    void usb_notify_add_bus(struct usb_bus *ubus)
    {
    blocking_notifier_call_chain(&usb_notifier_list, USB_BUS_ADD, ubus);
    }
#[no_mangle]
pub unsafe extern "C" fn usb_notify_remove_bus(ubus: *mut usb_bus) {
    void usb_notify_remove_bus(struct usb_bus *ubus)
    {
    blocking_notifier_call_chain(&usb_notifier_list, USB_BUS_REMOVE, ubus);
    }
