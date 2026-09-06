//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/asm-generic/io.h
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

// prevent prefetching of coherent DMA data ahead of a dma-complete

// flush writes to coherent DMA data before possibly triggering a DMA read

// serialize device access against a spin_unlock, usually handled there.

pub const _THIS_IP_: c_int = 0;

//
// __raw_{read,write}{b,w,l,q}() access memory in native endianness.
//
// On some architectures memory mapped IO needs to be accessed differently.
// On the simple architectures, we just read/write the memory location
// directly.
//

// (volatile u8  *)addr = value;

// (volatile u16  *)addr = value;

// (volatile u32  *)addr = value;

// (volatile u64  *)addr = value;

//
// {read,write}{b,w,l,q}() access little endian memory and return result in
// native endianness.
//

//
// {read,write}{b,w,l,q}_relaxed() are like the regular version, but
// are not guaranteed to provide ordering against spinlocks or memory
// accesses.
//

//
// {read,write}s{b,w,l,q}() repeatedly access the same memory address in
// native endianness in 8-, 16-, 32- or 64-bit chunks (@count times).
//

// buf++ = x;

// buf++ = x;

// buf++ = x;

// buf++ = x;

