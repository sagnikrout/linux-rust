//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/ioctl.h
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
// ioctl command encoding: 32 bits total, command in lower 16 bits,
// size of the parameter structure in the lower 14 bits of the
// upper 16 bits.
// Encoding the size of the parameter structure in the ioctl request
// is useful for catching programs compiled with old versions
// and to avoid overwriting user space outside the user buffer area.
// The highest 2 bits are reserved for indicating the ``access mode''.
// NOTE: This limits the max parameter size to 16kB -1 !
//
// The following is for compatibility across the various Linux
// platforms.  The generic ioctl numbering scheme doesn't really enforce
// a type field.  De facto, however, the top 8 bits of the lower 16
// bits are indeed used as a type field, so we might just as well make
// this explicit here.  Please be sure to use the decoding macros
// below from now on.
//
pub const _IOC_NRBITS: c_int = 8;
pub const _IOC_TYPEBITS: c_int = 8;
//
// Let any architecture override either of the following before
// including this file.
//

pub const _IOC_NRSHIFT: c_int = 0;

//
// Direction bits, which any architecture can choose to override
// before including this file.
//
// NOTE: _IOC_WRITE means userland is writing and kernel is
// reading. _IOC_READ means userland is reading and kernel is writing.
//

//
// Used to create numbers.
//
// NOTE: _IOW means userland is writing and kernel is reading. _IOR
// means userland is reading and kernel is writing.
//

// used to decode ioctl numbers..

// ...and for the drivers/sound files...

