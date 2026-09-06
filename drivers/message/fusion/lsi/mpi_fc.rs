//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_fc.h
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
// Name:  mpi_fc.h
// Title:  MPI Fibre Channel messages and structures
// Creation Date:  June 12, 2000
//
// mpi_fc.h Version:  01.05.01
//
// Version History
// ---------------
//
// Date      Version   Description
// --------  --------  ------------------------------------------------------
// 05-08-00  00.10.01  Original release for 0.10 spec dated 4/26/2000.
// 06-06-00  01.00.01  Update version number for 1.0 release.
// 06-12-00  01.00.02  Added _MSG_FC_ABORT_REPLY structure.
// 11-02-00  01.01.01  Original release for post 1.0 work
// 12-04-00  01.01.02  Added messages for Common Transport Send and
// Primitive Send.
// 01-09-01  01.01.03  Modifed some of the new flags to have an MPI prefix
// and modified the FcPrimitiveSend flags.
// 01-25-01  01.01.04  Move InitiatorIndex in LinkServiceRsp reply to a larger
// field.
// Added FC_ABORT_TYPE_CT_SEND_REQUEST and
// FC_ABORT_TYPE_EXLINKSEND_REQUEST for FcAbort request.
// Added MPI_FC_PRIM_SEND_FLAGS_STOP_SEND.
// 02-20-01  01.01.05  Started using MPI_POINTER.
// 03-27-01  01.01.06  Added Flags field to MSG_LINK_SERVICE_BUFFER_POST_REPLY
// and defined MPI_LS_BUF_POST_REPLY_FLAG_NO_RSP_NEEDED.
// Added MPI_FC_PRIM_SEND_FLAGS_RESET_LINK define.
// Added structure offset comments.
// 04-09-01  01.01.07  Added RspLength field to MSG_LINK_SERVICE_RSP_REQUEST.
// 08-08-01  01.02.01  Original release for v1.2 work.
// 09-28-01  01.02.02  Change name of reserved field in
// MSG_LINK_SERVICE_RSP_REPLY.
// 05-31-02  01.02.03  Adding AliasIndex to FC Direct Access requests.
// 01-16-04  01.02.04  Added define for MPI_FC_PRIM_SEND_FLAGS_ML_RESET_LINK.
// 05-11-04  01.03.01  Original release for MPI v1.3.
// 08-19-04  01.05.01  Original release for MPI v1.5.
// --------------------------------------------------------------------------
//
// F C    D i r e c t    A c c e s s     M e s s a g e s
//
// Link Service Buffer Post messages
//

// Link Service Buffer Post Reply

// obsolete name for the above

//
// Link Service Response messages
//

// Link Service Response Reply
//
// Extended Link Service Send messages
//

// Extended Link Service Send Reply
//
// FC Abort messages
//

// FC Abort Reply
//
// FC Common Transport Send messages
//

// FC Common Transport Send Reply
//
// FC Primitive Send messages
//

// FC Primitive Send Reply
