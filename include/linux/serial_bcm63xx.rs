//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serial_bcm63xx.h
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
// UART Control Register
pub const UART_CTL_REG: c_uint = 0x0;
pub const UART_CTL_RXTMOUTCNT_SHIFT: c_int = 0;

pub const UART_CTL_RSTTXDN_SHIFT: c_int = 5;

pub const UART_CTL_RSTRXFIFO_SHIFT: c_int = 6;

pub const UART_CTL_RSTTXFIFO_SHIFT: c_int = 7;

pub const UART_CTL_STOPBITS_SHIFT: c_int = 8;

pub const UART_CTL_BITSPERSYM_SHIFT: c_int = 12;

pub const UART_CTL_XMITBRK_SHIFT: c_int = 14;

pub const UART_CTL_RSVD_SHIFT: c_int = 15;

pub const UART_CTL_RXPAREVEN_SHIFT: c_int = 16;

pub const UART_CTL_RXPAREN_SHIFT: c_int = 17;

pub const UART_CTL_TXPAREVEN_SHIFT: c_int = 18;

pub const UART_CTL_TXPAREN_SHIFT: c_int = 18;

pub const UART_CTL_LOOPBACK_SHIFT: c_int = 20;

pub const UART_CTL_RXEN_SHIFT: c_int = 21;

pub const UART_CTL_TXEN_SHIFT: c_int = 22;

pub const UART_CTL_BRGEN_SHIFT: c_int = 23;

// UART Baudword register
pub const UART_BAUD_REG: c_uint = 0x4;
// UART Misc Control register
pub const UART_MCTL_REG: c_uint = 0x8;
pub const UART_MCTL_DTR_SHIFT: c_int = 0;

pub const UART_MCTL_RTS_SHIFT: c_int = 1;

pub const UART_MCTL_RXFIFOTHRESH_SHIFT: c_int = 8;

pub const UART_MCTL_TXFIFOTHRESH_SHIFT: c_int = 12;

pub const UART_MCTL_RXFIFOFILL_SHIFT: c_int = 16;

pub const UART_MCTL_TXFIFOFILL_SHIFT: c_int = 24;

// UART External Input Configuration register
pub const UART_EXTINP_REG: c_uint = 0xc;
pub const UART_EXTINP_RI_SHIFT: c_int = 0;

pub const UART_EXTINP_CTS_SHIFT: c_int = 1;

pub const UART_EXTINP_DCD_SHIFT: c_int = 2;

pub const UART_EXTINP_DSR_SHIFT: c_int = 3;

pub const UART_EXTINP_IR_RI: c_int = 0;
pub const UART_EXTINP_IR_CTS: c_int = 1;
pub const UART_EXTINP_IR_DCD: c_int = 2;
pub const UART_EXTINP_IR_DSR: c_int = 3;
pub const UART_EXTINP_RI_NOSENSE_SHIFT: c_int = 16;

pub const UART_EXTINP_CTS_NOSENSE_SHIFT: c_int = 17;

pub const UART_EXTINP_DCD_NOSENSE_SHIFT: c_int = 18;

pub const UART_EXTINP_DSR_NOSENSE_SHIFT: c_int = 19;

// UART Interrupt register
pub const UART_IR_REG: c_uint = 0x10;

pub const UART_IR_EXTIP: c_int = 0;
pub const UART_IR_TXUNDER: c_int = 1;
pub const UART_IR_TXOVER: c_int = 2;
pub const UART_IR_TXTRESH: c_int = 3;
pub const UART_IR_TXRDLATCH: c_int = 4;
pub const UART_IR_TXEMPTY: c_int = 5;
pub const UART_IR_RXUNDER: c_int = 6;
pub const UART_IR_RXOVER: c_int = 7;
pub const UART_IR_RXTIMEOUT: c_int = 8;
pub const UART_IR_RXFULL: c_int = 9;
pub const UART_IR_RXTHRESH: c_int = 10;
pub const UART_IR_RXNOTEMPTY: c_int = 11;
pub const UART_IR_RXFRAMEERR: c_int = 12;
pub const UART_IR_RXPARERR: c_int = 13;
pub const UART_IR_RXBRK: c_int = 14;
pub const UART_IR_TXDONE: c_int = 15;
// UART Fifo register
pub const UART_FIFO_REG: c_uint = 0x14;
pub const UART_FIFO_VALID_SHIFT: c_int = 0;
pub const UART_FIFO_VALID_MASK: c_uint = 0xff;
pub const UART_FIFO_FRAMEERR_SHIFT: c_int = 8;

pub const UART_FIFO_PARERR_SHIFT: c_int = 9;

pub const UART_FIFO_BRKDET_SHIFT: c_int = 10;

