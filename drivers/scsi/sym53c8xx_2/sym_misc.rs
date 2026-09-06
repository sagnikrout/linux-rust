//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/sym53c8xx_2/sym_misc.h
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
// Device driver for the SYMBIOS/LSILOGIC 53C8XX and 53C1010 family
// of PCI-SCSI IO processors.
//
// Copyright (C) 1999-2001  Gerard Roudier <groudier@free.fr>
//
// This driver is derived from the Linux sym53c8xx driver.
// Copyright (C) 1998-2000  Gerard Roudier
//
// The sym53c8xx driver is derived from the ncr53c8xx driver that had been
// a port of the FreeBSD ncr driver to Linux-1.2.13.
//
// The original ncr driver has been written for 386bsd and FreeBSD by
// Wolfgang Stanglmeier        <wolf@cologne.de>
// Stefan Esser                <se@mi.Uni-Koeln.de>
// Copyright (C) 1994  Wolfgang Stanglmeier
//
// Other major contributions:
//
// NVRAM detection and reading.
// Copyright (C) 1997 Richard Waltham <dormouse@farsrobt.demon.co.uk>
//
// -----------------------------------------------------------------------------
//
// A la VMS/CAM-3 queue management.
//

//
// This one may be useful.
//

//
// FreeBSD does not offer our kind of queue in the CAM CCB.
// So, we have to cast.
//

//
// Simple bitmap operations.
//

//
// The below round up/down macros are to be used with a constant
// as argument (sizeof(...) for example), for the compiler to
// optimize the whole thing.
//

//
// Round up logarithm to base 2 of a 16 bit constant.
//

