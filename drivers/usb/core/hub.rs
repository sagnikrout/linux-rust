//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/core/hub.h
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
// usb hub driver head file
//
// Copyright (C) 1999 Linus Torvalds
// Copyright (C) 1999 Johannes Erdfelt
// Copyright (C) 1999 Gregory P. Smith
// Copyright (C) 2001 Brad Hards (bhards@bigpond.net.au)
// Copyright (C) 2012 Intel Corp (tianyu.lan@intel.com)
//
// move struct usb_hub to this file.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_hub {
    pub /: *mut *mut *mut device intfdev; / the "interface" device,
    pub hdev: *mut usb_device,
    pub kref: kref,
    pub /: *mut *mut *mut urb urb; / for interrupt polling pipe,
// buffer for urb ... with extra space in case of babble
    pub (*buffer)[8]: *mut u8,
    pub hub: usb_hub_status,
    pub port: usb_port_status,
    pub /: *mut *mut *mut } status; / buffer for status reports,
    pub /: *mut *mut mutex status_mutex; / for the status buffer,
    pub /: *mut *mut int error; / last reported error,
    pub /: *mut *mut int nerrors; / track consecutive errors,
    pub /: *mut *mut unsigned long event_bits[1]; / status change bitmask,
    pub connect: *mut *mut unsigned long change_bits[1]; / ports with logical,
    pub "removed": *mut *mut unsigned long removed_bits[1]; / ports with a,
    pub signaled: *mut *mut unsigned long wakeup_bits[1]; / ports that have,
    pub /: *mut *mut unsigned long power_bits[1]; / ports that are powered,
    pub for: *mut *mut unsigned long child_usage_bits[1]; / ports powered on,
    pub warm: *mut *mut unsigned long warm_reset_bits[1]; / ports requesting,

    pub /: *mut *mut *mut usb_hub_descriptor descriptor; / class descriptor,
    pub /: *mut *mut usb_tt tt; / Transaction Translator,
    pub /: *mut *mut unsigned mA_per_port; / current for each child,

    pub wakeup_enabled_descendants: unsigned,

    pub limited_power:1: unsigned,
    pub quiescing:1: unsigned,
    pub disconnected:1: unsigned,
    pub in_reset:1: unsigned,
    pub quirk_disable_autosuspend:1: unsigned,
    pub quirk_check_port_auto_suspend:1: unsigned,
    pub has_indicators:1: unsigned,
    pub indicator: [u8; USB_MAXCHILDREN],
    pub leds: delayed_work,
    pub init_work: delayed_work,
    pub post_resume_work: delayed_work,
    pub events: work_struct,
    pub irq_urb_lock: spinlock_t,
    pub irq_urb_retry: timer_list,
    pub ports: *mut usb_port,
    pub onboard_devs: list_head,
}

//
// struct usb port - kernel's representation of a usb port
// @child: usb device attached to the port
// @dev: generic device interface
// @port_owner: port's owner
// @peer: related usb2 and usb3 ports (share the same connector)
// @connector: USB Type-C connector
// @req: default pm qos request for hubs without port power control
// @connect_type: port's connect type
// @state: device state of the usb device attached to the port
// @state_kn: kernfs_node of the sysfs attribute that accesses @state
// @location: opaque representation of platform connector location
// @status_lock: synchronize port_event() vs usb_port_{suspend|resume}
// @portnum: port index num based one
// @is_superspeed cache super-speed status
// @usb3_lpm_u1_permit: whether USB3 U1 LPM is permitted.
// @usb3_lpm_u2_permit: whether USB3 U2 LPM is permitted.
// @early_stop: whether port initialization will be stopped earlier.
// @ignore_event: whether events of the port are ignored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_port {
    pub child: *mut usb_device,
    pub dev: device,
    pub port_owner: *mut usb_dev_state,
    pub peer: *mut usb_port,
    pub connector: *mut typec_connector,
    pub req: *mut dev_pm_qos_request,
    pub connect_type: usb_port_connect_type,
    pub state: usb_device_state,
    pub state_kn: *mut kernfs_node,
    pub location: usb_port_location_t,
    pub status_lock: mutex,
    pub over_current_count: u32,
    pub portnum: u8,
    pub quirks: u32,
    pub early_stop:1: c_uint,
    pub ignore_event:1: c_uint,
    pub is_superspeed:1: c_uint,
    pub usb3_lpm_u1_permit:1: c_uint,
    pub usb3_lpm_u2_permit:1: c_uint,
}

extern "C" {
    pub fn hub_get(hub: *mut usb_hub);
}
extern "C" {
    pub fn hub_put(hub: *mut usb_hub);
}
extern "C" {
    pub fn usb_port_is_power_on(hub: *mut usb_hub, portstatus: c_uint) -> c_int;
}
extern "C" {
    pub fn max(_arg: delay, _arg: 100U) -> return;
}
extern "C" {
    pub fn hub_port_debounce(_arg: hub, _arg: port1, _arg: true) -> return;
}
extern "C" {
    pub fn hub_port_debounce(_arg: hub, _arg: port1, _arg: false) -> return;
}
