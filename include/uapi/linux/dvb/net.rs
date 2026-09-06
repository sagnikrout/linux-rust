//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/dvb/net.h
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


// SPDX-License-Identifier: LGPL-2.1+ WITH Linux-syscall-note
//
// net.h
//
// Copyright (C) 2000 Marcus Metzler <marcus@convergence.de>
// & Ralph  Metzler <ralph@convergence.de>
// for convergence integrated media GmbH
//

//
// struct dvb_net_if - describes a DVB network interface
//
// @pid: Packet ID (PID) of the MPEG-TS that contains data
// @if_num: number of the Digital TV interface.
// @feedtype: Encapsulation type of the feed.
//
// A MPEG-TS stream may contain packet IDs with IP packages on it.
// This struct describes it, and the type of encoding.
//
// @feedtype can be:
//
// - %DVB_NET_FEEDTYPE_MPE for MPE encoding
// - %DVB_NET_FEEDTYPE_ULE for ULE encoding.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dvb_net_if {
    pub pid: __u16,
    pub if_num: __u16,
    pub feedtype: __u8,

}

// binary compatibility cruft:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __dvb_net_if_old {
    pub pid: __u16,
    pub if_num: __u16,
}

