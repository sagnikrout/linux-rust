//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/devices/docg3.h
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
// Handles the M-Systems DiskOnChip G3 chip
//
// Copyright (C) 2011 Robert Jarzmik
//

//
// Flash memory areas :
// - 0x0000 .. 0x07ff : IPL
// - 0x0800 .. 0x0fff : Data area
// - 0x1000 .. 0x17ff : Registers
// - 0x1800 .. 0x1fff : Unknown
//
pub const DOC_IOSPACE_IPL: c_uint = 0x0000;
pub const DOC_IOSPACE_DATA: c_uint = 0x0800;
pub const DOC_IOSPACE_SIZE: c_uint = 0x2000;
//
// DOC G3 layout and adressing scheme
// A page address for the block "b", plane "P" and page "p":
// address = [bbbb bPpp pppp]
//
pub const DOC_ADDR_PAGE_MASK: c_uint = 0x3f;
pub const DOC_ADDR_BLOCK_SHIFT: c_int = 6;
pub const DOC_LAYOUT_NBPLANES: c_int = 2;
pub const DOC_LAYOUT_PAGES_PER_BLOCK: c_int = 64;
pub const DOC_LAYOUT_PAGE_SIZE: c_int = 512;
pub const DOC_LAYOUT_OOB_SIZE: c_int = 16;
pub const DOC_LAYOUT_WEAR_SIZE: c_int = 8;

//
// ECC related constants
//
pub const DOC_ECC_BCH_M: c_int = 14;
pub const DOC_ECC_BCH_T: c_int = 4;
pub const DOC_ECC_BCH_PRIMPOLY: c_uint = 0x4443;
pub const DOC_ECC_BCH_SIZE: c_int = 7;

//
// Blocks distribution
//
pub const DOC_LAYOUT_BLOCK_BBT: c_int = 0;
pub const DOC_LAYOUT_BLOCK_OTP: c_int = 0;
pub const DOC_LAYOUT_BLOCK_FIRST_DATA: c_int = 6;
pub const DOC_LAYOUT_PAGE_BBT: c_int = 4;
//
// Extra page OOB (16 bytes wide) layout
//
pub const DOC_LAYOUT_OOB_PAGEINFO_OFS: c_int = 0;
pub const DOC_LAYOUT_OOB_HAMMING_OFS: c_int = 7;
pub const DOC_LAYOUT_OOB_BCH_OFS: c_int = 8;
pub const DOC_LAYOUT_OOB_UNUSED_OFS: c_int = 15;
pub const DOC_LAYOUT_OOB_PAGEINFO_SZ: c_int = 7;
pub const DOC_LAYOUT_OOB_HAMMING_SZ: c_int = 1;
pub const DOC_LAYOUT_OOB_BCH_SZ: c_int = 7;
pub const DOC_LAYOUT_OOB_UNUSED_SZ: c_int = 1;
pub const DOC_CHIPID_G3: c_uint = 0x200;
pub const DOC_ERASE_MARK: c_uint = 0xaa;
pub const DOC_MAX_NBFLOORS: c_int = 4;
//
// Flash registers
//
pub const DOC_CHIPID: c_uint = 0x1000;
pub const DOC_TEST: c_uint = 0x1004;
pub const DOC_BUSLOCK: c_uint = 0x1006;
pub const DOC_ENDIANCONTROL: c_uint = 0x1008;
pub const DOC_DEVICESELECT: c_uint = 0x100a;
pub const DOC_ASICMODE: c_uint = 0x100c;
pub const DOC_CONFIGURATION: c_uint = 0x100e;
pub const DOC_INTERRUPTCONTROL: c_uint = 0x1010;
pub const DOC_READADDRESS: c_uint = 0x101a;
pub const DOC_DATAEND: c_uint = 0x101e;
pub const DOC_INTERRUPTSTATUS: c_uint = 0x1020;
pub const DOC_FLASHSEQUENCE: c_uint = 0x1032;
pub const DOC_FLASHCOMMAND: c_uint = 0x1034;
pub const DOC_FLASHADDRESS: c_uint = 0x1036;
pub const DOC_FLASHCONTROL: c_uint = 0x1038;
pub const DOC_NOP: c_uint = 0x103e;
pub const DOC_ECCCONF0: c_uint = 0x1040;
pub const DOC_ECCCONF1: c_uint = 0x1042;
pub const DOC_ECCPRESET: c_uint = 0x1044;
pub const DOC_HAMMINGPARITY: c_uint = 0x1046;

