//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/dwc3/gadget.h
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
// gadget.h - DesignWare USB3 DRD Gadget Header
//
// Copyright (C) 2010-2011 Texas Instruments Incorporated - https://www.ti.com
//
// Authors: Felipe Balbi <balbi@ti.com>,
// Sebastian Andrzej Siewior <bigeasy@linutronix.de>
//

// DEPCFG parameter 1

// DEPCFG parameter 0

// This applies for core versions earlier than 1.94a

// These apply for core versions 1.94a and later

// DEPXFERCFG parameter 0

// U1 Device exit Latency
pub const DWC3_DEFAULT_U1_DEV_EXIT_LAT: c_uint = 0x0A	/* Less then 10 microsec */;
// U2 Device exit Latency
pub const DWC3_DEFAULT_U2_DEV_EXIT_LAT: c_uint = 0x1FF	/* Less then 511 microsec */;
// Frame/Microframe Number Mask
pub const DWC3_FRNUMBER_MASK: c_uint = 0x3fff;
// --------------------------------------------------------------------------

//
// next_request - gets the next request on the given list
// @list: the request list to operate on
//
// Caller should take care of locking. This function return %NULL or the first
// request available on @list.
//
extern "C" {
    pub fn list_first_entry_or_null(_arg: list, dwc3_request: struct, _arg: list) -> return;
}
//
// dwc3_gadget_move_started_request - move @req to the started_list
// @req: the request to be moved
//
// Caller should take care of locking. This function will move @req from its
// current list to the endpoint's started_list.
//
// dwc3_gadget_move_cancelled_request - move @req to the cancelled_list
// @req: the request to be moved
// @reason: cancelled reason for the dwc3 request
//
// Caller should take care of locking. This function will move @req from its
// current list to the endpoint's cancelled_list.
//
extern "C" {
    pub fn dwc3_ep0_out_start(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_ep0_end_control_data(dwc: *mut dwc3, dep: *mut dwc3_ep);
}
extern "C" {
    pub fn dwc3_ep0_stall_and_restart(dwc: *mut dwc3);
}
extern "C" {
    pub fn __dwc3_gadget_ep0_set_halt(ep: *mut usb_ep, value: c_int) -> c_int;
}
extern "C" {
    pub fn dwc3_gadget_ep0_set_halt(ep: *mut usb_ep, value: c_int) -> c_int;
}
extern "C" {
    pub fn __dwc3_gadget_ep_set_halt(dep: *mut dwc3_ep, value: c_int, protocol: c_int) -> c_int;
}
extern "C" {
    pub fn dwc3_ep0_send_delayed_status(dwc: *mut dwc3);
}
extern "C" {
    pub fn dwc3_stop_active_transfer(dep: *mut dwc3_ep, force: bool, interrupt: bool);
}
extern "C" {
    pub fn dwc3_gadget_start_config(dwc: *mut dwc3, resource_index: c_uint) -> c_int;
}
//
// dwc3_gadget_ep_get_transfer_index - Gets transfer index from HW
// @dep: dwc3 endpoint
//
// Caller should take care of locking. Returns the transfer resource
// index for a given endpoint.
//
// dwc3_gadget_dctl_write_safe - write to DCTL safe from link state change
// @dwc: pointer to our context structure
// @value: value to write to DCTL
//
// Use this function when doing read-modify-write to DCTL. It will not
// send link state change request.
//
