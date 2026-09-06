//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/amba/kmi.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linux/include/asm-arm/hardware/amba_kmi.h
//
// Internal header file for AMBA KMI ports
//
// Copyright (C) 2000 Deep Blue Solutions Ltd.
//
// ---------------------------------------------------------------------------
// From ARM PrimeCell(tm) PS2 Keyboard/Mouse Interface (PL050) Technical
// Reference Manual - ARM DDI 0143B - see http://www.arm.com
// ---------------------------------------------------------------------------
//
// KMI control register:
// KMICR_TYPE       0 = PS2/AT mode, 1 = No line control bit mode
// KMICR_RXINTREN   1 = enable RX interrupts
// KMICR_TXINTREN   1 = enable TX interrupts
// KMICR_EN         1 = enable KMI
// KMICR_FD         1 = force KMI data low
// KMICR_FC         1 = force KMI clock low
//

//
// KMI status register:
// KMISTAT_TXEMPTY  1 = transmitter register empty
// KMISTAT_TXBUSY   1 = currently sending data
// KMISTAT_RXFULL   1 = receiver register ready to be read
// KMISTAT_RXBUSY   1 = currently receiving data
// KMISTAT_RXPARITY parity of last databyte received
// KMISTAT_IC       current level of KMI clock input
// KMISTAT_ID       current level of KMI data input
//

//
// KMI data register
//

//
// KMI clock divisor: to generate 8MHz internal clock
// div = (ref / 8MHz) - 1; 0 <= div <= 15
//

//
// KMI interrupt register:
// KMIIR_TXINTR     1 = transmit interrupt asserted
// KMIIR_RXINTR     1 = receive interrupt asserted
//

//
// The size of the KMI primecell
//

