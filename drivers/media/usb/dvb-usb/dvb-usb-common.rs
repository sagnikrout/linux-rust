//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb/dvb-usb-common.h
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
// dvb-usb-common.h is part of the DVB USB library.
//
// Copyright (C) 2004-5 Patrick Boettcher (patrick.boettcher@posteo.de)
// see dvb-usb-init.c for copyright information.
//
// a header file containing prototypes and types for internal use of the dvb-usb-lib
//

// commonly used  methods
extern "C" {
    pub fn dvb_usb_device_power_ctrl(d: *mut dvb_usb_device, onoff: c_int) -> c_int;
}
extern "C" {
    pub fn usb_urb_init(stream: *mut usb_data_stream, props: *mut usb_data_stream_properties) -> c_int;
}
extern "C" {
    pub fn usb_urb_exit(stream: *mut usb_data_stream) -> c_int;
}
extern "C" {
    pub fn usb_urb_submit(stream: *mut usb_data_stream) -> c_int;
}
extern "C" {
    pub fn usb_urb_kill(stream: *mut usb_data_stream) -> c_int;
}
extern "C" {
    pub fn dvb_usb_adapter_stream_init(adap: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dvb_usb_adapter_stream_exit(adap: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dvb_usb_i2c_init(: *mut dvb_usb_device) -> c_int;
}
extern "C" {
    pub fn dvb_usb_i2c_exit(: *mut dvb_usb_device) -> c_int;
}
extern "C" {
    pub fn dvb_usb_adapter_dvb_exit(adap: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dvb_usb_adapter_frontend_init(adap: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dvb_usb_adapter_frontend_exit(adap: *mut dvb_usb_adapter) -> c_int;
}
extern "C" {
    pub fn dvb_usb_remote_init(: *mut dvb_usb_device) -> c_int;
}
extern "C" {
    pub fn dvb_usb_remote_exit(: *mut dvb_usb_device) -> c_int;
}
