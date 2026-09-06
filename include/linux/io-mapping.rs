//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/io-mapping.h
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
// Copyright © 2008 Keith Packard <keithp@keithp.com>
//

//
// The io_mapping mechanism provides an abstraction for mapping
// individual pages from an io device to the CPU in an efficient fashion.
//
// See Documentation/driver-api/io-mapping.rst
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_mapping {
    pub base: resource_size_t,
    pub size: c_ulong,
    pub prot: pgprot_t,
    pub iomem: *mut void __iomem,
}

//
// For small address space machines, mapping large objects
// into the kernel virtual space isn't practical. Where
// available, use fixmap support to dynamically map pages
// of the object at run time.
//
// Atomic map/unmap
extern "C" {
    pub fn __iomap_local_pfn_prot(_arg: PHYS_PFN(phys_addr), _arg: mapping->prot) -> return;
}
extern "C" {
    pub fn __iomap_local_pfn_prot(_arg: PHYS_PFN(phys_addr), _arg: mapping->prot) -> return;
}
extern "C" {
    pub fn ioremap_wc(_arg: phys_addr, _arg: size) -> return;
}

// Create the io_mapping object
// Non-atomic map/unmap
// Atomic map/unmap
extern "C" {
    pub fn io_mapping_map_wc(_arg: mapping, _arg: offset, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn io_mapping_map_wc(_arg: mapping, _arg: offset, _arg: PAGE_SIZE) -> return;
}

