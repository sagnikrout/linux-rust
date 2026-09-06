//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/altera/altera_msgdmahw.h
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
// Altera TSE SGDMA and MSGDMA Linux driver
// Copyright (C) 2014 Altera Corporation. All rights reserved
//
// mSGDMA extended descriptor format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_extended_desc {
    pub /: *mut *mut u32 read_addr_lo; / data buffer source address low bits,
    pub /: *mut *mut u32 write_addr_lo; / data buffer destination address low bits,
    pub transfer: *mut *mut u32 len; / the number of bytes to,
// per descriptor
//
    pub burst: *mut *mut u32 burst_seq_num; / bit 31:24 write,
// bit 23:16 read burst
// bit 15:0  sequence number
//
    pub stride: *mut *mut u32 stride; / bit 31:16 write,
// bit 15:0  read stride
//
    pub /: *mut *mut u32 read_addr_hi; / data buffer source address high bits,
    pub /: *mut *mut u32 write_addr_hi; / data buffer destination address high bits,
    pub /: *mut *mut u32 control; / characteristics of the transfer,
}

// mSGDMA descriptor control field bit definitions
//

// Writing ‘1’ to the ‘go’ bit commits the entire descriptor into the
// descriptor FIFO(s)
//

// Tx buffer control flags
//

// mSGDMA extended descriptor stride definitions
//

// mSGDMA dispatcher control and status register map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_csr {
    pub /: *mut *mut u32 status; / Read/Clear,
    pub /: *mut *mut u32 control; / Read/Write,
    pub level: *mut *mut u32 rw_fill_level; / bit 31:16 - write fill,
// bit 15:0  - read fill level
//
    pub /: *mut *mut u32 resp_fill_level; / bit 15:0,
    pub number: *mut *mut u32 rw_seq_num; / bit 31:16 - write sequence,
// bit 15:0  - read sequence number
//
    pub /: *mut *mut u32 pad[3]; / reserved,
}

// mSGDMA CSR status register bit definitions
//

pub const MSGDMA_CSR_STAT_MASK: c_uint = 0x3FF;
pub const MSGDMA_CSR_STAT_MASK_WITHOUT_IRQ: c_uint = 0x1FF;

// mSGDMA CSR control register bit definitions
//

// mSGDMA CSR fill level bits
//

// mSGDMA response register map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msgdma_response {
    pub bytes_transferred: u32,
    pub status: u32,
}

// mSGDMA response register bit definitions
//

pub const MSGDMA_RESP_ERR_MASK: c_uint = 0xFF;
