//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/mtd/mtd-abi.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org> et al.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erase_info_user {
    pub start: __u32,
    pub length: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erase_info_user64 {
    pub start: __u64,
    pub length: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_oob_buf {
    pub start: __u32,
    pub length: __u32,
    pub ptr: *mut unsigned char __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_oob_buf64 {
    pub start: __u64,
    pub pad: __u32,
    pub length: __u32,
    pub usr_ptr: __u64,
}

//
// MTD operation modes
//
// @MTD_OPS_PLACE_OOB:	OOB data are placed at the given offset (default)
// @MTD_OPS_AUTO_OOB:	OOB data are automatically placed at the free areas
// which are defined by the internal ecclayout
// @MTD_OPS_RAW:	data are transferred as-is, with no error correction;
// this mode implies %MTD_OPS_PLACE_OOB
//
// These modes can be passed to ioctl(MEMWRITE) and ioctl(MEMREAD); they are
// also used internally. See notes on "MTD file modes" for discussion on
// %MTD_OPS_RAW vs. %MTD_FILE_MODE_RAW.
//
// struct mtd_write_req - data structure for requesting a write operation
//
// @start:	start address
// @len:	length of data buffer (only lower 32 bits are used)
// @ooblen:	length of OOB buffer (only lower 32 bits are used)
// @usr_data:	user-provided data buffer
// @usr_oob:	user-provided OOB buffer
// @mode:	MTD mode (see "MTD operation modes")
// @padding:	reserved, must be set to 0
//
// This structure supports ioctl(MEMWRITE) operations, allowing data and/or OOB
// writes in various modes. To write to OOB-only, set @usr_data == NULL, and to
// write data-only, set @usr_oob == NULL. However, setting both @usr_data and
// @usr_oob to NULL is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_write_req {
    pub start: __u64,
    pub len: __u64,
    pub ooblen: __u64,
    pub usr_data: __u64,
    pub usr_oob: __u64,
    pub mode: __u8,
    pub padding: [__u8; 7],
}

//
// struct mtd_read_req_ecc_stats - ECC statistics for a read operation
//
// @uncorrectable_errors: the number of uncorrectable errors that happened
// during the read operation
// @corrected_bitflips: the number of bitflips corrected during the read
// operation
// @max_bitflips: the maximum number of bitflips detected in any single ECC
// step for the data read during the operation; this information
// can be used to decide whether the data stored in a specific
// region of the MTD device should be moved somewhere else to
// avoid data loss.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_read_req_ecc_stats {
    pub uncorrectable_errors: __u32,
    pub corrected_bitflips: __u32,
    pub max_bitflips: __u32,
}

//
// struct mtd_read_req - data structure for requesting a read operation
//
// @start:	start address
// @len:	length of data buffer (only lower 32 bits are used)
// @ooblen:	length of OOB buffer (only lower 32 bits are used)
// @usr_data:	user-provided data buffer
// @usr_oob:	user-provided OOB buffer
// @mode:	MTD mode (see "MTD operation modes")
// @padding:	reserved, must be set to 0
// @ecc_stats:	ECC statistics for the read operation
//
// This structure supports ioctl(MEMREAD) operations, allowing data and/or OOB
// reads in various modes. To read from OOB-only, set @usr_data == NULL, and to
// read data-only, set @usr_oob == NULL. However, setting both @usr_data and
// @usr_oob to NULL is not allowed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_read_req {
    pub start: __u64,
    pub len: __u64,
    pub ooblen: __u64,
    pub usr_data: __u64,
    pub usr_oob: __u64,
    pub mode: __u8,
    pub padding: [__u8; 7],
    pub ecc_stats: mtd_read_req_ecc_stats,
}

pub const MTD_ABSENT: c_int = 0;
pub const MTD_RAM: c_int = 1;
pub const MTD_ROM: c_int = 2;
pub const MTD_NORFLASH: c_int = 3;

pub const MTD_DATAFLASH: c_int = 6;
pub const MTD_UBIVOLUME: c_int = 7;

pub const MTD_WRITEABLE: c_uint = 0x400	/* Device is writeable */;
pub const MTD_BIT_WRITEABLE: c_uint = 0x800	/* Single bits can be flipped */;
pub const MTD_NO_ERASE: c_uint = 0x1000	/* No erase necessary */;
pub const MTD_POWERUP_LOCK: c_uint = 0x2000	/* Always locked after reset */;
pub const MTD_SLC_ON_MLC_EMULATION: c_uint = 0x4000	/* Emulate SLC behavior on MLC NANDs */;
// Some common devices / combinations of capabilities
pub const MTD_CAP_ROM: c_int = 0;

// Obsolete ECC byte placement modes (used with obsolete MEMGETOOBSEL)

