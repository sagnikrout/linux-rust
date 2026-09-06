//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/fsl_hypervisor.h
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
// Freescale hypervisor ioctl and kernel interface
//
// Copyright (C) 2008-2011 Freescale Semiconductor, Inc.
// Author: Timur Tabi <timur@freescale.com>
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
// * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// * Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// * Neither the name of Freescale Semiconductor nor the
// names of its contributors may be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// ALTERNATIVELY, this software may be distributed under the terms of the
// GNU General Public License ("GPL") as published by the Free Software
// Foundation, either version 2 of that License or (at your option) any
// later version.
//
// This software is provided by Freescale Semiconductor "as is" and any
// express or implied warranties, including, but not limited to, the implied
// warranties of merchantability and fitness for a particular purpose are
// disclaimed. In no event shall Freescale Semiconductor be liable for any
// direct, indirect, incidental, special, exemplary, or consequential damages
// (including, but not limited to, procurement of substitute goods or services;
// loss of use, data, or profits; or business interruption) however caused and
// on any theory of liability, whether in contract, strict liability, or tort
// (including negligence or otherwise) arising in any way out of the use of this
// software, even if advised of the possibility of such damage.
//
// This file is used by the Freescale hypervisor management driver.  It can
// also be included by applications that need to communicate with the driver
// via the ioctl interface.
//

//
// fsl_hv_event_register() - register a callback for failover events
// @nb: pointer to caller-supplied notifier_block structure
//
// This function is called by device drivers to register their callback
// functions for fail-over events.
//
// The caller should allocate a notifier_block object and initialize the
// 'priority' and 'notifier_call' fields.
//
extern "C" {
    pub fn fsl_hv_failover_register(nb: *mut notifier_block) -> c_int;
}
//
// fsl_hv_event_unregister() - unregister a callback for failover events
// @nb: the same 'nb' used in previous fsl_hv_failover_register call
//
extern "C" {
    pub fn fsl_hv_failover_unregister(nb: *mut notifier_block) -> c_int;
}
