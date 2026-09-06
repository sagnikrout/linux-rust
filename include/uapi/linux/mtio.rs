//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mtio.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// linux/mtio.h header file for Linux. Written by H. Bergman
//
// Modified for special ioctls provided by zftape in September 1997
// by C.-J. Heine.
//

//
// Structures and definitions for mag tape io control commands
//
// structure for MTIOCTOP - mag tape op command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtop {
    pub /: *mut *mut short mt_op; / operations defined below,
    pub /: *mut *mut int mt_count; / how many of them,
}

// Magnetic Tape operations [Not all operations supported by all drivers]:

// position at first record of next file
//

// MTEOM positions after the last FM, ready for
// appending another file.
//

// ordinary buffered operation with code 1

// structure for MTIOCGET - mag tape get status command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtget {
    pub /: *mut *mut long mt_type; / type of magtape device,
    pub sure): *mut *mut long mt_resid; / residual count: (not,
// number of bytes ignored, or
// number of files not skipped, or
// number of records not skipped.
//
// the following registers are device dependent
    pub /: *mut *mut long mt_dsreg; / status register,
    pub /: *mut *mut long mt_gstat; / generic (device independent) status,
    pub /: *mut *mut long mt_erreg; / error register,
// The next two fields are not always used
    pub /: *mut *mut __kernel_daddr_t mt_fileno; / number of current file on tape,
    pub /: *mut *mut __kernel_daddr_t mt_blkno; / current block number,
}

//
// Constants for mt_type. Not all of these are supported,
// and these are not all of the ones that are supported.
//
pub const MT_ISUNKNOWN: c_uint = 0x01;
pub const MT_ISQIC02: c_uint = 0x02	/* Generic QIC-02 tape streamer */;
pub const MT_ISWT5150: c_uint = 0x03	/* Wangtek 5150EQ, QIC-150, QIC-02 */;
pub const MT_ISARCHIVE_5945L2: c_uint = 0x04	/* Archive 5945L-2, QIC-24, QIC-02? */;
pub const MT_ISCMSJ500: c_uint = 0x05	/* CMS Jumbo 500 (QIC-02?) */;
pub const MT_ISTDC3610: c_uint = 0x06	/* Tandberg 6310, QIC-24 */;
pub const MT_ISARCHIVE_VP60I: c_uint = 0x07	/* Archive VP60i, QIC-02 */;
pub const MT_ISARCHIVE_2150L: c_uint = 0x08	/* Archive Viper 2150L */;
pub const MT_ISARCHIVE_2060L: c_uint = 0x09	/* Archive Viper 2060L */;
pub const MT_ISARCHIVESC499: c_uint = 0x0A	/* Archive SC-499 QIC-36 controller */;
pub const MT_ISQIC02_ALL_FEATURES: c_uint = 0x0F	/* Generic QIC-02 with all features */;
pub const MT_ISWT5099EEN24: c_uint = 0x11	/* Wangtek 5099-een24, 60MB, QIC-24 */;
pub const MT_ISTEAC_MT2ST: c_uint = 0x12	/* Teac MT-2ST 155mb drive, Teac DC-1 card (Wangtek type) */;
pub const MT_ISEVEREX_FT40A: c_uint = 0x32	/* Everex FT40A (QIC-40) */;
pub const MT_ISDDS1: c_uint = 0x51	/* DDS device without partitions */;
pub const MT_ISDDS2: c_uint = 0x52	/* DDS device with partitions */;
pub const MT_ISONSTREAM_SC: c_uint = 0x61   /* OnStream SCSI tape drives (SC-x0);
pub const MT_ISSCSI1: c_uint = 0x71	/* Generic ANSI SCSI-1 tape unit */;
pub const MT_ISSCSI2: c_uint = 0x72	/* Generic ANSI SCSI-2 tape unit */;
// QIC-40/80/3010/3020 ftape supported drives.
// 20bit vendor ID + 0x800000 (see ftape-vendors.h)
//
pub const MT_ISFTAPE_UNKNOWN: c_uint = 0x800000 /* obsolete */;
pub const MT_ISFTAPE_FLAG: c_uint = 0x800000;
// structure for MTIOCPOS - mag tape get position command
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtpos {
    pub /: *mut *mut long mt_blkno; / current block number,
}

// mag tape io control commands

// Generic Mag Tape (device independent) status macros for examining
// mt_gstat -- HP-UX compatible.
// There is room for more generic status bits here, but I don't
// know which of them are reserved. At least three or so should
// be added to make this really useful.
//

// #define GMT_ ? 		((x) & 0x02000000)

// #define GMT_ ? 		((x) & 0x00100000)
// #define GMT_ ? 		((x) & 0x00080000)

// #define GMT_ ? 		((x) & 0x00020000)

// 15 generic status bits unused
// SCSI-tape specific definitions
// Bitfield shifts in the status
pub const MT_ST_BLKSIZE_SHIFT: c_int = 0;
pub const MT_ST_BLKSIZE_MASK: c_uint = 0xffffff;
pub const MT_ST_DENSITY_SHIFT: c_int = 24;
pub const MT_ST_DENSITY_MASK: c_uint = 0xff000000;
pub const MT_ST_SOFTERR_SHIFT: c_int = 0;
pub const MT_ST_SOFTERR_MASK: c_uint = 0xffff;
// Bitfields for the MTSETDRVBUFFER ioctl
pub const MT_ST_OPTIONS: c_uint = 0xf0000000;
pub const MT_ST_BOOLEANS: c_uint = 0x10000000;
pub const MT_ST_SETBOOLEANS: c_uint = 0x30000000;
pub const MT_ST_CLEARBOOLEANS: c_uint = 0x40000000;
pub const MT_ST_WRITE_THRESHOLD: c_uint = 0x20000000;
pub const MT_ST_DEF_BLKSIZE: c_uint = 0x50000000;
pub const MT_ST_DEF_OPTIONS: c_uint = 0x60000000;
pub const MT_ST_TIMEOUTS: c_uint = 0x70000000;

pub const MT_ST_SET_CLN: c_uint = 0x80000000;
pub const MT_ST_BUFFER_WRITES: c_uint = 0x1;
pub const MT_ST_ASYNC_WRITES: c_uint = 0x2;
pub const MT_ST_READ_AHEAD: c_uint = 0x4;
pub const MT_ST_DEBUGGING: c_uint = 0x8;
pub const MT_ST_TWO_FM: c_uint = 0x10;
pub const MT_ST_FAST_MTEOM: c_uint = 0x20;
pub const MT_ST_AUTO_LOCK: c_uint = 0x40;
pub const MT_ST_DEF_WRITES: c_uint = 0x80;
pub const MT_ST_CAN_BSR: c_uint = 0x100;
pub const MT_ST_NO_BLKLIMS: c_uint = 0x200;
pub const MT_ST_CAN_PARTITIONS: c_uint = 0x400;
pub const MT_ST_SCSI2LOGICAL: c_uint = 0x800;
pub const MT_ST_SYSV: c_uint = 0x1000;
pub const MT_ST_NOWAIT: c_uint = 0x2000;
pub const MT_ST_SILI: c_uint = 0x4000;
pub const MT_ST_NOWAIT_EOF: c_uint = 0x8000;
// The mode parameters to be controlled. Parameter chosen with bits 20-28
pub const MT_ST_CLEAR_DEFAULT: c_uint = 0xfffff;

// The offset for the arguments for the special HP changer load command.
pub const MT_ST_HPLOADER_OFFSET: c_int = 10000;
