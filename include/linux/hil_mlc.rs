//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hil_mlc.h
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
// HP Human Interface Loop Master Link Controller driver.
//
// Copyright (c) 2001 Brian S. Julin
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions, and the following disclaimer,
// without modification.
// 2. The name of the author may not be used to endorse or promote products
// derived from this software without specific prior written permission.
//
// Alternatively, this software may be distributed under the terms of the
// GNU General Public License ("GPL").
//
// THIS SOFTWARE IS PROVIDED BY THE AUTHOR AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE AUTHOR OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
//
// References:
// HP-HIL Technical Reference Manual.  Hewlett Packard Product No. 45918A
//

pub type hil_mlc = hil_mlc;
// The HIL has a complicated state engine.
// We define the structure of nodes in the state engine here.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hilse_act {
// HILSE_OUT prepares to receive input if the next node
// is an IN or EXPECT, and then sends the given packet.
//
    HILSE_OUT = 0,

// HILSE_CTS checks if the loop is busy.
    HILSE_CTS,

// HILSE_OUT_LAST sends the given command packet to
// the last configured/running device on the loop.
//
    HILSE_OUT_LAST,

// HILSE_OUT_DISC sends the given command packet to
// the next device past the last configured/running one.
//
    HILSE_OUT_DISC,

// HILSE_FUNC runs a callback function with given arguments.
// a positive return value causes the "ugly" branch to be taken.
//
    HILSE_FUNC,

// HILSE_IN simply expects any non-errored packet to arrive
// within arg usecs.
//
    HILSE_IN		= 0x100,

// HILSE_EXPECT expects a particular packet to arrive
// within arg usecs, any other packet is considered an error.
//
    HILSE_EXPECT,

// HILSE_EXPECT_LAST as above but dev field should be last
// discovered/operational device.
//
    HILSE_EXPECT_LAST,

// HILSE_EXPECT_LAST as above but dev field should be first
// undiscovered/inoperational device.
//
    HILSE_EXPECT_DISC
}

extern "C" {
    pub fn int(mlc: *mut hilse_func) (hil_mlc, arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hilse_node {
    pub /: *mut *mut hilse_act act; / How to process this node,
    pub /: *mut *mut *mut hilse_func func; / Function to call if HILSE_FUNC,
    pub /: *mut *mut hil_packet packet; / Packet to send or to compare,
    pub object: },
    pub /: *mut *mut int arg; / Timeout in usec or parm for func,
    pub /: *mut *mut int good; / Node to jump to on success,
    pub /: *mut *mut int bad; / Node to jump to on error,
    pub /: *mut *mut int ugly; / Node to jump to on timeout,
}

// Methods for back-end drivers, e.g. hp_sdc_mlc
extern "C" {
    pub fn int(mlc: *mut hil_mlc_cts) (hil_mlc) -> typedef;
}
extern "C" {
    pub fn int(mlc: *mut hil_mlc_out) (hil_mlc) -> typedef;
}
extern "C" {
    pub fn int(mlc: *mut hil_mlc_in) (hil_mlc, timeout: suseconds_t) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hil_mlc_devinfo {
    pub /: *mut *mut uint8_t idd[16]; / Device ID Byte and Describe Record,
    pub /: *mut *mut uint8_t rsc[16]; / Security Code Header and Record,
    pub /: *mut *mut uint8_t exd[16]; / Extended Describe Record,
    pub /: *mut *mut uint8_t rnm[16]; / Device name as returned by RNM command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hil_mlc_serio_map {
    pub mlc: *mut hil_mlc,
    pub di_revmap: c_int,
    pub didx: c_int,
}

// How many (possibly old/detached) devices the we try to keep track of
pub const HIL_MLC_DEVMEM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hil_mlc {
    pub /: *mut *mut list_head list; / hil_mlc is organized as linked list,
    pub lock: rwlock_t,
    pub /: *mut *mut *mut void priv; / Data specific to a particular type of MLC,
    pub /: *mut *mut int seidx; / Current node in state engine,
    pub ostarted: int istarted,,
    pub cts: *mut hil_mlc_cts,
    pub /: *mut *mut semaphore csem; / Raised when loop idle,
    pub out: *mut hil_mlc_out,
    pub /: *mut *mut semaphore osem; / Raised when outpacket dispatched,
    pub opacket: hil_packet,
    pub in: *mut hil_mlc_in,
    pub /: *mut *mut semaphore isem; / Raised when a packet arrives,
    pub ipacket: [hil_packet; 16],
    pub imatch: hil_packet,
    pub icount: c_int,
    pub instart: c_ulong,
    pub intimeout: c_ulong,
    pub /: *mut *mut int ddi; / Last operational device id,
    pub /: *mut *mut int lcv; / LCV to throttle loops,
    pub /: *mut *mut time64_t lcv_time; / Time loop was started,
    pub /: *mut *mut int di_map[7]; / Maps below items to live devs,
    pub di: [hil_mlc_devinfo; HIL_MLC_DEVMEM],
    pub serio: [*mut serio; HIL_MLC_DEVMEM],
    pub serio_map: [hil_mlc_serio_map; HIL_MLC_DEVMEM],
    pub serio_opacket: [hil_packet; HIL_MLC_DEVMEM],
    pub serio_oidx: [c_int; HIL_MLC_DEVMEM],
    pub /: *mut *mut hil_mlc_devinfo di_scratch; / Temporary area,
    pub opercnt: c_int,
    pub tasklet: *mut tasklet_struct,
}

extern "C" {
    pub fn hil_mlc_register(mlc: *mut hil_mlc) -> c_int;
}
extern "C" {
    pub fn hil_mlc_unregister(mlc: *mut hil_mlc) -> c_int;
}
