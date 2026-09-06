//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/musb_gadget.h
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
// MUSB OTG driver peripheral defines
//
// Copyright 2005 Mentor Graphics Corporation
// Copyright (C) 2005-2006 by Texas Instruments
// Copyright (C) 2006-2007 Nokia Corporation
//

extern "C" {
    pub fn musb_g_ep0_irq(: *mut musb) -> irqreturn_t;
}
extern "C" {
    pub fn musb_g_tx(: *mut musb, _arg: u8);
}
extern "C" {
    pub fn musb_g_rx(: *mut musb, _arg: u8);
}
extern "C" {
    pub fn musb_g_reset(: *mut musb);
}
extern "C" {
    pub fn musb_g_suspend(: *mut musb);
}
extern "C" {
    pub fn musb_g_resume(: *mut musb);
}
extern "C" {
    pub fn musb_g_wakeup(: *mut musb);
}
extern "C" {
    pub fn musb_g_disconnect(: *mut musb);
}
extern "C" {
    pub fn musb_gadget_cleanup(: *mut musb);
}
extern "C" {
    pub fn musb_gadget_setup(: *mut musb) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum buffer_map_state {
    UN_MAPPED = 0,
    PRE_MAPPED,
    MUSB_MAPPED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_request {
    pub request: usb_request,
    pub list: list_head,
    pub ep: *mut musb_ep,
    pub musb: *mut musb,
    pub /: *mut *mut u8 tx; / endpoint direction,
    pub epnum: u8,
    pub map_state: buffer_map_state,
}

extern "C" {
    pub fn musb_free_request(ep: *mut usb_ep, req: *mut usb_request);
}
//
// struct musb_ep - peripheral side view of endpoint rx or tx side
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct musb_ep {
// stuff towards the head is basically write-once.
    pub end_point: usb_ep,
    pub name: [c_char; 12],
    pub hw_ep: *mut musb_hw_ep,
    pub musb: *mut musb,
    pub current_epnum: u8,
// ... when enabled/disabled ...
    pub type: u8,
    pub is_in: u8,
    pub packet_sz: u16,
    pub desc: *const usb_endpoint_descriptor,
    pub dma: *mut dma_channel,
// later things are modified based on usage
    pub req_list: list_head,
    pub wedged: u8,
// true if lock must be dropped but req_list may not be advanced
    pub busy: u8,
    pub hb_mult: u8,
}

extern "C" {
    pub fn container_of(_arg: queue->next, musb_request: struct, _arg: list) -> return;
}
extern "C" {
    pub fn musb_g_giveback(: *mut musb_ep, : *mut usb_request, _arg: c_int);
}
extern "C" {
    pub fn musb_ep_restart(: *mut musb, : *mut musb_request);
}
