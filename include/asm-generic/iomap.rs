//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/iomap.h
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
// These are the "generic" interfaces for doing new-style
// memory-mapped or PIO accesses. Architectures may do
// their own arch-optimized versions, these just act as
// wrappers around the old-style IO register access functions:
// read[bwl]/write[bwl]/in[bwl]/out[bwl]
//
// Don't include this directly, include it from <asm/io.h>.
//
// Read/write from/to an (offsettable) iomem cookie. It might be a PIO
// access or a MMIO access, these functions don't care. The info is
// encoded in the hardware mapping set up by the mapping functions
// (or the cookie itself, depending on implementation and hw).
//
// The generic routines just encode the PIO/MMIO as part of the
// cookie, and coldly assume that the MMIO IO mappings are not
// in the low address range. Architectures for which this is not
// true can't use this generic implementation.
//
extern "C" {
    pub fn ioread8(: *const void __iomem) -> c_uint;
}
extern "C" {
    pub fn ioread16(: *const void __iomem) -> c_uint;
}
extern "C" {
    pub fn ioread16be(: *const void __iomem) -> c_uint;
}
extern "C" {
    pub fn ioread32(: *const void __iomem) -> c_uint;
}
extern "C" {
    pub fn ioread32be(: *const void __iomem) -> c_uint;
}
extern "C" {
    pub fn __ioread64_lo_hi(addr: *const void __iomem) -> u64;
}
extern "C" {
    pub fn __ioread64_hi_lo(addr: *const void __iomem) -> u64;
}
extern "C" {
    pub fn __ioread64be_lo_hi(addr: *const void __iomem) -> u64;
}
extern "C" {
    pub fn __ioread64be_hi_lo(addr: *const void __iomem) -> u64;
}
extern "C" {
    pub fn iowrite8(_arg: u8, : *mut void __iomem);
}
extern "C" {
    pub fn iowrite16(_arg: u16, : *mut void __iomem);
}
extern "C" {
    pub fn iowrite16be(_arg: u16, : *mut void __iomem);
}
extern "C" {
    pub fn iowrite32(_arg: u32, : *mut void __iomem);
}
extern "C" {
    pub fn iowrite32be(_arg: u32, : *mut void __iomem);
}
extern "C" {
    pub fn __iowrite64_lo_hi(val: u64, addr: *mut void __iomem);
}
extern "C" {
    pub fn __iowrite64_hi_lo(val: u64, addr: *mut void __iomem);
}
extern "C" {
    pub fn __iowrite64be_lo_hi(val: u64, addr: *mut void __iomem);
}
extern "C" {
    pub fn __iowrite64be_hi_lo(val: u64, addr: *mut void __iomem);
}
//
// "string" versions of the above. Note that they
// use native byte ordering for the accesses (on
// the assumption that IO and memory agree on a
// byte order, and CPU byteorder is irrelevant).
//
// They do _not_ update the port address. If you
// want MMIO that copies stuff laid out in MMIO
// memory across multiple ports, use "memcpy_toio()"
// and friends.
//
extern "C" {
    pub fn ioread8_rep(port: *const void __iomem, buf: *mut c_void, count: c_ulong);
}
extern "C" {
    pub fn ioread16_rep(port: *const void __iomem, buf: *mut c_void, count: c_ulong);
}
extern "C" {
    pub fn ioread32_rep(port: *const void __iomem, buf: *mut c_void, count: c_ulong);
}
extern "C" {
    pub fn iowrite8_rep(port: *mut void __iomem, buf: *const c_void, count: c_ulong);
}
extern "C" {
    pub fn iowrite16_rep(port: *mut void __iomem, buf: *const c_void, count: c_ulong);
}
extern "C" {
    pub fn iowrite32_rep(port: *mut void __iomem, buf: *const c_void, count: c_ulong);
}

// Create a virtual mapping cookie for an IO port range
extern "C" {
    pub fn ioport_unmap(: *mut void __iomem);
}

// See the comment in asm-generic/io.h about ioremap_np().

