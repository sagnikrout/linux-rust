//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/arm/arm_scsi.h
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
// Copyright (C) 2002 Russell King
//
// Commonly used functions by the ARM SCSI-II drivers.
//

// Macro flag: #define BELT_AND_BRACES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_cmd_priv {
    pub scsi_pointer: scsi_pointer,
}

//
// The scatter-gather list handling.  This contains all
// the yucky stuff that needs to be fixed properly.
//
// copy_SCp_to_sg() Assumes contiguous allocation at @sg of at-most @max
// entries of uninitialized memory. SCp is from scsi-ml and has a valid
// (possibly chained) sg-list
//
// FIXME: It should be easy for drivers to loop on copy_SCp_to_sg().
// and to remove this BUG_ON. Use min() in-its-place
//
// (++sg) = *src_sg;
// SCp->ptr = c;

// Calculate correct buffer length.  Some commands
// come in with the wrong scsi_bufflen.
//
// FIXME: Totaly naive fixup. We should abort
// with error
//
