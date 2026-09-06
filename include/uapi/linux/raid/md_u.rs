//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/raid/md_u.h
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
// Different major versions are not compatible.
// Different minor versions are only downward compatible.
// Different patchlevel versions are downward and upward compatible.
//
pub const MD_MAJOR_VERSION: c_int = 0;
pub const MD_MINOR_VERSION: c_int = 90;
//
// MD_PATCHLEVEL_VERSION indicates kernel functionality.
// >=1 means different superblock formats are selectable using SET_ARRAY_INFO
// and major_version/minor_version accordingly
// >=2 means that Internal bitmaps are supported by setting MD_SB_BITMAP_PRESENT
// in the super status byte
// >=3 means that bitmap superblock version 4 is supported, which uses
// little-ending representation rather than host-endian
//
pub const MD_PATCHLEVEL_VERSION: c_int = 3;
// ioctls
// status

// configuration

// usage

// 0x31 was START_ARRAY

// 63 partitions with the alternate major number (mdp)
pub const MdpMinorShift: c_int = 6;
//
// Generic constant information
//
// Generic state information
//
// Personality information
//

// we need a value for 'no level specified' and 0
// means 'raid0', so we need something else.  This is
// for internal use only
//

//
// configuration/status of one particular disk
//
// configuration/status of one particular disk
//
