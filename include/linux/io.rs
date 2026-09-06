//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/io.h
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
// Copyright 2006 PathScale, Inc.  All Rights Reserved.
//

extern "C" {
    pub fn __iowrite32_copy(to: *mut void __iomem, from: *const c_void, count: usize);
}

extern "C" {
    pub fn __ioread32_copy(to: *mut c_void, from: *const void __iomem, count: usize);
}

extern "C" {
    pub fn __iowrite64_copy(to: *mut void __iomem, from: *const c_void, count: usize);
}

//
// Managed iomap interface
//

extern "C" {
    pub fn devm_ioport_unmap(dev: *mut device, addr: *mut void __iomem);
}

extern "C" {
    pub fn devm_iounmap(dev: *mut device, addr: *mut void __iomem);
}
extern "C" {
    pub fn devm_ioremap_release(dev: *mut device, res: *mut c_void);
}
extern "C" {
    pub fn devm_memunmap(dev: *mut device, addr: *mut c_void);
}
// architectures can override this

//
// The PCI specifications (Rev 3.0, 3.2.5 "Transaction Ordering and
// Posting") mandate non-posted configuration transactions. This default
// implementation attempts to use the ioremap_np() API to provide this
// on arches that support it, and falls back to ioremap() on those that
// don't. Overriding this function is deprecated; arches that properly
// support non-posted accesses should implement ioremap_np() instead, which
// this default implementation can then use to return mappings compliant with
// the PCI specification.
//

extern "C" {
    pub fn ioremap_np(_arg: offset, ioremap(offset: size) ?:, _arg: size) -> return;
}

//
// Some systems do not have legacy ISA devices.
// /dev/port is not a valid interface on these systems.
// So for those archs, <asm/io.h> should define the following symbol.
//

//
// Some systems (x86 without PAT) have a somewhat reliable way to mark a
// physical address range such that uncached mappings will actually
// end up write-combining.  This facility should be used in conjunction
// with pgprot_writecombine, ioremap-wc, or set_memory_wc, since it has
// no effect if the per-page mechanisms are functional.
// (On x86 without PAT, these functions manipulate MTRRs.)
//
// arch_phys_del_wc(0) or arch_phys_del_wc(any error code) is guaranteed
// to have no effect.
//

extern "C" {
    pub fn devm_arch_phys_wc_add(dev: *mut device, base: c_ulong, size: c_ulong) -> c_int;
}
// See memremap() kernel-doc for usage description...
extern "C" {
    pub fn memunmap(addr: *mut c_void);
}
//
// On x86 PAT systems we have memory tracking that keeps track of
// the allowed mappings on memory ranges. This tracking works for
// all the in-kernel mapping APIs (ioremap*), but where the user
// wishes to map a range from a physical device into user memory
// the tracking won't be updated. This API is to be used by
// drivers which remap physical device pages into userspace,
// and wants to make sure they are mapped WC and not UC.
//

