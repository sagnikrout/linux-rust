//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/function/u_serial.h
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
// u_serial.h - interface to USB gadget "serial port"/TTY utilities
//
// Copyright (C) 2008 David Brownell
// Copyright (C) 2008 by Nokia Corporation
//

pub const MAX_U_SERIAL_PORTS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f_serial_opts {
    pub func_inst: usb_function_instance,
    pub port_num: u8,
    pub protocol: u8,
    pub /: *mut *mut mutex lock; / protect instances,
    pub instances: c_int,
}

//
// One non-multiplexed "serial" I/O port ... there can be several of these
// on any given USB peripheral device, if it provides enough endpoints.
//
// The "u_serial" utility component exists to do one thing:  manage TTY
// style I/O using the USB peripheral endpoints listed here, including
// hookups to sysfs and /dev for each logical "tty" device.
//
// REVISIT at least ACM could support tiocmget() if needed.
//
// REVISIT someday, allow multiplexing several TTYs over these endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gserial {
    pub func: usb_function,
// port is managed by gserial_{connect,disconnect}
    pub ioport: *mut gs_port,
    pub in: *mut usb_ep,
    pub out: *mut usb_ep,
// REVISIT avoid this CDC-ACM support harder ...
    pub /: *mut *mut usb_cdc_line_coding port_line_coding; / 9600-8-N-1 etc,
// notification callbacks
    pub p): *mut *mut void (connect)(struct gserial,
    pub p): *mut *mut void (disconnect)(struct gserial,
    pub duration): *mut *mut *mut int (send_break)(struct gserial p, int,
}

// utilities to allocate/free request and buffer
extern "C" {
    pub fn gs_free_req(: *mut usb_ep, req: *mut usb_request);
}
// management of individual TTY ports
extern "C" {
    pub fn gserial_alloc_line_no_console(port_line: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn gserial_alloc_line(port_line: *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn gserial_free_line(port_line: c_uchar);
}

extern "C" {
    pub fn gserial_set_console(port_num: c_uchar, page: *const c_char, count: usize) -> isize;
}
extern "C" {
    pub fn gserial_get_console(port_num: c_uchar, page: *mut c_char) -> isize;
}

// connect/disconnect is handled by individual functions
extern "C" {
    pub fn gserial_connect(: *mut gserial, port_num: u8) -> c_int;
}
extern "C" {
    pub fn gserial_disconnect(: *mut gserial);
}
extern "C" {
    pub fn gserial_suspend(p: *mut gserial);
}
extern "C" {
    pub fn gserial_resume(p: *mut gserial);
}
