//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/renesas-xspi-if-regs.h
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
// RZ xSPI Interface Registers Definitions
//
// Copyright (C) 2025 Renesas Electronics Corporation
//

// xSPI Wrapper Configuration Register
pub const XSPI_WRAPCFG: c_uint = 0x0000;
// xSPI Bridge Configuration Register
pub const XSPI_BMCFG: c_uint = 0x0008;

// xSPI Command Map Configuration Register 0 CS0
pub const XSPI_CMCFG0CS0: c_uint = 0x0010;

// xSPI Command Map Configuration Register 1 CS0
pub const XSPI_CMCFG1CS0: c_uint = 0x0014;

// xSPI Command Map Configuration Register 2 CS0
pub const XSPI_CMCFG2CS0: c_uint = 0x0018;

// xSPI Link I/O Configuration Register CS0
pub const XSPI_LIOCFGCS0: c_uint = 0x0050;

// xSPI Bridge Map Control Register 0
pub const XSPI_BMCTL0: c_uint = 0x0060;

// xSPI Bridge Map Control Register 1
pub const XSPI_BMCTL1: c_uint = 0x0064;

// xSPI Command Manual Control Register 0
pub const XSPI_CDCTL0: c_uint = 0x0070;

// xSPI Command Manual Type Buf
pub const XSPI_CDTBUF0: c_uint = 0x0080;

// xSPI Command Manual Address Buff
pub const XSPI_CDABUF0: c_uint = 0x0084;
// xSPI Command Manual Data 0 Buf
pub const XSPI_CDD0BUF0: c_uint = 0x0088;
// xSPI Command Manual Data 1 Buf
pub const XSPI_CDD1BUF0: c_uint = 0x008c;
// xSPI Command Calibration Control Register 0 CS0
pub const XSPI_CCCTL0CS0: c_uint = 0x0130;

// xSPI Interrupt Status Register
pub const XSPI_INTS: c_uint = 0x0190;

// xSPI Interrupt Clear Register
pub const XSPI_INTC: c_uint = 0x0194;

// xSPI Interrupt Enable Register
pub const XSPI_INTE: c_uint = 0x0198;

// Maximum data size of MWRSIZE
pub const MWRSIZE_MAX: c_int = 64;
// xSPI Protocol mode
pub const PROTO_1S_2S_2S: c_uint = 0x48;
pub const PROTO_2S_2S_2S: c_uint = 0x49;
pub const PROTO_1S_4S_4S: c_uint = 0x090;
pub const PROTO_4S_4S_4S: c_uint = 0x092;
