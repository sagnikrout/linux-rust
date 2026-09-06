//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/serial_s3c.h
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
// Internal header file for Samsung S3C2410 serial ports (UART0-2)
//
// Copyright (C) 2002 Shane Nay (shane@minirl.com)
//
// Additional defines, Copyright 2003 Simtec Electronics (linux@simtec.co.uk)
//
// Adapted from:
//
// Internal header file for MX1ADS serial ports (UART1 & 2)
//
// Copyright (C) 2002 Shane Nay (shane@minirl.com)
//

// S3C2440 FIFO trigger levels

// UFSTAT S3C2443 same as S3C2440

// S3C64XX interrupt registers.
pub const S3C64XX_UINTP: c_uint = 0x30;
pub const S3C64XX_UINTSP: c_uint = 0x34;
pub const S3C64XX_UINTM: c_uint = 0x38;

// Following are specific to S5PV210

// Default values for s5pv210 UCON and UFCON uart registers

pub const APPLE_S5L_UCON_RXTO_ENA: c_int = 9;
pub const APPLE_S5L_UCON_RXTO_LEGACY_ENA: c_int = 11;
pub const APPLE_S5L_UCON_RXTHRESH_ENA: c_int = 12;
pub const APPLE_S5L_UCON_TXTHRESH_ENA: c_int = 13;

// configuration structure for per-machine configurations for the
// serial port
//
// the pointer is setup by the machine specific initialisation from the
// arch/arm/mach-s3c/ directory.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s3c2410_uartcfg {
    pub /: *mut *mut unsigned char hwport; / hardware port number,
    pub unused: c_uchar,
    pub flags: c_ushort,
    pub /: *mut *mut upf_t uart_flags; / default uart flags,
    pub clk_sel: c_uint,
    pub has_fracval: c_uint,
    pub /: *mut *mut unsigned long ucon; / value of ucon for port,
    pub /: *mut *mut unsigned long ulcon; / value of ulcon for port,
    pub /: *mut *mut unsigned long ufcon; / value of ufcon for port,
}

