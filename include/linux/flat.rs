//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/flat.h
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
// Copyright (C) 2002-2003  David McCullough <davidm@snapgear.com>
// Copyright (C) 1998       Kenneth Albanowski <kjahds@kjahds.com>
// The Silver Hammer Group, Ltd.
//
// This file provides the definitions and structures needed to
// support uClinux flat-format executables.
//
pub const FLAT_VERSION: c_uint = 0x00000004L;
//
// To make everything easier to port and manage cross platform
// development,  all fields are in network byte order.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flat_hdr {
    pub magic: [c_char; 4],
    pub /: *mut *mut __be32 rev; / version (as above),
    pub instruction: *mut *mut __be32 entry; / Offset of first executable,
    pub of: *mut *mut __be32 data_start; / Offset of data segment from beginning,
    pub beginning: *mut *mut __be32 data_end; / Offset of end of data segment from,
    pub beginning: *mut *mut __be32 bss_end; / Offset of end of bss segment from,
// (It is assumed that data_end through bss_end forms the bss segment.)
    pub /: *mut *mut __be32 stack_size; / Size of stack, in bytes,
    pub of: *mut *mut __be32 reloc_start; / Offset of relocation records from beginning,
    pub /: *mut *mut __be32 reloc_count; / Number of relocation records,
    pub flags: __be32,
    pub /: *mut *mut __be32 build_date; / When the program/library was built,
    pub /: *mut *mut __u32 filler[5]; / Reservered, set to zero,
}

pub const FLAT_FLAG_RAM: c_uint = 0x0001 /* load program entirely into RAM */;
pub const FLAT_FLAG_GOTPIC: c_uint = 0x0002 /* program is PIC with GOT */;
pub const FLAT_FLAG_GZIP: c_uint = 0x0004 /* all but the header is compressed */;
pub const FLAT_FLAG_GZDATA: c_uint = 0x0008 /* only data/relocs are compressed (for XIP) */;
pub const FLAT_FLAG_KTRACE: c_uint = 0x0010 /* output useful kernel trace for debugging */;
//
// While it would be nice to keep this header clean,  users of older
// tools still need this support in the kernel.  So this section is
// purely for compatibility with old tool chains.
//
// DO NOT make changes or enhancements to the old format please,  just work
// with the format above,  except to fix bugs with old format support.
//
pub const OLD_FLAT_VERSION: c_uint = 0x00000002L;
pub const OLD_FLAT_RELOC_TYPE_TEXT: c_int = 0;
pub const OLD_FLAT_RELOC_TYPE_DATA: c_int = 1;
pub const OLD_FLAT_RELOC_TYPE_BSS: c_int = 2;

