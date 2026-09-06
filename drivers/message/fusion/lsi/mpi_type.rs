//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_type.h
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
// Copyright (c) 2000-2008 LSI Corporation.
//
// Name:  mpi_type.h
// Title:  MPI Basic type definitions
// Creation Date:  June 6, 2000
//
// mpi_type.h Version:  01.05.02
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 11-02-00  01.01.01  Original release for post 1.0 work
// 02-20-01  01.01.02  Added define and ifdef for MPI_POINTER.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Original release for MPI v1.5.
// --------------------------------------------------------------------------
//
// Define MPI_POINTER if it hasn't already been defined. By default MPI_POINTER
// is defined to be a near pointer. MPI_POINTER can be defined as a far pointer
// by defining MPI_POINTER as "far *" before this header file is included.
//

//
// B a s i c    T y p e s
//
pub type S8 = signed   char;
pub type U8 = c_uchar;
pub type S16 = signed   short;
pub type U16 = c_ushort;
pub type S32 = i32;
pub type U32 = u_int32_t;
//
// Pointers
//
