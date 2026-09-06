//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_labpc_regs.h
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
// ni_labpc register definitions.
//
// Register map (all registers are 8-bit)
//
pub const STAT1_REG: c_uint = 0x00	/* R: Status 1 reg */;

pub const CMD1_REG: c_uint = 0x00	/* W: Command 1 reg */;

pub const CMD2_REG: c_uint = 0x01	/* W: Command 2 reg */;

pub const CMD3_REG: c_uint = 0x02	/* W: Command 3 reg */;

pub const ADC_START_CONVERT_REG: c_uint = 0x03	/* W: Start Convert reg */;

pub const ADC_FIFO_CLEAR_REG: c_uint = 0x08	/* W: A/D FIFO Clear reg */;
pub const ADC_FIFO_REG: c_uint = 0x0a	/* R: A/D FIFO reg */;
pub const DMATC_CLEAR_REG: c_uint = 0x0a	/* W: DMA Interrupt Clear reg */;
pub const TIMER_CLEAR_REG: c_uint = 0x0c	/* W: Timer Interrupt Clear reg */;
pub const CMD6_REG: c_uint = 0x0e	/* W: Command 6 reg */;

pub const CMD4_REG: c_uint = 0x0f	/* W: Command 3 reg */;

pub const DIO_BASE_REG: c_uint = 0x10	/* R/W: 8255 DIO base reg */;
pub const COUNTER_A_BASE_REG: c_uint = 0x14	/* R/W: 8253 Counter A base reg */;
pub const COUNTER_B_BASE_REG: c_uint = 0x18	/* R/W: 8253 Counter B base reg */;
pub const CMD5_REG: c_uint = 0x1c	/* W: Command 5 reg */;

pub const STAT2_REG: c_uint = 0x1d	/* R: Status 2 reg */;

pub const INTERVAL_COUNT_REG: c_uint = 0x1e	/* W: Interval Counter Data reg */;
pub const INTERVAL_STROBE_REG: c_uint = 0x1f	/* W: Interval Counter Strobe reg */;
