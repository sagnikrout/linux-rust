//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_extfree_item.h
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
// Copyright (c) 2000,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//
// kernel only EFI/EFD definitions
//
// Max number of extents in fast allocation path.
//
pub const XFS_EFI_MAX_FAST_EXTENTS: c_int = 16;
//
// This is the "extent free intention" log item.  It is used to log the fact
// that some extents need to be free.  It is used in conjunction with the
// "extent free done" log item described below.
//
// The EFI is reference counted so that it is not freed prior to both the EFI
// and EFD being committed and unpinned. This ensures the EFI is inserted into
// the AIL even in the event of out of order EFI/EFD processing. In other words,
// an EFI is born with two references:
//
// 1.) an EFI held reference to track EFI AIL insertion
// 2.) an EFD held reference to track EFD commit
//
// On allocation, both references are the responsibility of the caller. Once the
// EFI is added to and dirtied in a transaction, ownership of reference one
// transfers to the transaction. The reference is dropped once the EFI is
// inserted to the AIL or in the event of failure along the way (e.g., commit
// failure, log I/O error, etc.). Note that the caller remains responsible for
// the EFD reference under all circumstances to this point. The caller has no
// means to detect failure once the transaction is committed, however.
// Therefore, an EFD is required after this point, even in the event of
// unrelated failure.
//
// Once an EFD is allocated and dirtied in a transaction, reference two
// transfers to the transaction. The EFD reference is dropped once it reaches
// the unpin handler. Similar to the EFI, the reference also drops in the event
// of commit failure or log I/O errors. Note that the EFD is not inserted in the
// AIL, so at this point both the EFI and EFD are freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efi_log_item {
    pub efi_item: xfs_log_item,
    pub efi_refcount: core::sync::atomic::AtomicI32,
    pub efi_next_extent: core::sync::atomic::AtomicI32,
    pub efi_format: xfs_efi_log_format,
}

//
// This is the "extent free done" log item.  It is used to log
// the fact that some extents earlier mentioned in an efi item
// have been freed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_efd_log_item {
    pub efd_item: xfs_log_item,
    pub efd_efip: *mut xfs_efi_log_item,
    pub efd_next_extent: c_uint,
    pub efd_format: xfs_efd_log_format,
}

//
// Max number of extents in fast allocation path.
//
pub const XFS_EFD_MAX_FAST_EXTENTS: c_int = 16;
extern "C" {
    pub fn xfs_efi_log_space(nr: c_uint) -> c_uint;
}
extern "C" {
    pub fn xfs_efd_log_space(nr: c_uint) -> c_uint;
}
