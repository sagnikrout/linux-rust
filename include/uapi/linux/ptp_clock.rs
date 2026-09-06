//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/ptp_clock.h
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
// PTP 1588 clock support - user space interface
//
// Copyright (C) 2010 OMICRON electronics GmbH
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

//
// Bits of the ptp_extts_request.flags field:
//

//
// flag fields valid for the new PTP_EXTTS_REQUEST2 ioctl.
//
// Note: PTP_STRICT_FLAGS is always enabled by the kernel for
// PTP_EXTTS_REQUEST2 regardless of whether it is set by userspace.
//

//
// flag fields valid for the original PTP_EXTTS_REQUEST ioctl.
// DO NOT ADD NEW FLAGS HERE.
//

//
// flag fields valid for the ptp_extts_event report.
//

//
// Bits of the ptp_perout_request.flags field:
//

//
// flag fields valid for the new PTP_PEROUT_REQUEST2 ioctl.
//

//
// No flags are valid for the original PTP_PEROUT_REQUEST ioctl
//

//
// struct ptp_clock_time - represents a time value
//
// The sign of the seconds field applies to the whole value. The
// nanoseconds field is always unsigned. The reserved field is
// included for sub-nanosecond resolution, should the demand for
// this ever appear.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_time {
    pub /: *mut *mut __s64 sec; / seconds,
    pub /: *mut *mut __u32 nsec; / nanoseconds,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_caps {
    pub /: *mut *mut int max_adj; / Maximum frequency adjustment in parts per billon.,
    pub /: *mut *mut int n_alarm; / Number of programmable alarms.,
    pub /: *mut *mut int n_ext_ts; / Number of external time stamp channels.,
    pub /: *mut *mut int n_per_out; / Number of programmable periodic signals.,
    pub /: *mut *mut int pps; / Whether the clock supports a PPS callback.,
    pub /: *mut *mut int n_pins; / Number of input/output pins.,
// Whether the clock supports precise system-device cross timestamps
    pub cross_timestamping: c_int,
// Whether the clock supports adjust phase
    pub adjust_phase: c_int,
    pub /: *mut *mut int max_phase_adj; / Maximum phase adjustment in nanoseconds.,
    pub /: *mut *mut int rsv[11]; / Reserved for future use.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_extts_request {
    pub /: *mut *mut unsigned int index; / Which channel to configure.,
    pub /: *mut *mut unsigned int flags; / Bit field for PTP_xxx flags.,
    pub /: *mut *mut unsigned int rsv[2]; / Reserved for future use.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_perout_request {
//
// Absolute start time.
// Valid only if (flags & PTP_PEROUT_PHASE) is unset.
//
    pub start: ptp_clock_time,
//
// Phase offset. The signal should start toggling at an
// unspecified integer multiple of the period, plus this value.
// The start time should be "as soon as possible".
// Valid only if (flags & PTP_PEROUT_PHASE) is set.
//
    pub phase: ptp_clock_time,
}

//
// The "on" time of the signal.
// Must be lower than the period.
// Valid only if (flags & PTP_PEROUT_DUTY_CYCLE) is set.
//
// Reserved for future use.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_sys_offset {
    pub /: *mut *mut unsigned int n_samples; / Desired number of measurements.,
    pub /: *mut *mut unsigned int rsv[3]; / Reserved for future use.,
//
// Array of interleaved system/phc time stamps. The kernel
// will provide 2*n_samples + 1 time stamps, with the last
// one as a system time stamp.
//
    pub 1]: *mut *mut ptp_clock_time ts[2  PTP_MAX_SAMPLES +,
}

//
// ptp_sys_offset_extended - data structure for IOCTL operation
// PTP_SYS_OFFSET_EXTENDED
//
// @n_samples:	Desired number of measurements.
// @clockid:	clockid of a clock-base used for pre/post timestamps.
// @rsv:	Reserved for future use.
// @ts:		Array of samples in the form [pre-TS, PHC, post-TS]. The
// kernel provides @n_samples.
//
// Starting from kernel 6.12 and onwards, the first word of the reserved-field
// is used for @clockid. That's backward compatible since previous kernel
// expect all three reserved words (@rsv[3]) to be 0 while the clockid (first
// word in the new structure) for CLOCK_REALTIME is '0'.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_sys_offset_extended {
    pub n_samples: c_uint,
    pub clockid: __kernel_clockid_t,
    pub rsv: [c_uint; 2],
    pub ts: [ptp_clock_time; PTP_MAX_SAMPLES][3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_sys_offset_precise {
    pub device: ptp_clock_time,
    pub sys_realtime: ptp_clock_time,
    pub sys_monoraw: ptp_clock_time,
    pub /: *mut *mut unsigned int rsv[4]; / Reserved for future use.,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_pin_function {
    PTP_PF_NONE,
    PTP_PF_EXTTS,
    PTP_PF_PEROUT,
    PTP_PF_PHYSYNC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_pin_desc {
//
// Hardware specific human readable pin name. This field is
// set by the kernel during the PTP_PIN_GETFUNC ioctl and is
// ignored for the PTP_PIN_SETFUNC ioctl.
//
    pub name: [c_char; 64],
//
// Pin index in the range of zero to ptp_clock_caps.n_pins - 1.
//
    pub index: c_uint,
//
// Which of the PTP_PF_xxx functions to use on this pin.
//
    pub func: c_uint,
//
// The specific channel to use for this function.
// This corresponds to the 'index' field of the
// PTP_EXTTS_REQUEST and PTP_PEROUT_REQUEST ioctls.
//
    pub chan: c_uint,
//
// Reserved for future use.
//
    pub rsv: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_extts_event {
    pub /: *mut *mut ptp_clock_time t; / Time event occurred.,
    pub /: *mut *mut unsigned int index; / Which channel produced the event.,
    pub /: *mut *mut unsigned int flags; / Event type.,
    pub /: *mut *mut unsigned int rsv[2]; / Reserved for future use.,
}
