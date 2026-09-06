//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/io.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Generic I/O port emulation.
//
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// prevent prefetching of coherent DMA data ahead of a dma-complete

// flush writes to coherent DMA data before possibly triggering a DMA read

// serialize device access against a spin_unlock, usually handled there.

//
// "__DISABLE_TRACE_MMIO__" flag can be used to disable MMIO tracing for
// specific kernel drivers in case of excessive/unwanted logging.
//
// Usage: Add a #define flag at the beginning of the driver file.
// Ex: #define __DISABLE_TRACE_MMIO__
// #include <...>
// ...
//

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

pub const IO_SPACE_LIMIT: c_uint = 0xffff;

//
// {in,out}{b,w,l}() access little endian I/O. {in,out}{b,w,l}_p() can be
// implemented on hardware that needs an additional delay for I/O accesses to
// take effect.
//

extern "C" {
    pub fn inb(_arg: addr) -> return;
}

extern "C" {
    pub fn inw(_arg: addr) -> return;
}

extern "C" {
    pub fn inl(_arg: addr) -> return;
}

//
// {in,out}s{b,w,l}{,_p}() are variants of the above that repeatedly access a
// single I/O port multiple times.
//

extern "C" {
    pub fn readb(_arg: addr) -> return;
}

extern "C" {
    pub fn readw(_arg: addr) -> return;
}

extern "C" {
    pub fn readl(_arg: addr) -> return;
}

extern "C" {
    pub fn readq(_arg: addr) -> return;
}

extern "C" {
    pub fn swab16(_arg: readw(addr)) -> return;
}

extern "C" {
    pub fn swab32(_arg: readl(addr)) -> return;
}

extern "C" {
    pub fn swab64(_arg: readq(addr)) -> return;
}

//
// Change virtual addresses to physical addresses and vv.
// These are pretty trivial
//

extern "C" {
    pub fn __pa(long)address: (unsigned) -> return;
}

extern "C" {
    pub fn __va(_arg: address) -> return;
}

//
// DOC: ioremap() and ioremap_*() variants
//
// Architectures with an MMU are expected to provide ioremap() and iounmap()
// themselves or rely on GENERIC_IOREMAP.  For NOMMU architectures we provide
// a default nop-op implementation that expect that the physical address used
// for MMIO are already marked as uncached, and can be used as kernel virtual
// addresses.
//
// ioremap_wc() and ioremap_wt() can provide more relaxed caching attributes
// for specific drivers if the architecture choses to implement them.  If they
// are not implemented we fall back to plain ioremap. Conversely, ioremap_np()
// can provide stricter non-posted write semantics if the architecture
// implements them.
//

extern "C" {
    pub fn iounmap(addr: *mut volatile void __iomem);
}
extern "C" {
    pub fn generic_iounmap(addr: *mut volatile void __iomem);
}

// _PAGE_IOREMAP needs to be supplied by the architecture
extern "C" {
    pub fn ioremap_prot(_arg: addr, _arg: size, _arg: __pgprot(_PAGE_IOREMAP)) -> return;
}

//
// ioremap_uc is special in that we do require an explicit architecture
// implementation.  In general you do not want to use this function in a
// driver and use plain ioremap, which is uncached by default.  Similarly
// architectures should not implement it unless they have a very good
// reason.
//

//
// ioremap_np needs an explicit architecture implementation, as it
// requests stronger semantics than regular ioremap(). Portable drivers
// should instead use one of the higher-level abstractions, like
// devm_ioremap_resource(), to choose the correct variant for any given
// device and bus. Portable drivers with a good reason to want non-posted
// write semantics should always provide an ioremap() fallback in case
// ioremap_np() is not available.
//

// Macro flag: #define ARCH_HAS_GENERIC_IOPORT_MAP

extern "C" {
    pub fn ioport_unmap(p: *mut void __iomem);
}

// Macro flag: #define ARCH_WANTS_GENERIC_PCI_IOUNMAP

extern "C" {
    pub fn __va(_arg: addr) -> return;
}

//
// memset_io -	Set a range of I/O memory to a constant value
// @addr:	The beginning of the I/O-memory range to set
// @val:	The value to set the memory to
// @count:	The number of bytes to set
//
// Set a range of I/O memory to a given value.
//
extern "C" {
    pub fn memset_io(addr: *mut volatile void __iomem, val: c_int, count: usize);
}

//
// memcpy_fromio -	Copy a block of data from I/O memory
// @dst:		The (RAM) destination for the copy
// @src:		The (I/O memory) source for the data
// @count:		The number of bytes to copy
//
// Copy a block of data from I/O memory.
//
extern "C" {
    pub fn memcpy_fromio(dst: *mut c_void, src: *const volatile void __iomem, count: usize);
}

//
// memcpy_toio -	Copy a block of data into I/O memory
// @dst:		The (I/O memory) destination for the copy
// @src:		The (RAM) source for the data
// @count:		The number of bytes to copy
//
// Copy a block of data to I/O memory.
//
extern "C" {
    pub fn memcpy_toio(dst: *mut volatile void __iomem, src: *const c_void, count: usize);
}

extern "C" {
    pub fn devmem_is_allowed(pfn: c_ulong) -> c_int;
}

