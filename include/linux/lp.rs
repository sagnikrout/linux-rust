//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lp.h
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
// usr/include/linux/lp.h c.1991-1992 James Wiegand
// many modifications copyright (C) 1992 Michael K. Johnson
// Interrupt support added 1993 Nigel Gamble
// Removed 8255 status defines from inside __KERNEL__ Marcelo Tosatti
//

// Magic numbers for defining port-device mappings

// PARPORT_IRQ_NONE means polled

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp_stats {
    pub chars: c_ulong,
    pub sleeps: c_ulong,
    pub maxrun: c_uint,
    pub maxwait: c_uint,
    pub meanwait: c_uint,
    pub mdev: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lp_struct {
    pub dev: *mut pardevice,
    pub flags: c_ulong,
    pub chars: c_uint,
    pub time: c_uint,
    pub wait: c_uint,
    pub lp_buffer: *mut c_char,

    pub lastcall: c_uint,
    pub runchars: c_uint,
    pub stats: lp_stats,

    pub waitq: wait_queue_head_t,
    pub last_error: c_uint,
    pub port_mutex: mutex,
    pub dataq: wait_queue_head_t,
    pub timeout: c_long,
    pub best_mode: c_uint,
    pub current_mode: c_uint,
    pub bits: c_ulong,
}

//
// The following constants describe the various signals of the printer port
// hardware.  Note that the hardware inverts some signals and that some
// signals are active low.  An example is LP_STROBE, which must be programmed
// with 1 for being active and 0 for being inactive, because the strobe signal
// gets inverted, but it is also active low.
//
// defines for 8255 control port
// base + 2
// accessed with LP_C(minor)
//
pub const LP_PINTEN: c_uint = 0x10  /* high to read data in or-ed with data out */;
pub const LP_PSELECP: c_uint = 0x08  /* inverted output, active low */;
pub const LP_PINITP: c_uint = 0x04  /* unchanged output, active low */;
pub const LP_PAUTOLF: c_uint = 0x02  /* inverted output, active low */;
pub const LP_PSTROBE: c_uint = 0x01  /* short high output on raising edge */;
//
// the value written to ports to test existence. PC-style ports will
// return the value written. AT-style ports will return 0. so why not
// make them the same ?
//
pub const LP_DUMMY: c_uint = 0x00;
//
// This is the port delay time, in microseconds.
// It is used only in the lp_init() and lp_reset() routine.
//
pub const LP_DELAY: c_int = 50;
