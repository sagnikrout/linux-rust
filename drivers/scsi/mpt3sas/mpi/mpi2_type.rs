//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/mpt3sas/mpi/mpi2_type.h
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
// Copyright 2000-2014 Avago Technologies.  All rights reserved.
//
// Name:  mpi2_type.h
// Title:  MPI basic type definitions
// Creation Date:  August 16, 2006
//
// mpi2_type.h Version:  02.00.01
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 04-30-07  02.00.00  Corresponds to Fusion-MPT MPI Specification Rev A.
// 11-18-14  02.00.01  Updated copyright information.
// --------------------------------------------------------------------------
//
// Define * if it hasn't already been defined. By default
// * is defined to be a near pointer. MPI2_POINTER can be defined as
// a far pointer by defining * as "far *" before this header file is
// included.
//
// the basic types may have already been included by mpi_type.h
//
// Basic Types
//
pub type U8 = u8;
pub type U16 = __le16;
pub type U32 = __le32;
extern "C" {
    pub fn __attribute__(_arg: (aligned(4))) -> typedef __le64 U64;
}
//
// Pointer Types
//

