//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/hvm/ioreq.h
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


// SPDX-License-Identifier: MIT
//
// ioreq.h: I/O request definitions for device models
// Copyright (c) 2004, Intel Corporation.
//
pub const IOREQ_READ: c_int = 1;
pub const IOREQ_WRITE: c_int = 0;
pub const STATE_IOREQ_NONE: c_int = 0;
pub const STATE_IOREQ_READY: c_int = 1;
pub const STATE_IOREQ_INPROCESS: c_int = 2;
pub const STATE_IORESP_READY: c_int = 3;

pub const IOREQ_TYPE_PCI_CONFIG: c_int = 2;
pub const IOREQ_TYPE_TIMEOFFSET: c_int = 7;

//
// VMExit dispatcher should cooperate with instruction decoder to
// prepare this structure and notify service OS and DM by sending
// virq.
//
// For I/O type IOREQ_TYPE_PCI_CONFIG, the physical address is formatted
// as follows:
//
// 63....48|47..40|39..35|34..32|31........0
// SEGMENT |BUS   |DEV   |FN    |OFFSET
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioreq {
    pub /: *mut *mut uint64_t addr; / physical address,
    pub /: *mut *mut uint64_t data; / data (or paddr of data),
    pub /: *mut *mut uint32_t count; / for rep prefixes,
    pub /: *mut *mut uint32_t size; / size in bytes,
    pub /: *mut *mut uint32_t vp_eport; / evtchn for notifications to/from device model,
    pub _pad0: u16,
    pub state:4: u8,
    pub paddr: *mut *mut uint8_t data_is_ptr:1; / if 1, data above is the guest,
// of the real data to use.
    pub /: *mut *mut uint8_t dir:1; / 1=read, 0=write,
    pub df:1: u8,
    pub _pad1:1: u8,
    pub /: *mut *mut uint8_t type; / I/O type,
}
