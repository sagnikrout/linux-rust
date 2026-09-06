//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/of_address.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_pci_range_parser {
    pub node: *mut device_node,
    pub bus: *const of_bus,
    pub range: *const __be32,
    pub end: *const __be32,
    pub na: c_int,
    pub ns: c_int,
    pub pna: c_int,
    pub dma: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_pci_range {
    pub pci_addr: u64,
    pub bus_addr: u64,
}

//
// of_range_count - Get the number of "ranges" or "dma-ranges" entries
// @parser:	Parser state initialized by of_range_parser_init()
//
// Returns the number of entries or 0 if none.
//
// Note that calling this within or after the for_each_of_range() iterator will
// be inaccurate giving the number of entries remaining.
//
// Translate a DMA address from device space to CPU space

extern "C" {
    pub fn of_translate_address(np: *mut device_node, addr: *const __be32) -> u64;
}
// Extract an address from a device, returns the region size and
// the address space flags too. The PCI version uses a BAR number
// instead of an absolute index
//
extern "C" {
    pub fn of_property_read_reg(np: *mut device_node, idx: c_int, addr: *mut u64, size: *mut u64) -> c_int;
}
extern "C" {
    pub fn of_dma_is_coherent(np: *mut device_node) -> bool;
}

extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn __of_get_address(_arg: dev, _arg: index, _arg: -1, _arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __of_get_address(_arg: dev, _arg: -1, _arg: bar_no, _arg: size, _arg: flags) -> return;
}
