//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/dbdma.h
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
// Definitions for using the Apple Descriptor-Based DMA controller
// in Power Macintosh computers.
//
// Copyright (C) 1996 Paul Mackerras.
//

//
// DBDMA control/status registers.  All little-endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbdma_regs {
    pub /: *mut *mut unsigned int control; / lets you change bits in status,
    pub /: *mut *mut unsigned int status; / DMA and device status bits (see below),
    pub /: *mut *mut unsigned int cmdptr_hi; / upper 32 bits of command address,
    pub /: *mut *mut unsigned int cmdptr; / (lower 32 bits of) command address (phys),
    pub /: *mut *mut unsigned int intr_sel; / select interrupt condition bit,
    pub /: *mut *mut unsigned int br_sel; / select branch condition bit,
    pub /: *mut *mut unsigned int wait_sel; / select wait condition bit,
    pub xfer_mode: c_uint,
    pub data2ptr_hi: c_uint,
    pub data2ptr: c_uint,
    pub res1: c_uint,
    pub address_hi: c_uint,
    pub br_addr_hi: c_uint,
    pub res2: [c_uint; 3],
}

// Bits in control and status registers
pub const RUN: c_uint = 0x8000;
pub const PAUSE: c_uint = 0x4000;
pub const FLUSH: c_uint = 0x2000;
pub const WAKE: c_uint = 0x1000;
pub const DEAD: c_uint = 0x0800;
pub const ACTIVE: c_uint = 0x0400;
pub const BT: c_uint = 0x0100;
pub const DEVSTAT: c_uint = 0x00ff;
//
// DBDMA command structure.  These fields are all little-endian!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dbdma_cmd {
    pub /: *mut *mut __le16 req_count; / requested byte transfer count,
    pub /: *mut *mut __le16 command; / command word (has bit-fields),
    pub /: *mut *mut __le32 phy_addr; / physical data address,
    pub /: *mut *mut __le32 cmd_dep; / command-dependent field,
    pub /: *mut *mut __le16 res_count; / residual count after completion,
    pub /: *mut *mut __le16 xfer_status; / transfer status,
}

// DBDMA command values in command field

pub const OUTPUT_LAST: c_uint = 0x1000	/* ditto followed by end marker */;
pub const INPUT_MORE: c_uint = 0x2000	/* transfer stream data to memory */;
pub const INPUT_LAST: c_uint = 0x3000	/* ditto, expect end marker */;
pub const STORE_WORD: c_uint = 0x4000	/* write word (4 bytes) to device reg */;
pub const LOAD_WORD: c_uint = 0x5000	/* read word (4 bytes) from device reg */;
pub const DBDMA_NOP: c_uint = 0x6000	/* do nothing */;
pub const DBDMA_STOP: c_uint = 0x7000	/* suspend processing */;
// Key values in command field

pub const KEY_STREAM1: c_uint = 0x100	/* control/status stream */;
pub const KEY_STREAM2: c_uint = 0x200	/* device-dependent stream */;
pub const KEY_STREAM3: c_uint = 0x300	/* device-dependent stream */;
pub const KEY_REGS: c_uint = 0x500	/* device register space */;
pub const KEY_SYSTEM: c_uint = 0x600	/* system memory-mapped space */;
pub const KEY_DEVICE: c_uint = 0x700	/* device memory-mapped space */;
// Interrupt control values in command field

pub const INTR_IFSET: c_uint = 0x10	/* intr if condition bit is 1 */;
pub const INTR_IFCLR: c_uint = 0x20	/* intr if condition bit is 0 */;
pub const INTR_ALWAYS: c_uint = 0x30	/* always interrupt */;
// Branch control values in command field

pub const BR_IFSET: c_uint = 0x4	/* branch if condition bit is 1 */;
pub const BR_IFCLR: c_uint = 0x8	/* branch if condition bit is 0 */;
pub const BR_ALWAYS: c_uint = 0xc	/* always branch */;
// Wait control values in command field

// Align an address for a DBDMA command structure

// Useful macros

