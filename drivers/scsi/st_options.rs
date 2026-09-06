//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/st_options.h
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
// If TRY_DIRECT_IO is non-zero, the driver tries to transfer data directly
pub const TRY_DIRECT_IO: c_int = 1;
// The driver does not wait for some operations to finish before returning
pub const ST_NOWAIT: c_int = 0;
// If ST_IN_FILE_POS is nonzero, the driver positions the tape after the
pub const ST_IN_FILE_POS: c_int = 0;
// If ST_RECOVERED_WRITE_FATAL is non-zero, recovered errors while writing
pub const ST_RECOVERED_WRITE_FATAL: c_int = 0;
// The "guess" for the block size for devices that don't support MODE
pub const ST_DEFAULT_BLOCK: c_int = 0;
// The minimum tape driver buffer size in kilobytes in fixed block mode.
pub const ST_FIXED_BUFFER_BLOCKS: c_int = 32;
// Maximum number of scatter/gather segments
pub const ST_MAX_SG: c_int = 256;
// The number of scatter/gather segments to allocate at first try (must be
pub const ST_FIRST_SG: c_int = 8;
// The size of the first scatter/gather segments (determines the maximum block
pub const ST_FIRST_ORDER: c_int = 5;
// The following lines define defaults for properties that can be set
// If ST_TWO_FM is non-zero, the driver writes two filemarks after a
pub const ST_TWO_FM: c_int = 0;
// If ST_BUFFER_WRITES is non-zero, writes in fixed block mode are
pub const ST_BUFFER_WRITES: c_int = 1;
// If ST_ASYNC_WRITES is non-zero, the SCSI write command may be started
pub const ST_ASYNC_WRITES: c_int = 1;
// If ST_READ_AHEAD is non-zero, blocks are read ahead in fixed block
pub const ST_READ_AHEAD: c_int = 1;
// If ST_AUTO_LOCK is non-zero, the drive door is locked at the first
pub const ST_AUTO_LOCK: c_int = 0;
// If ST_FAST_MTEOM is non-zero, the MTEOM ioctl is done using the
pub const ST_FAST_MTEOM: c_int = 0;
// If ST_SCSI2LOGICAL is nonzero, the logical block addresses are used for
pub const ST_SCSI2LOGICAL: c_int = 0;
// If ST_SYSV is non-zero, the tape behaves according to the SYS V semantics.
pub const ST_SYSV: c_int = 0;
// If ST_SILI is non-zero, the SILI bit is set when reading in variable block
pub const ST_SILI: c_int = 0;
// Time to wait for the drive to become ready if blocking open
pub const ST_BLOCK_SECONDS: c_int = 120;
