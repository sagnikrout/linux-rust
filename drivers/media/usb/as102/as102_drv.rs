//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/as102/as102_drv.h
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
// Abilis Systems Single DVB-T Receiver
// Copyright (C) 2008 Pierrick Hascoet <pierrick.hascoet@abilis.com>
//

pub const AS102_DEVICE_MAJOR: c_int = 192;
pub const AS102_USB_BUF_SIZE: c_int = 512;
pub const MAX_STREAM_URB: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct as10x_bus_adapter_t {
    pub usb_dev: *mut usb_device,
// bus token lock
    pub lock: mutex,
// low level interface for bus adapter
#[repr(C)]
#[derive(Copy, Clone)]
pub union as10x_bus_token_t {
// usb token
    pub usb: as10x_usb_token_cmd_t,
    pub token: },
// token cmd xfer id
    pub cmd_xid: u16,
// as10x command and response for dvb interface
    pub rsp: *mut *mut as10x_cmd_t cmd,,
// bus adapter private ops callback
    pub ops: *const as102_priv_ops_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct as102_dev_t {
    pub name: *const c_char,
    pub bus_adap: as10x_bus_adapter_t,
    pub device_entry: list_head,
    pub kref: kref,
    pub elna_cfg: u8,
    pub dvb_adap: dvb_adapter,
    pub dvb_fe: *mut dvb_frontend,
    pub dvb_dmx: dvb_demux,
    pub dvb_dmxdev: dmxdev,
// timer handle to trig ts stream download
    pub timer_handle: timer_list,
    pub sem: mutex,
    pub dma_addr: dma_addr_t,
    pub stream: *mut c_void,
    pub streaming: c_int,
    pub stream_urb: [*mut urb; MAX_STREAM_URB],
}

extern "C" {
    pub fn as102_dvb_register(dev: *mut as102_dev_t) -> c_int;
}
extern "C" {
    pub fn as102_dvb_unregister(dev: *mut as102_dev_t);
}
