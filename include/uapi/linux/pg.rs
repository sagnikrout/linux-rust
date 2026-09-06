//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/pg.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
// pg.h (c) 1998  Grant R. Guenther <grant@torque.net>
//

pub const PG_MAX_DATA: c_int = 32768;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_write_hdr {
    pub /: *mut *mut char magic; / == PG_MAGIC,
    pub /: *mut *mut char func; / PG_RESET or PG_COMMAND,
    pub /: *mut *mut int dlen; / number of bytes expected to transfer,
    pub /: *mut *mut int timeout; / number of seconds before timeout,
    pub /: *mut *mut char packet[12]; / packet command,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_read_hdr {
    pub /: *mut *mut char magic; / == PG_MAGIC,
    pub /: *mut *mut char scsi; / "scsi" status == sense key,
    pub /: *mut *mut int dlen; / size of device transfer request,
    pub /: *mut *mut int duration; / time in seconds command took,
    pub /: *mut *mut char pad[12]; / not used,
}
