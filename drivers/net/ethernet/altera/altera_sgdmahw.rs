//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/altera/altera_sgdmahw.h
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
// SGDMA descriptor structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgdma_descrip {
    pub /: *mut *mut u32 raddr; / address of data to be read,
    pub pad1: u32,
    pub waddr: u32,
    pub pad2: u32,
    pub next: u32,
    pub pad3: u32,
    pub bytes: u16,
    pub rburst: u8,
    pub wburst: u8,
    pub /: *mut *mut u16 bytes_xferred; / 16 bits, bytes xferred,
// bit 0: error
// bit 1: length error
// bit 2: crc error
// bit 3: truncated error
// bit 4: phy error
// bit 5: collision error
// bit 6: reserved
// bit 7: status eop for recv case
//
    pub status: u8,
// bit 0: eop
// bit 1: read_fixed
// bit 2: write fixed
// bits 3,4,5,6: Channel (always 0)
// bit 7: hardware owned
//
    pub control: u8,
    pub __packed: },

// Channel is always 0, so just zero initialize it

// SGDMA register space
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgdma_csr {
// bit 0: error
// bit 1: eop
// bit 2: descriptor completed
// bit 3: chain completed
// bit 4: busy
// remainder reserved
//
    pub status: u32,
    pub pad1: [u32; 3],
// bit 0: interrupt on error
// bit 1: interrupt on eop
// bit 2: interrupt after every descriptor
// bit 3: interrupt after last descrip in a chain
// bit 4: global interrupt enable
// bit 5: starts descriptor processing
// bit 6: stop core on dma error
// bit 7: interrupt on max descriptors
// bits 8-15: max descriptors to generate interrupt
// bit 16: Software reset
// bit 17: clears owned by hardware if 0, does not clear otherwise
// bit 18: enables descriptor polling mode
// bit 19-26: clocks before polling again
// bit 27-30: reserved
// bit 31: clear interrupt
//
    pub control: u32,
    pub pad2: [u32; 3],
    pub next_descrip: u32,
    pub pad3: [u32; 3],
}

