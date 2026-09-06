//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/isp1760/isp1760-udc.h
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
// Driver for the NXP ISP1761 device controller
//
// Copyright 2021 Linaro, Rui Miguel Silva
// Copyright 2014 Ideas on Board Oy
//
// Contacts:
// Laurent Pinchart <laurent.pinchart@ideasonboard.com>
// Rui Miguel Silva <rui.silva@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isp1760_ctrl_state {
    ISP1760_CTRL_SETUP,		/* Waiting for a SETUP transaction */
    ISP1760_CTRL_DATA_IN,		/* Setup received, data IN stage */
    ISP1760_CTRL_DATA_OUT,		/* Setup received, data OUT stage */
    ISP1760_CTRL_STATUS,		/* 0-length request in status stage */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_ep {
    pub udc: *mut isp1760_udc,
    pub ep: usb_ep,
    pub queue: list_head,
    pub addr: c_uint,
    pub maxpacket: c_uint,
    pub name: [c_char; 7],
    pub desc: *const usb_endpoint_descriptor,
    pub rx_pending: bool,
    pub halted: bool,
    pub wedged: bool,
}

//
// struct isp1760_udc - UDC state information
// irq: IRQ number
// irqname: IRQ name (as passed to request_irq)
// regs: regmap for UDC registers
// driver: Gadget driver
// gadget: Gadget device
// lock: Protects driver, vbus_timer, ep, ep0_*, DC_EPINDEX register
// ep: Array of endpoints
// ep0_state: Control request state for endpoint 0
// ep0_dir: Direction of the current control request
// ep0_length: Length of the current control request
// connected: Tracks gadget driver bus connection state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isp1760_udc {
    pub isp: *mut isp1760_device,
    pub irq: c_int,
    pub irqname: *mut c_char,
    pub regs: *mut regmap,
    pub fields: [*mut regmap_field; DC_FIELD_MAX],
    pub driver: *mut usb_gadget_driver,
    pub gadget: usb_gadget,
    pub lock: spinlock_t,
    pub vbus_timer: timer_list,
    pub ep: [isp1760_ep; 15],
    pub ep0_state: isp1760_ctrl_state,
    pub ep0_dir: u8,
    pub ep0_length: u16,
    pub connected: bool,
    pub is_isp1763: bool,
    pub devstatus: c_uint,
}

extern "C" {
    pub fn isp1760_udc_unregister(isp: *mut isp1760_device);
}

