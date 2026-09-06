//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/decompress/mm.h
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
// linux/compr_mm.h
//
// Memory management for pre-boot and ramdisk uncompressors
//
// Authors: Alain Knaff <alain@knaff.lu>
//

// Code active when included from pre-boot environment:
//
// Some architectures want to ensure there is no local data in their
// pre-boot environment, so that data can arbitrarily relocated (via
// GOT references).  This is achieved by defining STATIC_RW_DATA to
// be null.
//

//
// When an architecture needs to share the malloc()/free() implementation
// between compilation units, it needs to have non-local visibility.
//

// A trivial malloc implementation, adapted from
// malloc by Hannu Savolainen 1993 and Matthias Urlichs 1994
//

// Macro flag: #define INIT

// Code active when compiled standalone for use when loading ramdisk:

// Use defines rather than static inline in order to avoid spurious
// warnings when not needed (indeed large_malloc / large_free are not
// needed by inflate

// Macro flag: #define STATIC

