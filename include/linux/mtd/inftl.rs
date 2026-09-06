//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/inftl.h
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
// inftl.h -- defines to support the Inverse NAND Flash Translation Layer
//
// (C) Copyright 2002, Greg Ungerer (gerg@snapgear.com)
//

pub const INFTL_MAJOR: c_int = 96;

pub const INFTL_PARTN_BITS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct INFTLrecord {
    pub mbd: mtd_blktrans_dev,
    pub MediaUnit: __u16,
    pub EraseSize: __u32,
    pub MediaHdr: INFTLMediaHeader,
    pub usecount: c_int,
    pub heads: c_uchar,
    pub sectors: c_uchar,
    pub cylinders: c_ushort,
    pub numvunits: __u16,
    pub firstEUN: __u16,
    pub lastEUN: __u16,
    pub numfreeEUNs: __u16,
    pub /: *mut *mut __u16 LastFreeEUN; / To speed up finding a free EUN,
    pub head,sect,cyl: c_int,
    pub /: *mut *mut *mut __u16 PUtable; / Physical Unit Table,
    pub /: *mut *mut *mut __u16 VUtable; / Virtual Unit Table,
    pub /: *mut *mut unsigned int nb_blocks; / number of physical blocks,
    pub /: *mut *mut unsigned int nb_boot_blocks; / number of blocks used by the bios,
    pub instr: erase_info,
}

extern "C" {
    pub fn INFTL_mount(s: *mut INFTLrecord) -> c_int;
}
extern "C" {
    pub fn INFTL_formatblock(s: *mut INFTLrecord, block: c_int) -> c_int;
}
extern "C" {
    pub fn INFTL_dumptables(s: *mut INFTLrecord);
}
extern "C" {
    pub fn INFTL_dumpVUchains(s: *mut INFTLrecord);
}

