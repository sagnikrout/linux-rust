//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/gadget/u_os_desc.h
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
// u_os_desc.h
//
// Utility definitions for "OS Descriptors" support
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

pub const USB_EXT_PROP_DW_SIZE: c_int = 0;
pub const USB_EXT_PROP_DW_PROPERTY_DATA_TYPE: c_int = 4;
pub const USB_EXT_PROP_W_PROPERTY_NAME_LENGTH: c_int = 8;
pub const USB_EXT_PROP_B_PROPERTY_NAME: c_int = 10;
pub const USB_EXT_PROP_DW_PROPERTY_DATA_LENGTH: c_int = 10;
pub const USB_EXT_PROP_B_PROPERTY_DATA: c_int = 14;
pub const USB_EXT_PROP_RESERVED: c_int = 0;
pub const USB_EXT_PROP_UNICODE: c_int = 1;
pub const USB_EXT_PROP_UNICODE_ENV: c_int = 2;
pub const USB_EXT_PROP_BINARY: c_int = 3;
pub const USB_EXT_PROP_LE32: c_int = 4;
pub const USB_EXT_PROP_BE32: c_int = 5;
pub const USB_EXT_PROP_UNICODE_LINK: c_int = 6;
pub const USB_EXT_PROP_UNICODE_MULTI: c_int = 7;
extern "C" {
    pub fn __usb_ext_prop_ptr(_arg: buf, _arg: USB_EXT_PROP_DW_SIZE) -> return;
}
extern "C" {
    pub fn __usb_ext_prop_ptr(_arg: buf, _arg: USB_EXT_PROP_DW_PROPERTY_DATA_TYPE) -> return;
}
extern "C" {
    pub fn __usb_ext_prop_ptr(_arg: buf, _arg: USB_EXT_PROP_W_PROPERTY_NAME_LENGTH) -> return;
}
extern "C" {
    pub fn __usb_ext_prop_ptr(_arg: buf, _arg: USB_EXT_PROP_B_PROPERTY_NAME) -> return;
}
extern "C" {
    pub fn __usb_ext_prop_ptr(_arg: buf, off: USB_EXT_PROP_B_PROPERTY_DATA +) -> return;
}
