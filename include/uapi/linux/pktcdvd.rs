//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pktcdvd.h
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
// Copyright (C) 2000 Jens Axboe <axboe@suse.de>
// Copyright (C) 2001-2004 Peter Osterlund <petero2@telia.com>
//
// May be copied or modified under the terms of the GNU General Public
// License.  See linux/COPYING for more information.
//
// Packet writing layer for ATAPI and SCSI CD-R, CD-RW, DVD-R, and
// DVD-RW devices.
//

//
// UNUSED:
// 1 for normal debug messages, 2 is very verbose. 0 to turn it off.
//
pub const PACKET_DEBUG: c_int = 1;
pub const MAX_WRITERS: c_int = 8;
pub const PKT_RB_POOL_SIZE: c_int = 512;
//
// How long we should hold a non-full packet before starting data gathering.
//

//
// No user-servicable parts beyond this point ->
//
// device types
//
pub const PACKET_CDR: c_int = 1;
pub const PACKET_CDRW: c_int = 2;
pub const PACKET_DVDR: c_int = 3;
pub const PACKET_DVDRW: c_int = 4;
//
// flags
//

// underlying cdrom device happy
//
// Disc status -- from READ_DISC_INFO
//
pub const PACKET_DISC_EMPTY: c_int = 0;
pub const PACKET_DISC_INCOMPLETE: c_int = 1;
pub const PACKET_DISC_COMPLETE: c_int = 2;
pub const PACKET_DISC_OTHER: c_int = 3;
//
// write type, and corresponding data block type
//
pub const PACKET_MODE1: c_int = 1;
pub const PACKET_MODE2: c_int = 2;
pub const PACKET_BLOCK_MODE1: c_int = 8;
pub const PACKET_BLOCK_MODE2: c_int = 10;
//
// Last session/border status
//
pub const PACKET_SESSION_EMPTY: c_int = 0;
pub const PACKET_SESSION_INCOMPLETE: c_int = 1;
pub const PACKET_SESSION_RESERVED: c_int = 2;
pub const PACKET_SESSION_COMPLETE: c_int = 3;

pub const PKT_CTRL_CMD_SETUP: c_int = 0;
pub const PKT_CTRL_CMD_TEARDOWN: c_int = 1;
pub const PKT_CTRL_CMD_STATUS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pkt_ctrl_command {
    pub /: *mut *mut __u32 command; / in: Setup, teardown, status,
    pub /: *mut *mut __u32 dev_index; / in/out: Device index,
    pub /: *mut *mut __u32 dev; / in/out: Device nr for cdrw device,
    pub /: *mut *mut __u32 pkt_dev; / in/out: Device nr for packet device,
    pub /: *mut *mut __u32 num_devices; / out: Largest device index + 1,
    pub /: *mut *mut __u32 padding; / Not used,
}

//
// packet ioctls
//

