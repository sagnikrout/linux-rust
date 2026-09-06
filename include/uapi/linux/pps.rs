//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pps.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// PPS API header
//
// Copyright (C) 2005-2009   Rodolfo Giometti <giometti@linux.it>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//

// Implementation note: the logical states ``assert'' and ``clear''
// are implemented in terms of the chip register, i.e. ``assert''
// means the bit is set.
//
// 3.2 New data structures
//
pub const PPS_API_VERS_1: c_int = 1;

pub const PPS_MAX_NAME_LEN: c_int = 32;
// 32-bit vs. 64-bit compatibility.
//
// 0n i386, the alignment of a uint64_t is only 4 bytes, while on most other
// architectures it's 8 bytes. On i386, there will be no padding between the
// two consecutive 'struct pps_ktime' members of struct pps_kinfo and struct
// pps_kparams. But on most platforms there will be padding to ensure correct
// alignment.
//
// The simple fix is probably to add an explicit padding.
// [David Woodhouse]
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_ktime {
    pub sec: __s64,
    pub nsec: __s32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_ktime_compat {
    pub sec: __s64,
    pub nsec: __s32,
    pub flags: __u32,
// C attribute field omitted

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_kinfo {
    pub /: *mut *mut __u32 assert_sequence; / seq. num. of assert event,
    pub /: *mut *mut __u32 clear_sequence; / seq. num. of clear event,
    pub /: *mut *mut pps_ktime assert_tu; / time of assert event,
    pub /: *mut *mut pps_ktime clear_tu; / time of clear event,
    pub /: *mut *mut int current_mode; / current mode bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_kinfo_compat {
    pub /: *mut *mut __u32 assert_sequence; / seq. num. of assert event,
    pub /: *mut *mut __u32 clear_sequence; / seq. num. of clear event,
    pub /: *mut *mut pps_ktime_compat assert_tu; / time of assert event,
    pub /: *mut *mut pps_ktime_compat clear_tu; / time of clear event,
    pub /: *mut *mut int current_mode; / current mode bits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_kparams {
    pub /: *mut *mut int api_version; / API version #,
    pub /: *mut *mut int mode; / mode bits,
    pub /: *mut *mut pps_ktime assert_off_tu; / offset compensation for assert,
    pub /: *mut *mut pps_ktime clear_off_tu; / offset compensation for clear,
}

//
// 3.3 Mode bit definitions
//
// Device/implementation parameters
pub const PPS_CAPTUREASSERT: c_uint = 0x01	/* capture assert events */;
pub const PPS_CAPTURECLEAR: c_uint = 0x02	/* capture clear events */;
pub const PPS_CAPTUREBOTH: c_uint = 0x03	/* capture assert and clear events */;
pub const PPS_OFFSETASSERT: c_uint = 0x10	/* apply compensation for assert event */;
pub const PPS_OFFSETCLEAR: c_uint = 0x20	/* apply compensation for clear event */;
pub const PPS_CANWAIT: c_uint = 0x100	/* can we wait for an event? */;
pub const PPS_CANPOLL: c_uint = 0x200	/* bit reserved for future use */;
// Kernel actions
pub const PPS_ECHOASSERT: c_uint = 0x40	/* feed back assert event to output */;
pub const PPS_ECHOCLEAR: c_uint = 0x80	/* feed back clear event to output */;
// Timestamp formats
pub const PPS_TSFMT_TSPEC: c_uint = 0x1000	/* select timespec format */;
pub const PPS_TSFMT_NTPFP: c_uint = 0x2000	/* select NTP format */;
//
// 3.4.4 New functions: disciplining the kernel timebase
//
// Kernel consumers

//
// Here begins the implementation-specific part!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_fdata {
    pub info: pps_kinfo,
    pub timeout: pps_ktime,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_fdata_compat {
    pub info: pps_kinfo_compat,
    pub timeout: pps_ktime_compat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_bind_args {
    pub /: *mut *mut int tsformat; / format of time stamps,
    pub /: *mut *mut int edge; / selected event type,
    pub /: *mut *mut int consumer; / selected kernel consumer,
}

