//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/cec-notifier.h
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
// cec-notifier.h - notify CEC drivers of physical address changes
//
// Copyright 2016 Russell King.
// Copyright 2016-2017 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

//
// cec_notifier_conn_register - find or create a new cec_notifier for the given
// HDMI device and connector tuple.
// @hdmi_dev: HDMI device that sends the events.
// @port_name: the connector name from which the event occurs. May be NULL
// if there is always only one HDMI connector created by the HDMI device.
// @conn_info: the connector info from which the event occurs (may be NULL)
//
// If a notifier for device @dev and connector @port_name already exists, then
// increase the refcount and return that notifier.
//
// If it doesn't exist, then allocate a new notifier struct and return a
// pointer to that new struct.
//
// Return NULL if the memory could not be allocated.
//
// cec_notifier_conn_unregister - decrease refcount and delete when the
// refcount reaches 0.
// @n: notifier. If NULL, then this function does nothing.
//
extern "C" {
    pub fn cec_notifier_conn_unregister(n: *mut cec_notifier);
}
//
// cec_notifier_cec_adap_register - find or create a new cec_notifier for the
// given device.
// @hdmi_dev: HDMI device that sends the events.
// @port_name: the connector name from which the event occurs. May be NULL
// if there is always only one HDMI connector created by the HDMI device.
// @adap: the cec adapter that registered this notifier.
//
// If a notifier for device @dev and connector @port_name already exists, then
// increase the refcount and return that notifier.
//
// If it doesn't exist, then allocate a new notifier struct and return a
// pointer to that new struct.
//
// Return NULL if the memory could not be allocated.
//
// cec_notifier_cec_adap_unregister - decrease refcount and delete when the
// refcount reaches 0.
// @n: notifier. If NULL, then this function does nothing.
// @adap: the cec adapter that registered this notifier.
//
// cec_notifier_set_phys_addr - set a new physical address.
// @n: the CEC notifier
// @pa: the CEC physical address
//
// Set a new CEC physical address.
// Does nothing if @n == NULL.
//
extern "C" {
    pub fn cec_notifier_set_phys_addr(n: *mut cec_notifier, pa: u16);
}
//
// cec_notifier_set_phys_addr_from_edid - set parse the PA from the EDID.
// @n: the CEC notifier
// @edid: the struct edid pointer
//
// Parses the EDID to obtain the new CEC physical address and set it.
// Does nothing if @n == NULL.
//
// cec_notifier_parse_hdmi_phandle - find the hdmi device from "hdmi-phandle"
// @dev: the device with the "hdmi-phandle" device tree property
//
// Returns the device pointer referenced by the "hdmi-phandle" property.
// Note that the refcount of the returned device is not incremented.
// This device pointer is only used as a key value in the notifier
// list, but it is never accessed by the CEC driver.
//

// A non-NULL pointer is expected on success
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// cec_notifier_phys_addr_invalidate() - set the physical address to INVALID
//
// @n: the CEC notifier
//
// This is a simple helper function to invalidate the physical
// address. Does nothing if @n == NULL.
//
