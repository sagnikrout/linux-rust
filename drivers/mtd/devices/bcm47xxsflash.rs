//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/devices/bcm47xxsflash.h
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

// Used for ST flashes only.
pub const OPCODE_ST_WREN: c_uint = 0x0006		/* Write Enable */;
pub const OPCODE_ST_WRDIS: c_uint = 0x0004		/* Write Disable */;
pub const OPCODE_ST_RDSR: c_uint = 0x0105		/* Read Status Register */;
pub const OPCODE_ST_WRSR: c_uint = 0x0101		/* Write Status Register */;
pub const OPCODE_ST_READ: c_uint = 0x0303		/* Read Data Bytes */;
pub const OPCODE_ST_PP: c_uint = 0x0302		/* Page Program */;
pub const OPCODE_ST_SE: c_uint = 0x02d8		/* Sector Erase */;
pub const OPCODE_ST_BE: c_uint = 0x00c7		/* Bulk Erase */;
pub const OPCODE_ST_DP: c_uint = 0x00b9		/* Deep Power-down */;
pub const OPCODE_ST_RES: c_uint = 0x03ab		/* Read Electronic Signature */;
pub const OPCODE_ST_CSA: c_uint = 0x1000		/* Keep chip select asserted */;
pub const OPCODE_ST_SSE: c_uint = 0x0220		/* Sub-sector Erase */;
pub const OPCODE_ST_READ4B: c_uint = 0x6313		/* Read Data Bytes in 4Byte addressing mode */;
// Used for Atmel flashes only.
pub const OPCODE_AT_READ: c_uint = 0x07e8;
pub const OPCODE_AT_PAGE_READ: c_uint = 0x07d2;
pub const OPCODE_AT_STATUS: c_uint = 0x01d7;
pub const OPCODE_AT_BUF1_WRITE: c_uint = 0x0384;
pub const OPCODE_AT_BUF2_WRITE: c_uint = 0x0387;
pub const OPCODE_AT_BUF1_ERASE_PROGRAM: c_uint = 0x0283;
pub const OPCODE_AT_BUF2_ERASE_PROGRAM: c_uint = 0x0286;
pub const OPCODE_AT_BUF1_PROGRAM: c_uint = 0x0288;
pub const OPCODE_AT_BUF2_PROGRAM: c_uint = 0x0289;
pub const OPCODE_AT_PAGE_ERASE: c_uint = 0x0281;
pub const OPCODE_AT_BLOCK_ERASE: c_uint = 0x0250;
pub const OPCODE_AT_BUF1_WRITE_ERASE_PROGRAM: c_uint = 0x0382;
pub const OPCODE_AT_BUF2_WRITE_ERASE_PROGRAM: c_uint = 0x0385;
pub const OPCODE_AT_BUF1_LOAD: c_uint = 0x0253;
pub const OPCODE_AT_BUF2_LOAD: c_uint = 0x0255;
pub const OPCODE_AT_BUF1_COMPARE: c_uint = 0x0260;
pub const OPCODE_AT_BUF2_COMPARE: c_uint = 0x0261;
pub const OPCODE_AT_BUF1_REPROGRAM: c_uint = 0x0258;
pub const OPCODE_AT_BUF2_REPROGRAM: c_uint = 0x0259;
// Status register bits for ST flashes
pub const SR_ST_WIP: c_uint = 0x01		/* Write In Progress */;
pub const SR_ST_WEL: c_uint = 0x02		/* Write Enable Latch */;
pub const SR_ST_BP_MASK: c_uint = 0x1c		/* Block Protect */;
pub const SR_ST_BP_SHIFT: c_int = 2;
pub const SR_ST_SRWD: c_uint = 0x80		/* Status Register Write Disable */;
// Status register bits for Atmel flashes
pub const SR_AT_READY: c_uint = 0x80;
pub const SR_AT_MISMATCH: c_uint = 0x40;
pub const SR_AT_ID_MASK: c_uint = 0x38;
pub const SR_AT_ID_SHIFT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm47xxsflash_type {
    BCM47XXSFLASH_TYPE_ATMEL,
    BCM47XXSFLASH_TYPE_ST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm47xxsflash {
    pub bcma_cc: *mut bcma_drv_cc,
    pub offset): *mut *mut *mut int (cc_read)(struct bcm47xxsflash b47s, u16,
    pub value): *mut *mut *mut void (cc_write)(struct bcm47xxsflash b47s, u16 offset, u32,
    pub type: bcm47xxsflash_type,
    pub window: *mut void __iomem,
    pub blocksize: u32,
    pub numblocks: u16,
    pub size: u32,
    pub mtd: mtd_info,
}
