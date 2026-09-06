//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/mtd/nftl-user.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

// Block Control Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftl_bci {
    pub ECCSig: [c_uchar; 6],
    pub Status: __u8,
    pub Status1: __u8,
// Unit Control Information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftl_uci0 {
    pub VirtUnitNum: __u16,
    pub ReplUnitNum: __u16,
    pub SpareVirtUnitNum: __u16,
    pub SpareReplUnitNum: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftl_uci1 {
    pub WearInfo: __u32,
    pub EraseMark: __u16,
    pub EraseMark1: __u16,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftl_uci2 {
    pub FoldMark: __u16,
    pub FoldMark1: __u16,
    pub unused: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub union nftl_uci {
    pub a: nftl_uci0,
    pub b: nftl_uci1,
    pub c: nftl_uci2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nftl_oob {
    pub b: nftl_bci,
    pub u: nftl_uci,
}

// NFTL Media Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NFTLMediaHeader {
    pub DataOrgID: [c_char; 6],
    pub NumEraseUnits: __u16,
    pub FirstPhysicalEUN: __u16,
    pub FormattedSize: __u32,
    pub UnitSizeFactor: c_uchar,
    pub __attribute__((packed)): },

pub const ERASE_MARK: c_uint = 0x3c69;
pub const SECTOR_FREE: c_uint = 0xff;
pub const SECTOR_USED: c_uint = 0x55;
pub const SECTOR_IGNORE: c_uint = 0x11;
pub const SECTOR_DELETED: c_uint = 0x00;
pub const FOLD_MARK_IN_PROGRESS: c_uint = 0x5555;
pub const ZONE_GOOD: c_uint = 0xff;
pub const ZONE_BAD_ORIGINAL: c_int = 0;
pub const ZONE_BAD_MARKED: c_int = 7;
