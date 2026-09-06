//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/drbd_limits.h
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


// SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note
//
// Our current limitations.
// Some of them are hard limits,
// some of them are arbitrary range limits, that make it easier to provide
// feedback about nonsense settings for certain configurable values.
//

// valid port number

pub const DRBD_PORT_MAX: c_uint = 0xffffU;

// startup {
// if you want more than 3.4 days, disable

// }
// net {
// timeout, unit centi seconds
// more than one minute timeout is not useful

// If backing disk takes longer than disk_timeout, mark the disk as failed

// active connection retries when C_WF_CONNECTION

// keep-alive probes when idle

// timeout for the ping packets.

// max number of write requests between write barriers

// I don't think that a tcp send buffer of more than 10M is useful

// @4k PageSize -> 128kB - 512MB

// @4k PageSize -> 4kB - 512MB

// 0 is disabled.
// 200 should be more than enough even for very short timeouts

// }
// syncer {
// FIXME allow rate to be zero?

// channel bonding 10 GbE, or other hardware

// we use u16 as "slot number", (u16)~0 is "FREE".
// If you use >= 292 kB on-disk ring buffer,
// this is the maximum you can use:
pub const DRBD_AL_EXTENTS_MAX: c_uint = 0xfffeU;

// }
// drbdsetup XY resize -d Z
// you are free to reduce the device size to nothing, if you want to.
// the upper limit with 64bit kernel, enough ram and flexible meta data
// is 1 PiB, currently.
// DRBD_MAX_SECTORS

// We used to ignore the discard_zeroes_data setting.
// To not change established (and expected) behaviour,
// by default assume that, for discard_zeroes_data=0,
// we can make that an effective discard_zeroes_data=1,
// if we only explicitly zero-out unaligned partial chunks.

// Some backends pretend to support WRITE SAME,
// but fail such requests when they are actually submitted.
// This is to tell DRBD to not even try.

