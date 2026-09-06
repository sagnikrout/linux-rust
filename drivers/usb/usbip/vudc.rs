//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/usbip/vudc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2015 Karol Kosik <karo9@interia.eu>
// Copyright (C) 2015-2016 Samsung Electronics
// Igor Kotrasinski <i.kotrasinsk@samsung.com>
// Krzysztof Opasiak <k.opasiak@samsung.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vep {
    pub ep: usb_ep,
    pub /: *mut *mut *mut unsigned type:2; / type, as USB_ENDPOINT_XFER_,
    pub /: *mut *mut char name[8]; / space for ep name,
    pub desc: *const usb_endpoint_descriptor,
    pub gadget: *mut usb_gadget,
    pub /: *mut *mut list_head req_queue; / Request queue,
    pub halted:1: unsigned,
    pub wedged:1: unsigned,
    pub already_seen:1: unsigned,
    pub setup_stage:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vrequest {
    pub req: usb_request,
    pub /: *mut *mut list_head req_entry; / Request queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct urbp {
    pub urb: *mut urb,
    pub ep: *mut vep,
    pub /: *mut *mut list_head urb_entry; / urb queue,
    pub seqnum: c_ulong,
    pub /: *mut *mut unsigned type:2; / for tx, since ep type can change after,
    pub new:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct v_unlink {
    pub seqnum: c_ulong,
    pub status: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx_type {
    TX_UNLINK,
    TX_SUBMIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_item {
    pub tx_entry: list_head,
    pub type: tx_type,
    pub s: *mut urbp,
    pub u: *mut v_unlink,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tr_state {
    VUDC_TR_RUNNING,
    VUDC_TR_IDLE,
    VUDC_TR_STOPPED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transfer_timer {
    pub timer: timer_list,
    pub state: tr_state,
    pub frame_start: c_ulong,
    pub frame_limit: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vudc {
    pub gadget: usb_gadget,
    pub driver: *mut usb_gadget_driver,
    pub pdev: *mut platform_device,
    pub dev_desc: usb_device_descriptor,
    pub ud: usbip_device,
    pub tr_timer: transfer_timer,
    pub start_time: timespec64,
    pub urb_queue: list_head,
    pub lock_tx: spinlock_t,
    pub tx_queue: list_head,
    pub tx_waitq: wait_queue_head_t,
    pub lock: spinlock_t,
    pub ep: *mut vep,
    pub address: c_int,
    pub devstatus: u16,
    pub pullup:1: unsigned,
    pub connected:1: unsigned,
    pub desc_cached:1: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vudc_device {
    pub pdev: *mut platform_device,
    pub dev_entry: list_head,
}

// visible everywhere
extern "C" {
    pub fn container_of(_arg: _ep, vep: struct, _arg: ep) -> return;
}
extern "C" {
    pub fn container_of(_arg: _req, vrequest: struct, _arg: req) -> return;
}
extern "C" {
    pub fn container_of(_arg: _gadget, vudc: struct, _arg: gadget) -> return;
}
extern "C" {
    pub fn container_of(_arg: ep->gadget, vudc: struct, _arg: gadget) -> return;
}
// vudc_sysfs.c
extern "C" {
    pub fn get_gadget_descs(udc: *mut vudc) -> c_int;
}
// vudc_tx.c
extern "C" {
    pub fn v_tx_loop(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn v_enqueue_ret_unlink(udc: *mut vudc, seqnum: __u32, status: __u32);
}
extern "C" {
    pub fn v_enqueue_ret_submit(udc: *mut vudc, urb_p: *mut urbp);
}
// vudc_rx.c
extern "C" {
    pub fn v_rx_loop(data: *mut c_void) -> c_int;
}
// vudc_transfer.c
extern "C" {
    pub fn v_init_timer(udc: *mut vudc);
}
extern "C" {
    pub fn v_start_timer(udc: *mut vudc);
}
extern "C" {
    pub fn v_kick_timer(udc: *mut vudc, time: c_ulong);
}
extern "C" {
    pub fn v_stop_timer(udc: *mut vudc);
}
// vudc_dev.c
extern "C" {
    pub fn free_urbp_and_urb(urb_p: *mut urbp);
}
extern "C" {
    pub fn put_vudc_device(udc_dev: *mut vudc_device);
}
extern "C" {
    pub fn vudc_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn vudc_remove(pdev: *mut platform_device);
}
