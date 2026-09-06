//! Automatically rewritten from C Header to Rust Module
//! Source: fs/jfs/jfs_types.h
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
// Copyright (C) International Business Machines Corp., 2000-2004
//
// jfs_types.h:
//
// basic type/utility definitions
//
// note: this header file must be the 1st include file
// of JFS include list in all JFS .c file.
//

//
// transaction and lock id's
//
// Don't change these without carefully considering the impact on the
// size and alignment of all of the linelock variants
//
pub type tid_t = u16;
pub type lid_t = u16;
//
// Almost identical to Linux's timespec, but not quite
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timestruc_t {
    pub tv_sec: __le32,
    pub tv_nsec: __le32,
}

//
// handy
//
pub const LEFTMOSTONE: c_uint = 0x80000000;
pub const HIGHORDER: c_uint = 0x80000000u	/* high order bit on	*/;
pub const ONES: c_uint = 0xffffffffu	/* all bit on		*/;
//
// physical xd (pxd)
//
// The leftmost 24 bits of len_addr are the extent length.
// The rightmost 8 bits of len_addr are the most signficant bits of
// the extent address
//
// xd_t field construction
// xd_t field extraction
pub const MAXTREEHEIGHT: c_int = 8;
// pxd list
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxdlist {
    pub maxnpxd: i16,
    pub npxd: i16,
    pub pxd: [pxd_t; MAXTREEHEIGHT],
}

//
// data extent descriptor (dxd)
//
// dxd_t flags
pub const DXD_INDEX: c_uint = 0x80	/* B+-tree index */;
pub const DXD_INLINE: c_uint = 0x40	/* in-line data extent */;
pub const DXD_EXTENT: c_uint = 0x20	/* out-of-line single extent */;
pub const DXD_FILE: c_uint = 0x10	/* out-of-line file (inode) */;
pub const DXD_CORRUPT: c_uint = 0x08	/* Inconsistency detected */;
// dxd_t field construction
//

//
// directory entry argument
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct component_name {
    pub namlen: c_int,
    pub name: *mut wchar_t,
}

//
// DASD limit information - stored in directory inode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd {
    pub /: *mut *mut u8 thresh; / Alert Threshold (in percent),
    pub /: *mut *mut u8 delta; / Alert Threshold delta (in percent),
    pub rsrvd1: u8,
    pub /: *mut *mut u8 limit_hi; / DASD limit (in logical blocks),
    pub /: *mut *mut __le32 limit_lo; / DASD limit (in logical blocks),
    pub rsrvd2: [u8; 3],
    pub /: *mut *mut u8 used_hi; / DASD usage (in logical blocks),
    pub /: *mut *mut __le32 used_lo; / DASD usage (in logical blocks),
}

