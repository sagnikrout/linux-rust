//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/onenand.h
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
// linux/include/linux/mtd/onenand.h
//
// Copyright © 2005-2009 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
//

pub const MAX_DIES: c_int = 2;
pub const MAX_BUFFERRAM: c_int = 2;
// Scan and identify a OneNAND device
extern "C" {
    pub fn onenand_scan(mtd: *mut mtd_info, max_chips: c_int) -> c_int;
}
// Free resources held by the OneNAND device
extern "C" {
    pub fn onenand_release(mtd: *mut mtd_info);
}
//
// struct onenand_bufferram - OneNAND BufferRAM Data
// @blockpage:		block & page address in BufferRAM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onenand_bufferram {
    pub blockpage: c_int,
}

//
// struct onenand_chip - OneNAND Private Flash Chip Data
// @base:		[BOARDSPECIFIC] address to access OneNAND
// @dies:		[INTERN][FLEX-ONENAND] number of dies on chip
// @boundary:		[INTERN][FLEX-ONENAND] Boundary of the dies
// @diesize:		[INTERN][FLEX-ONENAND] Size of the dies
// @chipsize:		[INTERN] the size of one chip for multichip arrays
// FIXME For Flex-OneNAND, chipsize holds maximum possible
// device size ie when all blocks are considered MLC
// @device_id:		[INTERN] device ID
// @density_mask:	chip density, used for DDP devices
// @verstion_id:	[INTERN] version ID
// @options:		[BOARDSPECIFIC] various chip options. They can
// partly be set to inform onenand_scan about
// @erase_shift:	[INTERN] number of address bits in a block
// @page_shift:		[INTERN] number of address bits in a page
// @page_mask:		[INTERN] a page per block mask
// @writesize:		[INTERN] a real page size
// @bufferram_index:	[INTERN] BufferRAM index
// @bufferram:		[INTERN] BufferRAM info
// @readw:		[REPLACEABLE] hardware specific function for read short
// @writew:		[REPLACEABLE] hardware specific function for write short
// @command:		[REPLACEABLE] hardware specific function for writing
// commands to the chip
// @wait:		[REPLACEABLE] hardware specific function for wait on ready
// @bbt_wait:		[REPLACEABLE] hardware specific function for bbt wait on ready
// @unlock_all:		[REPLACEABLE] hardware specific function for unlock all
// @read_bufferram:	[REPLACEABLE] hardware specific function for BufferRAM Area
// @write_bufferram:	[REPLACEABLE] hardware specific function for BufferRAM Area
// @read_word:		[REPLACEABLE] hardware specific function for read
// register of OneNAND
// @write_word:		[REPLACEABLE] hardware specific function for write
// register of OneNAND
// @mmcontrol:		sync burst read function
// @chip_probe:		[REPLACEABLE] hardware specific function for chip probe
// @block_markbad:	function to mark a block as bad
// @scan_bbt:		[REPLACEALBE] hardware specific function for scanning
// Bad block Table
// @chip_lock:		[INTERN] spinlock used to protect access to this
// structure and the chip
// @wq:			[INTERN] wait queue to sleep on if a OneNAND
// operation is in progress
// @state:		[INTERN] the current state of the OneNAND device
// @page_buf:		[INTERN] page main data buffer
// @oob_buf:		[INTERN] page oob data buffer
// @subpagesize:	[INTERN] holds the subpagesize
// @bbm:		[REPLACEABLE] pointer to Bad Block Management
// @priv:		[OPTIONAL] pointer to private chip date
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onenand_chip {
    pub base: *mut void __iomem,
    pub dies: unsigned,
    pub boundary: [unsigned; MAX_DIES],
    pub diesize: [loff_t; MAX_DIES],
    pub chipsize: c_uint,
    pub device_id: c_uint,
    pub version_id: c_uint,
    pub technology: c_uint,
    pub density_mask: c_uint,
    pub options: c_uint,
    pub badblockpos: c_uint,
    pub erase_shift: c_uint,
    pub page_shift: c_uint,
    pub page_mask: c_uint,
    pub writesize: c_uint,
    pub bufferram_index: c_uint,
    pub bufferram: [onenand_bufferram; MAX_BUFFERRAM],
    pub len): *mut *mut *mut int (command)(struct mtd_info mtd, int cmd, loff_t address, size_t,
    pub state): *mut *mut *mut int (wait)(struct mtd_info mtd, int,
    pub state): *mut *mut *mut int (bbt_wait)(struct mtd_info mtd, int,
    pub mtd): *mut *mut void (unlock_all)(struct mtd_info,
    pub count): *mut *mut unsigned char buffer, int offset, size_t,
    pub count): *const *const unsigned char buffer, int offset, size_t,
    pub addr): *mut *mut unsigned short (read_word)(void __iomem,
    pub addr): *mut *mut void (write_word)(unsigned short value, void __iomem,
    pub sync_read): *mut *mut *mut void (mmcontrol)(struct mtd_info mtd, int,
    pub mtd): *mut *mut int (chip_probe)(struct mtd_info,
    pub ofs): *mut *mut *mut int (block_markbad)(struct mtd_info mtd, loff_t,
    pub mtd): *mut *mut int (scan_bbt)(struct mtd_info,
    pub mtd): *mut *mut int (enable)(struct mtd_info,
    pub mtd): *mut *mut int (disable)(struct mtd_info,
    pub complete: completion,
    pub irq: c_int,
    pub chip_lock: spinlock_t,
    pub wq: wait_queue_head_t,
    pub state: flstate_t,
    pub page_buf: *mut c_uchar,
    pub oob_buf: *mut c_uchar,

    pub verify_buf: *mut c_uchar,

    pub subpagesize: c_int,
    pub bbm: *mut c_void,
    pub priv: *mut c_void,
//
// Shows that the current operation is composed
// of sequence of commands. For example, cache program.
// Such command status OnGo bit is checked at the end of
// sequence.
//
    pub ongoing: c_uint,
}

//
// Helper macros
//

// Check byte access in OneNAND

pub const ONENAND_BADBLOCK_POS: c_int = 0;
//
// Options bits
//

//
// OneNAND Flash Manufacturer ID Codes
//
pub const ONENAND_MFR_SAMSUNG: c_uint = 0xec;
pub const ONENAND_MFR_NUMONYX: c_uint = 0x20;
//
// struct onenand_manufacturers - NAND Flash Manufacturer ID Structure
// @name:	Manufacturer name
// @id:		manufacturer ID code of device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onenand_manufacturers {
    pub id: c_int,
    pub name: *mut c_char,
}

extern "C" {
    pub fn onenand_block(this: *mut onenand_chip, addr: loff_t) -> unsigned;
}
extern "C" {
    pub fn onenand_addr(this: *mut onenand_chip, block: c_int) -> loff_t;
}
extern "C" {
    pub fn flexonenand_region(mtd: *mut mtd_info, addr: loff_t) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct onenand_platform_data {
    pub sync_read): *mut *mut *mut void (mmcontrol)(struct mtd_info mtd, int,
    pub count): *mut *mut unsigned char buffer, int offset, size_t,
    pub parts: *mut mtd_partition,
    pub nr_parts: c_uint,
}
