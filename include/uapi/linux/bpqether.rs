//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/bpqether.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Defines for the BPQETHER pseudo device driver
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpq_ethaddr {
    pub destination: [c_uchar; ETH_ALEN],
    pub accept: [c_uchar; ETH_ALEN],
}

//
// For SIOCSBPQETHOPT - this is compatible with PI2/PacketTwin card drivers,
// currently not implemented, though. If someone wants to hook a radio
// to his Ethernet card he may find this useful. ;-)
//
pub const SIOCGBPQETHPARAM: c_uint = 0x5000  /* get Level 1 parameters */;
pub const SIOCSBPQETHPARAM: c_uint = 0x5001  /* set */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpq_req {
    pub cmd: c_int,
    pub /: *mut *mut int speed; / unused,
    pub /: *mut *mut int clockmode; / unused,
    pub txdelay: c_int,
    pub /: *mut *mut unsigned char persist; / unused,
    pub /: *mut *mut int slotime; / unused,
    pub squeldelay: c_int,
    pub /: *mut *mut int dmachan; / unused,
    pub /: *mut *mut int irq; / unused,
}
