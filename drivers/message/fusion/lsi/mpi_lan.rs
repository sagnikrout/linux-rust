//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_lan.h
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
// Name:  mpi_lan.h
// Title:  MPI LAN messages and structures
// Creation Date:  June 30, 2000
//
// mpi_lan.h Version:  01.05.01
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 05-24-00  00.10.02  Added LANStatus field to _MSG_LAN_SEND_REPLY.
// Added LANStatus field to _MSG_LAN_RECEIVE_POST_REPLY.
// Moved ListCount field in _MSG_LAN_RECEIVE_POST_REPLY.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 06-12-00  01.00.02  Added MPI_ to BUCKETSTATUS_ definitions.
// 06-22-00  01.00.03  Major changes to match new LAN definition in 1.0 spec.
// 06-30-00  01.00.04  Added Context Reply definitions per revised proposal.
// Changed transaction context usage to bucket/buffer.
// 07-05-00  01.00.05  Removed LAN_RECEIVE_POST_BUCKET_CONTEXT_MASK definition
// to lan private header file
// 11-02-00  01.01.01  Original release for post 1.0 work
// 02-20-01  01.01.02  Started using MPI_POINTER.
// 03-27-01  01.01.03  Added structure offset comments.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Original release for MPI v1.5.
// --------------------------------------------------------------------------
//
// L A N    M e s s a g e s
//
// LANSend messages
// LANReceivePost
// LANReset
//
// LAN Context Reply defines and macros
//

//
// LAN Current Device State defines
//

//
// LAN Loopback defines
//

