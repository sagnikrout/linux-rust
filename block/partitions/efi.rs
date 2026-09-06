//! Automatically rewritten from C Header to Rust Module
//! Source: block/partitions/efi.h
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
// EFI GUID Partition Table
// Per Intel EFI Specification v1.02
// http://developer.intel.com/technology/efi/efi.htm
//
// By Matt Domsch <Matt_Domsch@dell.com>  Fri Sep 22 22:15:56 CDT 2000
// Copyright 2000,2001 Dell Inc.
//

// Macro flag: #define FS_PART_EFI_H_INCLUDED

pub const MSDOS_MBR_SIGNATURE: c_uint = 0xaa55;
pub const EFI_PMBR_OSTYPE_EFI: c_uint = 0xEF;
pub const EFI_PMBR_OSTYPE_EFI_GPT: c_uint = 0xEE;
pub const GPT_MBR_PROTECTIVE: c_int = 1;
pub const GPT_MBR_HYBRID: c_int = 2;
pub const GPT_HEADER_SIGNATURE: c_uint = 0x5452415020494645ULL;
pub const GPT_HEADER_REVISION_V1: c_uint = 0x00010000;
pub const GPT_PRIMARY_PARTITION_TABLE_LBA: c_int = 1;

// The rest of the logical block is reserved by UEFI and must be zero.
// EFI standard handles this by:
//
// uint8_t		reserved2[ BlockSize - 92 ];
//
