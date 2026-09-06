//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/nftl.h
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
// Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
//

// these info are used in ReplUnitTable
pub const BLOCK_NIL: c_uint = 0xffff /* last block of a chain */;
pub const BLOCK_FREE: c_uint = 0xfffe /* free block */;
pub const BLOCK_NOTEXPLORED: c_uint = 0xfffd /* non explored block, only used during mounting */;
pub const BLOCK_RESERVED: c_uint = 0xfffc /* bios block or bad block */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NFTLrecord {
    pub mbd: mtd_blktrans_dev,
    pub SpareMediaUnit: __u16 MediaUnit,,
    pub EraseSize: __u32,
    pub MediaHdr: NFTLMediaHeader,
    pub usecount: c_int,
    pub heads: c_uchar,
    pub sectors: c_uchar,
    pub cylinders: c_ushort,
    pub numvunits: __u16,
    pub /: *mut *mut __u16 lastEUN; / should be suppressed,
    pub numfreeEUNs: __u16,
    pub /: *mut *mut __u16 LastFreeEUN; / To speed up finding a free EUN,
    pub head,sect,cyl: c_int,
    pub /: *mut *mut *mut __u16 EUNtable; / [numvunits]: First EUN for each virtual unit,
    pub /: *mut *mut *mut __u16 ReplUnitTable; / [numEUNs]: ReplUnitNumber for each,
    pub /: *mut *mut unsigned int nb_blocks; / number of physical blocks,
    pub /: *mut *mut unsigned int nb_boot_blocks; / number of blocks used by the bios,
    pub instr: erase_info,
}

extern "C" {
    pub fn NFTL_mount(s: *mut NFTLrecord) -> c_int;
}
extern "C" {
    pub fn NFTL_formatblock(s: *mut NFTLrecord, block: c_int) -> c_int;
}

pub const NFTL_MAJOR: c_int = 93;

pub const MAX_NFTLS: c_int = 16;
pub const MAX_SECTORS_PER_UNIT: c_int = 64;
pub const NFTL_PARTN_BITS: c_int = 4;
