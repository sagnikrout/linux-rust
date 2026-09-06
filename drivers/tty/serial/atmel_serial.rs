//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tty/serial/atmel_serial.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// include/linux/atmel_serial.h
//
// Copyright (C) 2005 Ivan Kokshaysky
// Copyright (C) SAN People
//
// USART registers.
// Based on AT91RM9200 datasheet revision E.
//

pub const ATMEL_US_CR: c_uint = 0x00	/* Control Register */;

pub const ATMEL_US_MR: c_uint = 0x04	/* Mode Register */;

pub const ATMEL_US_IER: c_uint = 0x08	/* Interrupt Enable Register */;

pub const ATMEL_US_IDR: c_uint = 0x0c	/* Interrupt Disable Register */;
pub const ATMEL_US_IMR: c_uint = 0x10	/* Interrupt Mask Register */;
pub const ATMEL_US_CSR: c_uint = 0x14	/* Channel Status Register */;
pub const ATMEL_US_RHR: c_uint = 0x18	/* Receiver Holding Register */;
pub const ATMEL_US_THR: c_uint = 0x1c	/* Transmitter Holding Register */;

pub const ATMEL_US_BRGR: c_uint = 0x20	/* Baud Rate Generator Register */;

pub const ATMEL_US_FP_MASK: c_uint = 0x7;
pub const ATMEL_US_RTOR: c_uint = 0x24	/* Receiver Time-out Register for USART */;
pub const ATMEL_UA_RTOR: c_uint = 0x28	/* Receiver Time-out Register for UART */;

pub const ATMEL_US_TTGR: c_uint = 0x28	/* Transmitter Timeguard Register */;

pub const ATMEL_US_FIDI: c_uint = 0x40	/* FI DI Ratio Register */;
pub const ATMEL_US_NER: c_uint = 0x44	/* Number of Errors Register */;
pub const ATMEL_US_IF: c_uint = 0x4c	/* IrDA Filter Register */;
pub const ATMEL_US_CMPR: c_uint = 0x90	/* Comparaison Register */;
pub const ATMEL_US_FMR: c_uint = 0xa0	/* FIFO Mode Register */;

pub const ATMEL_US_ONE_DATA: c_uint = 0x0;
pub const ATMEL_US_TWO_DATA: c_uint = 0x1;
pub const ATMEL_US_FOUR_DATA: c_uint = 0x2;

pub const ATMEL_US_FLR: c_uint = 0xa4	/* FIFO Level Register */;

pub const ATMEL_US_FIER: c_uint = 0xa8	/* FIFO Interrupt Enable Register */;
pub const ATMEL_US_FIDR: c_uint = 0xac	/* FIFO Interrupt Disable Register */;
pub const ATMEL_US_FIMR: c_uint = 0xb0	/* FIFO Interrupt Mask Register */;
pub const ATMEL_US_FESR: c_uint = 0xb4	/* FIFO Event Status Register */;

pub const ATMEL_US_NAME: c_uint = 0xf0	/* Ip Name */;
pub const ATMEL_US_VERSION: c_uint = 0xfc	/* Ip Version */;
