//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ingenic-tcu.h
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
// Header file for the Ingenic JZ47xx TCU driver
//

pub const TCU_REG_WDT_TDR: c_uint = 0x00;
pub const TCU_REG_WDT_TCER: c_uint = 0x04;
pub const TCU_REG_WDT_TCNT: c_uint = 0x08;
pub const TCU_REG_WDT_TCSR: c_uint = 0x0c;
pub const TCU_REG_TER: c_uint = 0x10;
pub const TCU_REG_TESR: c_uint = 0x14;
pub const TCU_REG_TECR: c_uint = 0x18;
pub const TCU_REG_TSR: c_uint = 0x1c;
pub const TCU_REG_TFR: c_uint = 0x20;
pub const TCU_REG_TFSR: c_uint = 0x24;
pub const TCU_REG_TFCR: c_uint = 0x28;
pub const TCU_REG_TSSR: c_uint = 0x2c;
pub const TCU_REG_TMR: c_uint = 0x30;
pub const TCU_REG_TMSR: c_uint = 0x34;
pub const TCU_REG_TMCR: c_uint = 0x38;
pub const TCU_REG_TSCR: c_uint = 0x3c;
pub const TCU_REG_TDFR0: c_uint = 0x40;
pub const TCU_REG_TDHR0: c_uint = 0x44;
pub const TCU_REG_TCNT0: c_uint = 0x48;
pub const TCU_REG_TCSR0: c_uint = 0x4c;
pub const TCU_REG_OST_DR: c_uint = 0xe0;
pub const TCU_REG_OST_CNTL: c_uint = 0xe4;
pub const TCU_REG_OST_CNTH: c_uint = 0xe8;
pub const TCU_REG_OST_TCSR: c_uint = 0xec;
pub const TCU_REG_TSTR: c_uint = 0xf0;
pub const TCU_REG_TSTSR: c_uint = 0xf4;
pub const TCU_REG_TSTCR: c_uint = 0xf8;
pub const TCU_REG_OST_CNTHBUF: c_uint = 0xfc;
pub const TCU_TCSR_RESERVED_BITS: c_uint = 0x3f;
pub const TCU_TCSR_PARENT_CLOCK_MASK: c_uint = 0x07;
pub const TCU_TCSR_PRESCALE_LSB: c_int = 3;
pub const TCU_TCSR_PRESCALE_MASK: c_uint = 0x38;

pub const TCU_CHANNEL_STRIDE: c_uint = 0x10;

