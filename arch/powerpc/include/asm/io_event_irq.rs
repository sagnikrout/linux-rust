//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/io_event_irq.h
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
// Copyright 2010, 2011 Mark Nelson and Tseng-Hui (Frank) Lin, IBM Corporation
//

pub const PSERIES_IOEI_RPC_MAX_LEN: c_int = 216;
pub const PSERIES_IOEI_TYPE_ERR_DETECTED: c_uint = 0x01;
pub const PSERIES_IOEI_TYPE_ERR_RECOVERED: c_uint = 0x02;
pub const PSERIES_IOEI_TYPE_EVENT: c_uint = 0x03;
pub const PSERIES_IOEI_TYPE_RPC_PASS_THRU: c_uint = 0x04;
pub const PSERIES_IOEI_SUBTYPE_NOT_APP: c_uint = 0x00;
pub const PSERIES_IOEI_SUBTYPE_REBALANCE_REQ: c_uint = 0x01;
pub const PSERIES_IOEI_SUBTYPE_NODE_ONLINE: c_uint = 0x03;
pub const PSERIES_IOEI_SUBTYPE_NODE_OFFLINE: c_uint = 0x04;
pub const PSERIES_IOEI_SUBTYPE_DUMP_SIZE_CHANGE: c_uint = 0x05;
pub const PSERIES_IOEI_SUBTYPE_TORRENT_IRV_UPDATE: c_uint = 0x06;
pub const PSERIES_IOEI_SUBTYPE_TORRENT_HFI_CFGED: c_uint = 0x07;
pub const PSERIES_IOEI_SCOPE_NOT_APP: c_uint = 0x00;
pub const PSERIES_IOEI_SCOPE_RIO_HUB: c_uint = 0x36;
pub const PSERIES_IOEI_SCOPE_RIO_BRIDGE: c_uint = 0x37;
pub const PSERIES_IOEI_SCOPE_PHB: c_uint = 0x38;
pub const PSERIES_IOEI_SCOPE_EADS_GLOBAL: c_uint = 0x39;
pub const PSERIES_IOEI_SCOPE_EADS_SLOT: c_uint = 0x3A;
pub const PSERIES_IOEI_SCOPE_TORRENT_HUB: c_uint = 0x3B;
pub const PSERIES_IOEI_SCOPE_SERVICE_PROC: c_uint = 0x51;
// Platform Event Log Format, Version 6, data portition of IO event section
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_io_event {
    pub /: *mut *mut uint8_t event_type; / 0x00 IO-Event Type,
    pub /: *mut *mut uint8_t rpc_data_len; / 0x01 RPC data length,
    pub /: *mut *mut uint8_t scope; / 0x02 Error/Event Scope,
    pub /: *mut *mut uint8_t event_subtype; / 0x03 I/O-Event Sub-Type,
    pub /: *mut *mut uint32_t drc_index; / 0x04 DRC Index,
    pub rpc_data: [u8; PSERIES_IOEI_RPC_MAX_LEN],
// 0x08 RPC Data (0-216 bytes,
// padded to 4 bytes alignment)
}
