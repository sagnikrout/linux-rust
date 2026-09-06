//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/keyspan_usa26msg.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// USA26: 0=internal, other=external
// USA17: 0=internal, other=external/RI
// USA26: host requests TX tri-state be set
// USA17: host requests RTS output be set
// USA26: host requests HSKOA output be set
// USA17: host requests DTR output be set
// must be set any time internal baud rate is set;
// must not be set when external clocking is used
// note: in USA17, prescaler is applied whenever
// setClocking is requested
//
// USA26: 1=normal; 0=ignore external clock
// USA17: 1=use DSR flow control, 0=don't
//
// defines for bits in lcr
pub const USA_DATABITS_5: c_uint = 0x00;
pub const USA_DATABITS_6: c_uint = 0x01;
pub const USA_DATABITS_7: c_uint = 0x02;
pub const USA_DATABITS_8: c_uint = 0x03;
pub const STOPBITS_5678_1: c_uint = 0x00	// 1 stop bit for all byte sizes;
pub const STOPBITS_5_1p5: c_uint = 0x04	// 1.5 stop bits for 5-bit byte;
pub const STOPBITS_678_2: c_uint = 0x04	// 2 stop bits for 6/7/8-bit byte;
pub const USA_PARITY_NONE: c_uint = 0x00;
pub const USA_PARITY_ODD: c_uint = 0x08;
pub const USA_PARITY_EVEN: c_uint = 0x18;
pub const PARITY_1: c_uint = 0x28;
pub const PARITY_0: c_uint = 0x38;
// all things called "StatusMessage" are sent on the status endpoint
// USA17: reports CTS pin
// USA17: reports DCD pin
// bits in RX data message when STAT byte is included
pub const RXERROR_OVERRUN: c_uint = 0x02;
pub const RXERROR_PARITY: c_uint = 0x04;
pub const RXERROR_FRAMING: c_uint = 0x08;
pub const RXERROR_BREAK: c_uint = 0x10;
// ie: the maximum length of an EZUSB endpoint buffer
pub const MAX_DATA_LEN: c_int = 64;
// update status approx. 60 times a second (16.6666 ms)
pub const STATUS_UPDATE_INTERVAL: c_int = 16;
// status rationing tuning value (each port gets checked each n ms)
pub const STATUS_RATION: c_int = 10;
