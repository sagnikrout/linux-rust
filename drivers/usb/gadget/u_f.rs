//! Automatically rewritten from C to Rust
//! Source: drivers/usb/gadget/u_f.c
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
// u_f.c -- USB function utilities for Gadget stack
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Author: Andrzej Pietrasiewicz <andrzejtp2010@gmail.com>
//

    struct usb_request *alloc_ep_req(struct usb_ep *ep, size_t len)
    {
    struct usb_request      *req;
    req = usb_ep_alloc_request(ep, GFP_ATOMIC);
    if (req) {
    req.length = usb_endpoint_dir_out(ep.desc) ?
    usb_ep_align(ep, len) : len;
    req.buf = kmalloc(req.length, GFP_ATOMIC);
    if (!req.buf) {
    usb_ep_free_request(ep, req);
    req = core::ptr::null_mut();
    }
    }
    return req;
    }
    EXPORT_SYMBOL_GPL(alloc_ep_req);
