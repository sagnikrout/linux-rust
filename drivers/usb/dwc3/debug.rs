//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc3/debug.h
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
// debug.h - DesignWare USB3 DRD Controller Debug Header
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

//
// dwc3_mode_string - returns mode name
// @mode: GCTL.PrtCapDir value
//
// dwc3_gadget_ep_cmd_string - returns endpoint command string
// @cmd: command code
//
// dwc3_gadget_generic_cmd_string - returns generic command string
// @cmd: command code
//
// dwc3_gadget_link_string - returns link name
// @link_state: link state code
//
// dwc3_gadget_hs_link_string - returns highspeed and below link name
// @link_state: link state code
//
// dwc3_trb_type_string - returns TRB type as a string
// @type: the type of the TRB
//
// dwc3_gadget_event_string - returns event name
// @event: the event code
//
// dwc3_ep_event_string - returns event name
// @event: then event code
//
// Control Endpoints
//
// dwc3_gadget_event_type_string - return event name
// @event: the event code
//
extern "C" {
    pub fn dwc3_gadget_event_string(_arg: str, _arg: size, _arg: &evt.devt) -> return;
}
extern "C" {
    pub fn dwc3_ep_event_string(_arg: str, _arg: size, _arg: &evt.depevt, _arg: ep0state) -> return;
}

extern "C" {
    pub fn dwc3_debugfs_create_endpoint_dir(dep: *mut dwc3_ep);
}
extern "C" {
    pub fn dwc3_debugfs_remove_endpoint_dir(dep: *mut dwc3_ep);
}
extern "C" {
    pub fn dwc3_debugfs_init(d: *mut dwc3);
}
extern "C" {
    pub fn dwc3_debugfs_exit(d: *mut dwc3);
}

