//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/gvp11.h
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
// $Id: gvp11.h,v 1.4 1997/01/19 23:07:12 davem Exp $
//
// Header file for the GVP Series II SCSI controller for Linux
//
// Written and (C) 1993, Ralf Baechle, see gvp11.c for more info
// based on a2091.h (C) 1993 by Hamish Macdonald
//

pub const CMD_PER_LUN: c_int = 2;

pub const CAN_QUEUE: c_int = 16;

//
// if the transfer address ANDed with this results in a non-zero
// result, then we can't use DMA.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gvp11_scsiregs {
    pub pad1: [c_uchar; 64],
    pub CNTR: volatile unsigned short,
    pub pad2: [c_uchar; 31],
    pub SASR: volatile unsigned char,
    pub pad3: c_uchar,
    pub SCMD: volatile unsigned char,
    pub pad4: [c_uchar; 4],
    pub BANK: volatile unsigned short,
    pub pad5: [c_uchar; 6],
    pub ACR: volatile unsigned long,
    pub /: *mut *mut volatile unsigned short secret1; / store 0 here,
    pub ST_DMA: volatile unsigned short,
    pub SP_DMA: volatile unsigned short,
    pub /: *mut *mut volatile unsigned short secret2; / store 1 here,
    pub /: *mut *mut volatile unsigned short secret3; / store 15 here,
}

// bits in CNTR