pub const DOC_PROTECTION: c_uint = 0x1056;
pub const DOC_DPS0_KEY: c_uint = 0x105c;
pub const DOC_DPS1_KEY: c_uint = 0x105e;
pub const DOC_DPS0_ADDRLOW: c_uint = 0x1060;
pub const DOC_DPS0_ADDRHIGH: c_uint = 0x1062;
pub const DOC_DPS1_ADDRLOW: c_uint = 0x1064;
pub const DOC_DPS1_ADDRHIGH: c_uint = 0x1066;
pub const DOC_DPS0_STATUS: c_uint = 0x106c;
pub const DOC_DPS1_STATUS: c_uint = 0x106e;
pub const DOC_ASICMODECONFIRM: c_uint = 0x1072;
pub const DOC_CHIPID_INV: c_uint = 0x1074;
pub const DOC_POWERMODE: c_uint = 0x107c;
//
// Flash sequences
// A sequence is preset before one or more commands are input to the chip.
//
pub const DOC_SEQ_RESET: c_uint = 0x00;
pub const DOC_SEQ_PAGE_SIZE_532: c_uint = 0x03;
pub const DOC_SEQ_SET_FASTMODE: c_uint = 0x05;
pub const DOC_SEQ_SET_RELIABLEMODE: c_uint = 0x09;
pub const DOC_SEQ_READ: c_uint = 0x12;
pub const DOC_SEQ_SET_PLANE1: c_uint = 0x0e;
pub const DOC_SEQ_SET_PLANE2: c_uint = 0x10;
pub const DOC_SEQ_PAGE_SETUP: c_uint = 0x1d;
pub const DOC_SEQ_ERASE: c_uint = 0x27;
pub const DOC_SEQ_PLANES_STATUS: c_uint = 0x31;
//
// Flash commands
//
pub const DOC_CMD_READ_PLANE1: c_uint = 0x00;
pub const DOC_CMD_SET_ADDR_READ: c_uint = 0x05;
pub const DOC_CMD_READ_ALL_PLANES: c_uint = 0x30;
pub const DOC_CMD_READ_PLANE2: c_uint = 0x50;
pub const DOC_CMD_READ_FLASH: c_uint = 0xe0;
pub const DOC_CMD_PAGE_SIZE_532: c_uint = 0x3c;
pub const DOC_CMD_PROG_BLOCK_ADDR: c_uint = 0x60;
pub const DOC_CMD_PROG_CYCLE1: c_uint = 0x80;
pub const DOC_CMD_PROG_CYCLE2: c_uint = 0x10;
pub const DOC_CMD_PROG_CYCLE3: c_uint = 0x11;
pub const DOC_CMD_ERASECYCLE2: c_uint = 0xd0;
pub const DOC_CMD_READ_STATUS: c_uint = 0x70;
pub const DOC_CMD_PLANES_STATUS: c_uint = 0x71;
pub const DOC_CMD_RELIABLE_MODE: c_uint = 0x22;
pub const DOC_CMD_FAST_MODE: c_uint = 0xa2;
pub const DOC_CMD_RESET: c_uint = 0xff;
//
// Flash register : DOC_FLASHCONTROL
//
pub const DOC_CTRL_VIOLATION: c_uint = 0x20;
pub const DOC_CTRL_CE: c_uint = 0x10;
pub const DOC_CTRL_UNKNOWN_BITS: c_uint = 0x08;
pub const DOC_CTRL_PROTECTION_ERROR: c_uint = 0x04;
pub const DOC_CTRL_SEQUENCE_ERROR: c_uint = 0x02;
pub const DOC_CTRL_FLASHREADY: c_uint = 0x01;
//
// Flash register : DOC_ASICMODE
//
pub const DOC_ASICMODE_RESET: c_uint = 0x00;
pub const DOC_ASICMODE_NORMAL: c_uint = 0x01;
pub const DOC_ASICMODE_POWERDOWN: c_uint = 0x02;
pub const DOC_ASICMODE_MDWREN: c_uint = 0x04;
pub const DOC_ASICMODE_BDETCT_RESET: c_uint = 0x08;
pub const DOC_ASICMODE_RSTIN_RESET: c_uint = 0x10;
pub const DOC_ASICMODE_RAM_WE: c_uint = 0x20;
//
// Flash register : DOC_ECCCONF0
//
pub const DOC_ECCCONF0_WRITE_MODE: c_uint = 0x0000;
pub const DOC_ECCCONF0_READ_MODE: c_uint = 0x8000;
pub const DOC_ECCCONF0_AUTO_ECC_ENABLE: c_uint = 0x4000;
pub const DOC_ECCCONF0_HAMMING_ENABLE: c_uint = 0x1000;
pub const DOC_ECCCONF0_BCH_ENABLE: c_uint = 0x0800;
pub const DOC_ECCCONF0_DATA_BYTES_MASK: c_uint = 0x07ff;
//
// Flash register : DOC_ECCCONF1
//
pub const DOC_ECCCONF1_BCH_SYNDROM_ERR: c_uint = 0x80;
pub const DOC_ECCCONF1_UNKOWN1: c_uint = 0x40;
pub const DOC_ECCCONF1_PAGE_IS_WRITTEN: c_uint = 0x20;
pub const DOC_ECCCONF1_UNKOWN3: c_uint = 0x10;
pub const DOC_ECCCONF1_HAMMING_BITS_MASK: c_uint = 0x0f;
//
// Flash register : DOC_PROTECTION
//
pub const DOC_PROTECT_FOUNDRY_OTP_LOCK: c_uint = 0x01;
pub const DOC_PROTECT_CUSTOMER_OTP_LOCK: c_uint = 0x02;
pub const DOC_PROTECT_LOCK_INPUT: c_uint = 0x04;
pub const DOC_PROTECT_STICKY_LOCK: c_uint = 0x08;
pub const DOC_PROTECT_PROTECTION_ENABLED: c_uint = 0x10;
pub const DOC_PROTECT_IPL_DOWNLOAD_LOCK: c_uint = 0x20;
pub const DOC_PROTECT_PROTECTION_ERROR: c_uint = 0x80;
//
// Flash register : DOC_DPS0_STATUS and DOC_DPS1_STATUS
//
pub const DOC_DPS_OTP_PROTECTED: c_uint = 0x01;
pub const DOC_DPS_READ_PROTECTED: c_uint = 0x02;
pub const DOC_DPS_WRITE_PROTECTED: c_uint = 0x04;
pub const DOC_DPS_HW_LOCK_ENABLED: c_uint = 0x08;
pub const DOC_DPS_KEY_OK: c_uint = 0x80;
//
// Flash register : DOC_CONFIGURATION
//
pub const DOC_CONF_IF_CFG: c_uint = 0x80;
pub const DOC_CONF_MAX_ID_MASK: c_uint = 0x30;
pub const DOC_CONF_VCCQ_3V: c_uint = 0x01;
//
// Flash register : DOC_READADDRESS
//
pub const DOC_READADDR_INC: c_uint = 0x8000;
pub const DOC_READADDR_ONE_BYTE: c_uint = 0x4000;
pub const DOC_READADDR_ADDR_MASK: c_uint = 0x1fff;
//
// Flash register : DOC_POWERMODE
//
pub const DOC_POWERDOWN_READY: c_uint = 0x80;
//
// Status of erase and write operation
//
pub const DOC_PLANES_STATUS_FAIL: c_uint = 0x01;
pub const DOC_PLANES_STATUS_PLANE0_KO: c_uint = 0x02;
pub const DOC_PLANES_STATUS_PLANE1_KO: c_uint = 0x04;
//
// DPS key management
//
// Each floor of docg3 has 2 protection areas: DPS0 and DPS1. These areas span
// across block boundaries, and define whether these blocks can be read or
// written.
// The definition is dynamically stored in page 0 of blocks (2,3) for DPS0, and
// page 0 of blocks (4,5) for DPS1.
//
pub const DOC_LAYOUT_DPS_KEY_LENGTH: c_int = 8;
//
// struct docg3_cascade - Cascade of 1 to 4 docg3 chips
// @floors: floors (ie. one physical docg3 chip is one floor)
// @base: IO space to access all chips in the cascade
// @bch: the BCH correcting control structure
// @lock: lock to protect docg3 IO space from concurrent accesses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct docg3_cascade {
    pub floors: [*mut mtd_info; DOC_MAX_NBFLOORS],
    pub base: *mut void __iomem,
    pub bch: *mut bch_control,
    pub lock: mutex,
}

