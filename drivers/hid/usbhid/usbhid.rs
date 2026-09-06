//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/usbhid/usbhid.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2001 Vojtech Pavlik
// Copyright (c) 2006 Jiri Kosina
//

// API provided by hid-core.c for USB HID drivers
extern "C" {
    pub fn usbhid_init_reports(hid: *mut hid_device);
}
// iofl flags
pub const HID_CTRL_RUNNING: c_int = 1;
pub const HID_OUT_RUNNING: c_int = 2;
pub const HID_IN_RUNNING: c_int = 3;
pub const HID_RESET_PENDING: c_int = 4;
pub const HID_SUSPENDED: c_int = 5;
pub const HID_CLEAR_HALT: c_int = 6;
pub const HID_DISCONNECTED: c_int = 7;
pub const HID_STARTED: c_int = 8;
pub const HID_KEYS_PRESSED: c_int = 10;
pub const HID_NO_BANDWIDTH: c_int = 11;
pub const HID_RESUME_RUNNING: c_int = 12;
//
// The device is opened, meaning there is a client that is interested
// in data coming from the device.
//
pub const HID_OPENED: c_int = 13;
//
// We are polling input endpoint by [re]submitting IN URB, because
// either HID device is opened or ALWAYS POLL quirk is set for the
// device.
//
pub const HID_IN_POLLING: c_int = 14;
//
// USB-specific HID struct, to be pointed to
// from struct hid_device->driver_data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usbhid_device {
    pub /: *mut *mut *mut hid_device hid; / pointer to corresponding HID dev,
    pub /: *mut *mut *mut usb_interface intf; / USB interface,
    pub /: *mut *mut int ifnum; / USB interface number,
    pub /: *mut *mut unsigned int bufsize; / URB buffer size,
    pub /: *mut *mut *mut urb urbin; / Input URB,
    pub /: *mut *mut *mut char inbuf; / Input buffer,
    pub /: *mut *mut dma_addr_t inbuf_dma; / Input buffer dma,
    pub /: *mut *mut *mut urb urbctrl; / Control URB,
    pub /: *mut *mut *mut usb_ctrlrequest cr; / Control request struct,
    pub /: *mut *mut hid_control_fifo ctrl[HID_CONTROL_FIFO_SIZE]; / Control fifo,
    pub /: *mut *mut unsigned char ctrlhead, ctrltail; / Control fifo head & tail,
    pub /: *mut *mut *mut char ctrlbuf; / Control buffer,
    pub /: *mut *mut dma_addr_t ctrlbuf_dma; / Control buffer dma,
    pub /: *mut *mut unsigned long last_ctrl; / record of last output for timeouts,
    pub /: *mut *mut *mut urb urbout; / Output URB,
    pub /: *mut *mut hid_output_fifo out[HID_CONTROL_FIFO_SIZE]; / Output pipe fifo,
    pub /: *mut *mut unsigned char outhead, outtail; / Output pipe fifo head & tail,
    pub /: *mut *mut *mut char outbuf; / Output buffer,
    pub /: *mut *mut dma_addr_t outbuf_dma; / Output buffer dma,
    pub /: *mut *mut unsigned long last_out; / record of last output for timeouts,
    pub /: *mut *mut mutex mutex; / start/stop/open/close,
    pub /: *mut *mut spinlock_t lock; / fifo spinlock,
    pub /: *mut *mut unsigned long iofl; / I/O flags (CTRL_RUNNING, OUT_RUNNING),
    pub /: *mut *mut timer_list io_retry; / Retry timer,
    pub /: *mut *mut unsigned long stop_retry; / Time to give up, in jiffies,
    pub /: *mut *mut unsigned int retry_delay; / Delay length in ms,
    pub /: *mut *mut work_reset_work; / Task context for resets,
    pub /: *mut *mut wait_queue_head_t wait; / For sleeping,
}

