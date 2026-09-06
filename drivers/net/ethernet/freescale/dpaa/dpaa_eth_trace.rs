//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa/dpaa_eth_trace.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0-or-later
//
// Copyright 2013-2015 Freescale Semiconductor Inc.
//

// This is used to declare a class of events.
// individual events of this type will be defined below.
//
// Store details about a frame descriptor and the FQ on which it was
// transmitted/received.
//
// Trace function prototype
// Repeat argument list here
// A structure containing the relevant information we want to record.
// Declare name and type for each normal element, name, type and size
// for arrays. Use __string for variable length strings.
//
// The function that assigns values to the above declared fields
// This is what gets printed when the trace event is triggered
// Now declare events of the above type. Format is:
// DEFINE_EVENT(class, name, proto, args), with proto and args same as for class
//
// Tx (egress) fd
// Rx fd
// Tx confirmation fd
// If only one event of a certain type needs to be declared, use TRACE_EVENT().
// The syntax is the same as for DECLARE_EVENT_CLASS().
//

// This must be outside ifdef _DPAA_ETH_TRACE_H

