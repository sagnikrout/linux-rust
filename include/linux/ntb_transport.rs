//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ntb_transport.h
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


//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// GPL LICENSE SUMMARY
//
// Copyright(c) 2012 Intel Corporation. All rights reserved.
// Copyright (C) 2015 EMC Corporation. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of version 2 of the GNU General Public License as
// published by the Free Software Foundation.
//
// BSD LICENSE
//
// Copyright(c) 2012 Intel Corporation. All rights reserved.
// Copyright (C) 2015 EMC Corporation. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copy
// notice, this list of conditions and the following disclaimer in
// the documentation and/or other materials provided with the
// distribution.
// * Neither the name of Intel Corporation nor the names of its
// contributors may be used to endorse or promote products derived
// from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// PCIe NTB Transport Linux driver
//
// Contact Information:
// Jon Mason <jon.mason@intel.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_transport_client {
    pub driver: device_driver,
    pub client_dev): *mut *mut int (probe)(struct device,
    pub client_dev): *mut *mut void (remove)(struct device,
}

extern "C" {
    pub fn ntb_transport_register_client(drvr: *mut ntb_transport_client) -> c_int;
}
extern "C" {
    pub fn ntb_transport_unregister_client(drvr: *mut ntb_transport_client);
}
extern "C" {
    pub fn ntb_transport_register_client_dev(device_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn ntb_transport_unregister_client_dev(device_name: *mut c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_queue_handlers {
    pub len): *mut *mut void data, int,
    pub len): *mut *mut void data, int,
    pub status): *mut *mut *mut void (event_handler)(void data, int,
}

extern "C" {
    pub fn ntb_transport_qp_num(qp: *mut ntb_transport_qp) -> c_uchar;
}
extern "C" {
    pub fn ntb_transport_max_size(qp: *mut ntb_transport_qp) -> c_uint;
}
extern "C" {
    pub fn ntb_transport_free_queue(qp: *mut ntb_transport_qp);
}
extern "C" {
    pub fn ntb_transport_link_up(qp: *mut ntb_transport_qp);
}
extern "C" {
    pub fn ntb_transport_link_down(qp: *mut ntb_transport_qp);
}
extern "C" {
    pub fn ntb_transport_link_query(qp: *mut ntb_transport_qp) -> bool;
}
extern "C" {
    pub fn ntb_transport_tx_free_entry(qp: *mut ntb_transport_qp) -> c_uint;
}
