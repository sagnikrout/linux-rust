//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/ata/sata_promise.h
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
// sata_promise.h - Promise SATA common definitions and inline funcs
//
// Copyright 2003-2004 Red Hat, Inc.
//
// libata documentation is available via 'make {ps|pdf}docs',
// as Documentation/driver-api/libata.rst
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pdc_packet_bits {
    PDC_PKT_READ		= (1 << 2),
    PDC_PKT_NODATA		= (1 << 3),

    PDC_PKT_SIZEMASK	= (1 << 7) | (1 << 6) | (1 << 5),
    PDC_PKT_CLEAR_BSY	= (1 << 4),
    PDC_PKT_WAIT_DRDY	= (1 << 3) | (1 << 4),
    PDC_LAST_REG		= (1 << 3),

    PDC_REG_DEVCTL		= (1 << 3) | (1 << 2) | (1 << 1),
}

// set control bits (byte 0), zero delay seq id (byte 3),
// and seq id (byte 2)
//
// select device
// device control register
// and finally the command itself; also includes end-of-pkt marker
// the "(1 << 5)" should be read "(count << 5)"
// ATA command block registers
// the "(2 << 5)" should be read "(count << 5)"
// ATA command block registers
