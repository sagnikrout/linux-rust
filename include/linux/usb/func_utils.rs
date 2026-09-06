//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/func_utils.h
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
// func_utils.h
//
// Utility definitions for USB functions
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

// Variable Length Array Macros

//
// alloc_ep_req - returns a usb_request allocated by the gadget driver and
// allocates the request's buffer.
//
// @ep: the endpoint to allocate a usb_request
// @len: usb_requests's buffer suggested size
//
// In case @ep direction is OUT, the @len will be aligned to ep's
// wMaxPacketSize. In order to avoid memory leaks or drops, *always* use
// usb_requests's length (req->length) to refer to the allocated buffer size.
// Requests allocated via alloc_ep_req() *must* be freed by free_ep_req().
//
// Frees a usb_request previously allocated by alloc_ep_req()
