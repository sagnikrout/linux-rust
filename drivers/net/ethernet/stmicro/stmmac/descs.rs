//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/descs.h
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

// Normal receive descriptor defines
// RDES0

// RDES1

// Enhanced receive descriptor defines
// RDES0 (similar to normal RDES)

// RDES1: completely differ from normal desc definitions

// Normal transmit descriptor defines
// TDES0

// TDES1

// Enhanced transmit descriptor defines
// TDES0

// TDES1

// Extended Receive descriptor definitions

// Extended RDES4 message type definitions
pub const RDES_EXT_NO_PTP: c_uint = 0x0;
pub const RDES_EXT_SYNC: c_uint = 0x1;
pub const RDES_EXT_FOLLOW_UP: c_uint = 0x2;
pub const RDES_EXT_DELAY_REQ: c_uint = 0x3;
pub const RDES_EXT_DELAY_RESP: c_uint = 0x4;
pub const RDES_EXT_PDELAY_REQ: c_uint = 0x5;
pub const RDES_EXT_PDELAY_RESP: c_uint = 0x6;
pub const RDES_EXT_PDELAY_FOLLOW_UP: c_uint = 0x7;
pub const RDES_PTP_ANNOUNCE: c_uint = 0x8;
pub const RDES_PTP_MANAGEMENT: c_uint = 0x9;
pub const RDES_PTP_SIGNALING: c_uint = 0xa;
pub const RDES_PTP_PKT_RESERVED_TYPE: c_uint = 0xf;
// Basic descriptor structure for normal and alternate descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_desc {
    pub des0: __le32,
    pub des1: __le32,
    pub des2: __le32,
    pub des3: __le32,
}

// Extended descriptor structure (e.g. >= databook 3.50a)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_extended_desc {
    pub /: *mut *mut dma_desc basic; / Basic descriptors,
    pub /: *mut *mut __le32 des4; / Extended Status,
    pub /: *mut *mut __le32 des5; / Reserved,
    pub /: *mut *mut __le32 des6; / Tx/Rx Timestamp Low,
    pub /: *mut *mut __le32 des7; / Tx/Rx Timestamp High,
}

// Enhanced descriptor for TBS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_edesc {
    pub des4: __le32,
    pub des5: __le32,
    pub des6: __le32,
    pub des7: __le32,
    pub basic: dma_desc,
}

// Transmit checksum insertion control

