//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/bbm.h
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
// NAND family Bad Block Management (BBM) header file
// - Bad Block Table (BBT) implementation
//
// Copyright © 2005 Samsung Electronics
// Kyungmin Park <kyungmin.park@samsung.com>
//
// Copyright © 2000-2005
// Thomas Gleixner <tglx@linuxtronix.de>
//
// The maximum number of NAND chips in an array
pub const NAND_MAX_CHIPS: c_int = 8;
//
// struct nand_bbt_descr - bad block table descriptor
// @options:	options for this descriptor
// @pages:	the page(s) where we find the bbt, used with option BBT_ABSPAGE
// when bbt is searched, then we store the found bbts pages here.
// Its an array and supports up to 8 chips now
// @offs:	offset of the pattern in the oob area of the page
// @veroffs:	offset of the bbt version counter in the oob are of the page
// @version:	version read from the bbt page during scan
// @len:	length of the pattern, if 0 no pattern check is performed
// @maxblocks:	maximum number of blocks to search for a bbt. This number of
// blocks is reserved at the end of the device where the tables are
// written.
// @reserved_block_code: if non-0, this pattern denotes a reserved (rather than
// bad) block in the stored bbt
// @pattern:	pattern to identify bad block table or factory marked good
// bad blocks, can be NULL, if len = 0
//
// Descriptor for the bad block table marker and the descriptor for the
// pattern which identifies good and bad blocks. The assumption is made
// that the pattern and the version count are always located in the oob area
// of the first block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_bbt_descr {
    pub options: c_int,
    pub pages: [c_int; NAND_MAX_CHIPS],
    pub offs: c_int,
    pub veroffs: c_int,
    pub version: [u8; NAND_MAX_CHIPS],
    pub len: c_int,
    pub maxblocks: c_int,
    pub reserved_block_code: c_int,
    pub pattern: *mut u8,
}

// Options for the bad block table descriptors
// The number of bits used per block in the bbt on the device
pub const NAND_BBT_NRBITS_MSK: c_uint = 0x0000000F;
pub const NAND_BBT_1BIT: c_uint = 0x00000001;
pub const NAND_BBT_2BIT: c_uint = 0x00000002;
pub const NAND_BBT_4BIT: c_uint = 0x00000004;
pub const NAND_BBT_8BIT: c_uint = 0x00000008;
// The bad block table is in the last good block of the device
pub const NAND_BBT_LASTBLOCK: c_uint = 0x00000010;
// The bbt is at the given page, else we must scan for the bbt
pub const NAND_BBT_ABSPAGE: c_uint = 0x00000020;
// bbt is stored per chip on multichip devices
pub const NAND_BBT_PERCHIP: c_uint = 0x00000080;
// bbt has a version counter at offset veroffs
pub const NAND_BBT_VERSION: c_uint = 0x00000100;
// Create a bbt if none exists
pub const NAND_BBT_CREATE: c_uint = 0x00000200;
//
// Create an empty BBT with no vendor information. Vendor's information may be
// unavailable, for example, if the NAND controller has a different data and OOB
// layout or if this information is already purged. Must be used in conjunction
// with NAND_BBT_CREATE.
//
pub const NAND_BBT_CREATE_EMPTY: c_uint = 0x00000400;
// Write bbt if neccecary
pub const NAND_BBT_WRITE: c_uint = 0x00002000;
// Read and write back block contents when writing bbt
pub const NAND_BBT_SAVECONTENT: c_uint = 0x00004000;
//
// Use a flash based bad block table. By default, OOB identifier is saved in
// OOB area. This option is passed to the default bad block table function.
//
pub const NAND_BBT_USE_FLASH: c_uint = 0x00020000;
//
// Do not store flash based bad block table marker in the OOB area; store it
// in-band.
//
pub const NAND_BBT_NO_OOB: c_uint = 0x00040000;
//
// Do not write new bad block markers to OOB; useful, e.g., when ECC covers
// entire spare area. Must be used with NAND_BBT_USE_FLASH.
//
pub const NAND_BBT_NO_OOB_BBM: c_uint = 0x00080000;
//
// Flag set by nand_create_default_bbt_descr(), marking that the nand_bbt_descr
// was allocated dynamicaly and must be freed in nand_cleanup(). Has no meaning
// in nand_chip.bbt_options.
//
pub const NAND_BBT_DYNAMICSTRUCT: c_uint = 0x80000000;
// The maximum number of blocks to scan for a bbt
pub const NAND_BBT_SCAN_MAXBLOCKS: c_int = 4;
//
// Bad block scanning errors
//
pub const ONENAND_BBT_READ_ERROR: c_int = 1;
pub const ONENAND_BBT_READ_ECC_ERROR: c_int = 2;
pub const ONENAND_BBT_READ_FATAL_ERROR: c_int = 4;
//
// struct bbm_info - [GENERIC] Bad Block Table data structure
// @bbt_erase_shift:	[INTERN] number of address bits in a bbt entry
// @options:		options for this descriptor
// @bbt:		[INTERN] bad block table pointer
// @isbad_bbt:		function to determine if a block is bad
// @badblock_pattern:	[REPLACEABLE] bad block scan pattern used for
// initial bad block scan
// @priv:		[OPTIONAL] pointer to private bbm date
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bbm_info {
    pub bbt_erase_shift: c_int,
    pub options: c_int,
    pub bbt: *mut u8,
    pub allowbbt): *mut *mut *mut int (isbad_bbt)(struct mtd_info mtd, loff_t ofs, int,
// TODO Add more NAND specific fileds
    pub badblock_pattern: *mut nand_bbt_descr,
    pub priv: *mut c_void,
}

// OneNAND BBT interface
extern "C" {
    pub fn onenand_default_bbt(mtd: *mut mtd_info) -> c_int;
}
