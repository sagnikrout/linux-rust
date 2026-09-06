//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/atm/usbatm.h
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
// usbatm.h - Generic USB xDSL driver core
//
// Copyright (C) 2001, Alcatel
// Copyright (C) 2003, Duncan Sands, SolNegro, Josep Comas
// Copyright (C) 2004, David Woodhouse
//

//
// Macro flag: #define VERBOSE_DEBUG
//

// FIXME: move to dev_* once ATM is driver model aware

// flags, set by mini-driver in bind()

// mini driver
//
// Assuming all methods exist and succeed, they are called in this order:
//
// bind, heavy_init, atm_start, ..., atm_stop, unbind
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbatm_driver {
    pub driver_name: *const c_char,
// init device ... can sleep, or cause probe() failure
    pub id): *const usb_device_id,
// additional device initialization that is too slow to be done in probe()
    pub ): *mut *mut *mut int (heavy_init) (struct usbatm_data , struct usb_interface,
// cleanup device ... can sleep, but can't fail
    pub ): *mut *mut *mut void (unbind) (struct usbatm_data , struct usb_interface,
// init ATM device ... can sleep, or cause ATM initialization failure
    pub ): *mut *mut *mut int (atm_start) (struct usbatm_data , struct atm_dev,
// cleanup ATM device ... can sleep, but can't fail
    pub ): *mut *mut *mut void (atm_stop) (struct usbatm_data , struct atm_dev,
    pub /: *mut *mut int bulk_in; / bulk rx endpoint,
    pub /: *mut *mut int isoc_in; / isochronous rx endpoint,
    pub /: *mut *mut int bulk_out; / bulk tx endpoint,
    pub rx_padding: unsigned,
    pub tx_padding: unsigned,
}

extern "C" {
    pub fn usbatm_usb_disconnect(intf: *mut usb_interface);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbatm_channel {
    pub /: *mut *mut int endpoint; / usb pipe,
    pub /: *mut *mut unsigned int stride; / ATM cell size + padding,
    pub /: *mut *mut unsigned int buf_size; / urb buffer size,
    pub /: *mut *mut unsigned int packet_size; / endpoint maxpacket,
    pub lock: spinlock_t,
    pub list: list_head,
    pub tasklet: tasklet_struct,
    pub delay: timer_list,
    pub usbatm: *mut usbatm_data,
}

// main driver data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbatm_data {
//
// public fields
//
// mini driver
    pub driver: *mut usbatm_driver,
    pub driver_data: *mut c_void,
    pub driver_name: [c_char; 16],
    pub /: *mut *mut unsigned int flags; / set by mini-driver in bind(),
// USB device
    pub usb_dev: *mut usb_device,
    pub usb_intf: *mut usb_interface,
    pub description: [c_char; 64],
// ATM device
    pub atm_dev: *mut atm_dev,
//
// private fields - do not use
//
    pub refcount: kref,
    pub serialize: mutex,
    pub disconnected: c_int,
// heavy init
    pub thread: *mut task_struct,
    pub thread_started: completion,
    pub thread_exited: completion,
// ATM device
    pub vcc_list: list_head,
    pub rx_channel: usbatm_channel,
    pub tx_channel: usbatm_channel,
    pub sndqueue: sk_buff_head,
    pub /: *mut *mut *mut sk_buff current_skb; / being emptied,
    pub cached_vcc: *mut usbatm_vcc_data,
    pub cached_vci: c_int,
    pub cached_vpi: c_short,
    pub /: *mut *mut *mut unsigned char cell_buf; / holds partial rx cell,
    pub buf_usage: c_uint,
    pub urbs: [*mut urb; ],
}
