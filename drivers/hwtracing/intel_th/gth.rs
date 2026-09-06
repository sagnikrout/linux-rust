//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/intel_th/gth.h
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
// Intel(R) Trace Hub Global Trace Hub (GTH) data structures
//
// Copyright (C) 2014-2015 Intel Corporation.
//
// Map output port parameter bits to symbolic names

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_th_output_parm {
// output port type
    TH_OUTPUT_PARM(port),
// generate NULL packet
    TH_OUTPUT_PARM(null),
// packet drop
    TH_OUTPUT_PARM(drop),
// port in reset state
    TH_OUTPUT_PARM(reset),
// flush out data
    TH_OUTPUT_PARM(flush),
// mainenance packet frequency
    TH_OUTPUT_PARM(smcfreq),
}

//
// Register offsets
//
// Common Capture Sequencer (CTS) registers
// waiting for Pipeline Empty bit(s) to assert for GTH
pub const GTH_PLE_WAITLOOP_DEPTH: c_int = 10000;

// waiting for Trigger status to assert for CTS
pub const CTS_TRIG_WAITLOOP_DEPTH: c_int = 10000;

pub const CTS_ACTION_CONTROL_STATE_OFF: c_int = 27;

pub const CTS_STATE_IDLE: c_uint = 0x10u;