// OTP mode selection
pub const MTD_OTP_OFF: c_int = 0;
pub const MTD_OTP_FACTORY: c_int = 1;
pub const MTD_OTP_USER: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_info_user {
    pub type: __u8,
    pub flags: __u32,
    pub /: *mut *mut __u32 size; / Total size of the MTD,
    pub erasesize: __u32,
    pub writesize: __u32,
    pub /: *mut *mut __u32 oobsize; / Amount of OOB data per block (e.g. 16),
    pub /: *mut *mut __u64 padding; / Old obsolete field; do not use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region_info_user {
    pub starts,: *mut *mut __u32 offset; / At which this region,
// from the beginning of the MTD
    pub /: *mut *mut __u32 erasesize; / For this region,
    pub /: *mut *mut __u32 numblocks; / Number of blocks in this region,
    pub regionindex: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otp_info {
    pub start: __u32,
    pub length: __u32,
    pub locked: __u32,
}

//
// Note, the following ioctl existed in the past and was removed:
// #define MEMSETOOBSEL           _IOW('M', 9, struct nand_oobinfo)
// Try to avoid adding a new ioctl with the same ioctl number.
//
// Get basic MTD characteristics info (better to use sysfs)

// Erase segment of MTD

// Write out-of-band data from MTD

// Read out-of-band data from MTD

// Lock a chip (for MTD that supports it)

// Unlock a chip (for MTD that supports it)

// Get the number of different erase regions

// Get information about the erase region for a specific index

// Get info about OOB modes (e.g., RAW, PLACE, AUTO) - legacy interface

// Check if an eraseblock is bad

// Mark an eraseblock as bad

// Set OTP (One-Time Programmable) mode (factory vs. user)

// Get number of OTP (One-Time Programmable) regions

// Get all OTP (One-Time Programmable) info about MTD

// Lock a given range of user data (must be in mode %MTD_FILE_MODE_OTP_USER)

// Get ECC layout (deprecated)

// Get statistics about corrected/uncorrected errors

// Set MTD mode on a per-file-descriptor basis (see "MTD file modes")

// Erase segment of MTD (supports 64-bit address)

// Write data to OOB (64-bit version)

// Read data from OOB (64-bit version)

// Check if chip is locked (for MTD that supports it)

//
// Most generic write interface; can write in-band and/or out-of-band in various
// modes (see "struct mtd_write_req"). This ioctl is not supported for flashes
// without OOB, e.g., NOR flash.
//

// Erase a given range of user data (must be in mode %MTD_FILE_MODE_OTP_USER)

//
// Most generic read interface; can read in-band and/or out-of-band in various
// modes (see "struct mtd_read_req"). This ioctl is not supported for flashes
// without OOB, e.g., NOR flash.
//

//
// Obsolete legacy interface. Keep it in order not to break userspace
// interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_oobinfo {
    pub useecc: __u32,
    pub eccbytes: __u32,
    pub oobfree: [__u32; 8][2],
    pub eccpos: [__u32; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_oobfree {
    pub offset: __u32,
    pub length: __u32,
}

pub const MTD_MAX_OOBFREE_ENTRIES: c_int = 8;
pub const MTD_MAX_ECCPOS_ENTRIES: c_int = 64;
//
// OBSOLETE: ECC layout control structure. Exported to user-space via ioctl
// ECCGETLAYOUT for backwards compatbility and should not be mistaken as a
// complete set of ECC information. The ioctl truncates the larger internal
// structure to retain binary compatibility with the static declaration of the
// ioctl. Note that the "MTD_MAX_..._ENTRIES" macros represent the max size of
// the user struct, not the MAX size of the internal OOB layout representation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecclayout_user {
    pub eccbytes: __u32,
    pub eccpos: [__u32; MTD_MAX_ECCPOS_ENTRIES],
    pub oobavail: __u32,
    pub oobfree: [nand_oobfree; MTD_MAX_OOBFREE_ENTRIES],
}

//
// struct mtd_ecc_stats - error correction stats
//
// @corrected:	number of corrected bits
// @failed:	number of uncorrectable errors
// @badblocks:	number of bad blocks in this partition
// @bbtblocks:	number of blocks reserved for bad block tables
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_ecc_stats {
    pub corrected: __u32,
    pub failed: __u32,
    pub badblocks: __u32,
    pub bbtblocks: __u32,
}

//
// MTD file modes - for read/write access to MTD
//
// @MTD_FILE_MODE_NORMAL:	OTP disabled, ECC enabled
// @MTD_FILE_MODE_OTP_FACTORY:	OTP enabled in factory mode
// @MTD_FILE_MODE_OTP_USER:	OTP enabled in user mode
// @MTD_FILE_MODE_RAW:		OTP disabled, ECC disabled
//
// These modes can be set via ioctl(MTDFILEMODE). The mode will be retained
// separately for each open file descriptor.
//
// Note: %MTD_FILE_MODE_RAW provides the same functionality as %MTD_OPS_RAW -
// raw access to the flash, without error correction or autoplacement schemes.
// Wherever possible, the MTD_OPS_* mode will override the MTD_FILE_MODE_* mode
// (e.g., when using ioctl(MEMWRITE) or ioctl(MEMREAD)), but in some cases, the
// MTD_FILE_MODE is used out of necessity (e.g., `write()',
// ioctl(MEMWRITEOOB64)).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtd_file_modes {
    MTD_FILE_MODE_NORMAL = MTD_OTP_OFF,
    MTD_FILE_MODE_OTP_FACTORY = MTD_OTP_FACTORY,
    MTD_FILE_MODE_OTP_USER = MTD_OTP_USER,
    MTD_FILE_MODE_RAW,
}
