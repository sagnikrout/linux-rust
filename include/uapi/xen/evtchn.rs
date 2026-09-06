//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/xen/evtchn.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR MIT)
//
// evtchn.h
//
// Interface to /dev/xen/evtchn.
//
// Copyright (c) 2003-2005, K A Fraser
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//
// Bind a fresh port to VIRQ @virq.
// Return allocated port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_bind_virq {
    pub virq: c_uint,
}

//
// Bind a fresh port to remote <@remote_domain, @remote_port>.
// Return allocated port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_bind_interdomain {
    pub remote_port: unsigned int remote_domain,,
}

//
// Allocate a fresh port for binding to @remote_domain.
// Return allocated port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_bind_unbound_port {
    pub remote_domain: c_uint,
}

//
// Unbind previously allocated @port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_unbind {
    pub port: c_uint,
}

//
// Unbind previously allocated @port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_notify {
    pub port: c_uint,
}

// Clear and reinitialise the event buffer. Clear error condition.

//
// Restrict this file descriptor so that it can only be used to bind
// new interdomain events from one domain.
//
// Once a file descriptor has been restricted it cannot be
// de-restricted, and must be closed and re-opened.  Event channels
// which were bound before restricting remain bound afterwards, and
// can be notified as usual.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_restrict_domid {
    pub domid: domid_t,
}

//
// Bind statically allocated @port.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_evtchn_bind {
    pub port: c_uint,
}
