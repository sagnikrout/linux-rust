//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvdimm/pfn.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2014-2015, Intel Corporation.
//

pub const PFN_SIG_LEN: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nd_pfn_sb {
    pub signature: [u8; PFN_SIG_LEN],
    pub uuid: [u8; 16],
    pub parent_uuid: [u8; 16],
    pub flags: __le32,
    pub version_major: __le16,
    pub version_minor: __le16,
    pub /: *mut *mut __le64 dataoff; / relative to namespace_base + start_pad,
    pub npfns: __le64,
    pub mode: __le32,
// minor-version-1 additions for section alignment
//
// @start_pad: Deprecated attribute to pad start-misaligned namespaces
//
// start_pad is deprecated because the original definition did
// not comprehend that dataoff is relative to the base address
// of the namespace not the start_pad adjusted base. The result
// is that the dax path is broken, but the block-I/O path is
// not. The kernel will no longer create namespaces using start
// padding, but it still supports block-I/O for legacy
// configurations mainly to allow a backup, reconfigure the
// namespace, and restore flow to repair dax operation.
//
    pub start_pad: __le32,
    pub end_trunc: __le32,
// minor-version-2 record the base alignment of the mapping
    pub align: __le32,
// minor-version-3 guarantee the padding and flags are zero
// minor-version-4 record the page size and struct page size
    pub page_size: __le32,
    pub page_struct_size: __le16,
    pub padding: [u8; 3994],
    pub checksum: __le64,
}
