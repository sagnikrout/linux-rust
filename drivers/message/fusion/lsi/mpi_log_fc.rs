//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/lsi/mpi_log_fc.h
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
// Copyright (c) 2000-2008 LSI Corporation. All rights reserved.
//
// NAME:           fc_log.h
// SUMMARY:        MPI IocLogInfo definitions for the SYMFC9xx chips
// DESCRIPTION:    Contains the enumerated list of values that may be returned
// in the IOCLogInfo field of a MPI Default Reply Message.
//
// CREATION DATE:  6/02/2000
// ID:             $Id: fc_log.h,v 4.6 2001/07/26 14:41:33 sschremm Exp $
//
// MpiIocLogInfo_t enum
//
// These 32 bit values are used in the IOCLogInfo field of the MPI reply
// messages.
// The value is 0xabcccccc where
// a = The type of log info as per the MPI spec. Since these codes are
// all for Fibre Channel this value will always be 2.
// b = Specifies a subclass of the firmware where
// 0 = FCP Initiator
// 1 = FCP Target
// 2 = LAN
// 3 = MPI Message Layer
// 4 = FC Link
// 5 = Context Manager
// 6 = Invalid Field Offset
// 7 = State Change Info
// all others are reserved for future use
// c = A specific value within the subclass.
//
// NOTE: Any new values should be added to the end of each subclass so that the
// codes remain consistent across firmware releases.
//
