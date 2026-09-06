//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/keyspan_usa90msg.h
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
// defines for bits in lcr
pub const USA_DATABITS_5: c_uint = 0x00;
pub const USA_DATABITS_6: c_uint = 0x01;
pub const USA_DATABITS_7: c_uint = 0x02;
pub const USA_DATABITS_8: c_uint = 0x03;
pub const STOPBITS_5678_1: c_uint = 0x00	// 1 stop bit for all byte sizes;
pub const STOPBITS_5_1p5: c_uint = 0x04	// 1.5 stop bits for 5-bit byte;
pub const STOPBITS_678_2: c_uint = 0x04	// 2 stop bits for 6-8 bit byte;
pub const USA_PARITY_NONE: c_uint = 0x00;
pub const USA_PARITY_ODD: c_uint = 0x08;
pub const USA_PARITY_EVEN: c_uint = 0x18;
pub const PARITY_MARK_1: c_uint = 0x28   	// force parity MARK;
pub const PARITY_SPACE_0: c_uint = 0x38	// force parity SPACE;
pub const TXFLOW_CTS: c_uint = 0x04;
pub const TXFLOW_DSR: c_uint = 0x08;
pub const TXFLOW_XOFF: c_uint = 0x01;
pub const TXFLOW_XOFF_ANY: c_uint = 0x02;

pub const RXFLOW_XOFF: c_uint = 0x10;
pub const RXFLOW_RTS: c_uint = 0x20;
pub const RXFLOW_DTR: c_uint = 0x40;
pub const RXFLOW_DSR_SENSITIVITY: c_uint = 0x80;
pub const RXMODE_BYHAND: c_uint = 0x00;
pub const RXMODE_DMA: c_uint = 0x02;
pub const TXMODE_BYHAND: c_uint = 0x00;
pub const TXMODE_DMA: c_uint = 0x02;
// all things called "StatusMessage" are sent on the status endpoint
// bits in RX data message when STAT byte is included
pub const RXERROR_OVERRUN: c_uint = 0x02;
pub const RXERROR_PARITY: c_uint = 0x04;
pub const RXERROR_FRAMING: c_uint = 0x08;
pub const RXERROR_BREAK: c_uint = 0x10;
pub const PORTSTATE_ENABLED: c_uint = 0x80;
pub const PORTSTATE_TXFLUSH: c_uint = 0x01;
pub const PORTSTATE_TXBREAK: c_uint = 0x02;
pub const PORTSTATE_LOOPBACK: c_uint = 0x04;
// MSR bits
pub const USA_MSR_dCTS: c_uint = 0x01		// CTS has changed since last report;
pub const USA_MSR_dDSR: c_uint = 0x02;
pub const USA_MSR_dRI: c_uint = 0x04;
pub const USA_MSR_dDCD: c_uint = 0x08;
pub const USA_MSR_CTS: c_uint = 0x10	  	// current state of CTS;
pub const USA_MSR_DSR: c_uint = 0x20;
pub const USA_USA_MSR_RI: c_uint = 0x40;
pub const MSR_DCD: c_uint = 0x80;
// ie: the maximum length of an endpoint buffer
pub const MAX_DATA_LEN: c_int = 64;