//
// struct docg3 - DiskOnChip driver private data
// @dev: the device currently under control
// @cascade: the cascade this device belongs to
// @device_id: number of the cascaded DoCG3 device (0, 1, 2 or 3)
// @if_cfg: if true, reads are on 16bits, else reads are on 8bits
// @reliable: if 0, docg3 in normal mode, if 1 docg3 in fast mode, if 2 in
// reliable mode
// Fast mode implies more errors than normal mode.
// Reliable mode implies that page 2*n and 2*n+1 are clones.
// @max_block: maximum block number for this device
// @bbt: bad block table cache
// @oob_write_ofs: offset of the MTD where this OOB should belong (ie. in next
// page_write)
// @oob_autoecc: if 1, use only bytes 0-7, 15, and fill the others with HW ECC
// if 0, use all the 16 bytes.
// @oob_write_buf: prepared OOB for next page_write
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct docg3 {
    pub dev: *mut device,
    pub cascade: *mut docg3_cascade,
    pub device_id:4: c_uint,
    pub if_cfg:1: c_uint,
    pub reliable:2: c_uint,
    pub max_block: c_int,
    pub bbt: *mut u8,
    pub oob_write_ofs: loff_t,
    pub oob_autoecc: c_int,
    pub oob_write_buf: [u8; DOC_LAYOUT_OOB_SIZE],
}

//
// Trace events part
//

// This part must be outside protection

