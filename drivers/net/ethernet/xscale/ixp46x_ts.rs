//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/xscale/ixp46x_ts.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PTP 1588 clock using the IXP46X
//
// Copyright (C) 2010 OMICRON electronics GmbH
//
pub const DEFAULT_ADDEND: c_uint = 0xF0000029;
pub const TICKS_NS_SHIFT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp46x_channel_ctl {
    pub /: *mut *mut u32 ch_control; / 0x40 Time Synchronization Channel Control,
    pub /: *mut *mut u32 ch_event; / 0x44 Time Synchronization Channel Event,
    pub /: *mut *mut u32 tx_snap_lo; / 0x48 Transmit Snapshot Low Register,
    pub /: *mut *mut u32 tx_snap_hi; / 0x4C Transmit Snapshot High Register,
    pub /: *mut *mut u32 rx_snap_lo; / 0x50 Receive Snapshot Low Register,
    pub /: *mut *mut u32 rx_snap_hi; / 0x54 Receive Snapshot High Register,
    pub /: *mut *mut u32 src_uuid_lo; / 0x58 Source UUID0 Low Register,
    pub /: *mut *mut u32 src_uuid_hi; / 0x5C Sequence Identifier/Source UUID0 High,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ixp46x_ts_regs {
    pub /: *mut *mut u32 control; / 0x00 Time Sync Control Register,
    pub /: *mut *mut u32 event; / 0x04 Time Sync Event Register,
    pub /: *mut *mut u32 addend; / 0x08 Time Sync Addend Register,
    pub /: *mut *mut u32 accum; / 0x0C Time Sync Accumulator Register,
    pub /: *mut *mut u32 test; / 0x10 Time Sync Test Register,
    pub /: *mut *mut u32 unused; / 0x14,
    pub /: *mut *mut u32 rsystime_lo; / 0x18 RawSystemTime_Low Register,
    pub /: *mut *mut u32 rsystime_hi; / 0x1C RawSystemTime_High Register,
    pub /: *mut *mut u32 systime_lo; / 0x20 SystemTime_Low Register,
    pub /: *mut *mut u32 systime_hi; / 0x24 SystemTime_High Register,
    pub /: *mut *mut u32 trgt_lo; / 0x28 TargetTime_Low Register,
    pub /: *mut *mut u32 trgt_hi; / 0x2C TargetTime_High Register,
    pub /: *mut *mut u32 asms_lo; / 0x30 Auxiliary Slave Mode Snapshot Low,
    pub /: *mut *mut u32 asms_hi; / 0x34 Auxiliary Slave Mode Snapshot High,
    pub /: *mut *mut u32 amms_lo; / 0x38 Auxiliary Master Mode Snapshot Low,
    pub /: *mut *mut u32 amms_hi; / 0x3C Auxiliary Master Mode Snapshot High,
    pub channel: [ixp46x_channel_ctl; 3],
}

// 0x00 Time Sync Control Register Bits

// 0x04 Time Sync Event Register Bits

// 0x40 Time Synchronization Channel Control Register Bits

// 0x44 Time Synchronization Channel Event Register Bits

extern "C" {
    pub fn ixp46x_ptp_find(regs: *mut *mut ixp46x_ts_regs __iomem, phc_index: *mut c_int) -> c_int;
}

// regs = NULL;
// phc_index = -1;

