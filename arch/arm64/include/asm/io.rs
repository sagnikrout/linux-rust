//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/io.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Based on arch/arm/include/asm/io.h
//
// Copyright (C) 1996-2000 Russell King
// Copyright (C) 2012 ARM Ltd.
//

//
// Generic IO read/write.  These perform native-endian accesses.
//

extern "C" {
    pub fn volatile(%w0: "strb, (val): %1" : : "rZ", (*ptr): *mut "Qo") -> asm;
}

extern "C" {
    pub fn volatile(%w0: "strh, (val): %1" : : "rZ", (*ptr): *mut "Qo") -> asm;
}

extern "C" {
    pub fn volatile(%w0: "str, (val): %1" : : "rZ", (*ptr): *mut "Qo") -> asm;
}

extern "C" {
    pub fn volatile(%x0: "str, (val): %1" : : "rZ", (*ptr): *mut "Qo") -> asm;
}

// IO barriers

// \
// Create a dummy control dependency from the IO read to any	\
// later instructions. This ensures that a subsequent call to	\
// udelay() will be ordered due to the ISB in get_cycles().	\
// \

// Macro flag: #define __io_br(v)
// Macro flag: #define __io_aw(v)
// arm64-specific, don't use in portable drivers

//
// I/O port access primitives.
//

//
// The ARM64 iowrite implementation is intended to support drivers that want to
// use write combining. For instance PCI drivers using write combining with a 64
// byte __iowrite64_copy() expect to get a 64 byte MemWr TLP on the PCIe bus.
//
// Newer ARM core have sensitive write combining buffers, it is important that
// the stores be contiguous blocks of store instructions. Normal memcpy
// approaches have a very low chance to generate write combining.
//
// Since this is the only API on ARM64 that should be used with write combining
// it also integrates the DGH hint which is supposed to lower the latency to
// emit the large TLP from the CPU.
//
extern "C" {
    pub fn __iowrite32_copy_full(to: *mut void __iomem, from: *const c_void, count: usize);
}

extern "C" {
    pub fn __iowrite64_copy_full(to: *mut void __iomem, from: *const c_void, count: usize);
}

//
// I/O memory mapping functions.
//
extern "C" {
    pub fn arm64_ioremap_prot_hook_register(hook: ioremap_prot_hook_t) -> c_int;
}
extern "C" {
    pub fn __ioremap_prot(_arg: phys, _arg: size, _arg: prot) -> return;
}

//
// io{read,write}{16,32,64}be() macros
//

extern "C" {
    pub fn __ioremap_prot(_arg: addr, _arg: size, _arg: __pgprot(PROT_NORMAL)) -> return;
}
//
// More restrictive address range checking than the default implementation
// (PHYS_OFFSET and PHYS_MASK taken into account).
//
// Macro flag: #define ARCH_HAS_VALID_PHYS_ADDR_RANGE
extern "C" {
    pub fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> c_int;
}
extern "C" {
    pub fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> c_int;
}

extern "C" {
    pub fn arm64_rsi_is_protected(_arg: phys_addr, _arg: size) -> return;
}
