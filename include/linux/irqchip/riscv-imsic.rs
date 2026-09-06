//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/riscv-imsic.h
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
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
// Copyright (C) 2022 Ventana Micro Systems Inc.
//

pub const IMSIC_MMIO_PAGE_SHIFT: c_int = 12;

pub const IMSIC_MMIO_PAGE_LE: c_uint = 0x00;
pub const IMSIC_MMIO_PAGE_BE: c_uint = 0x04;
pub const IMSIC_MIN_ID: c_int = 63;
pub const IMSIC_MAX_ID: c_int = 2048;
pub const IMSIC_EIDELIVERY: c_uint = 0x70;
pub const IMSIC_EITHRESHOLD: c_uint = 0x72;
pub const IMSIC_EIP0: c_uint = 0x80;
pub const IMSIC_EIP63: c_uint = 0xbf;
pub const IMSIC_EIPx_BITS: c_int = 32;
pub const IMSIC_EIE0: c_uint = 0xc0;
pub const IMSIC_EIE63: c_uint = 0xff;
pub const IMSIC_EIEx_BITS: c_int = 32;

pub const IMSIC_MMIO_SETIPNUM_LE: c_uint = 0x00;
pub const IMSIC_MMIO_SETIPNUM_BE: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsic_local_config {
    pub msi_pa: phys_addr_t,
    pub msi_va: *mut void __iomem,
// Number of guest interrupt files per-HART
    pub nr_guest_files: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imsic_global_config {
//
// MSI Target Address Scheme
//
// XLEN-1                                                12     0
// |                                                     |     |
// -------------------------------------------------------------
// |xxxxxx|Group Index|xxxxxxxxxxx|HART Index|Guest Index|  0  |
// -------------------------------------------------------------
//
// Bits representing Guest index, HART index, and Group index
    pub guest_index_bits: u32,
    pub hart_index_bits: u32,
    pub group_index_bits: u32,
    pub group_index_shift: u32,
// Global base address matching all target MSI addresses
    pub base_addr: phys_addr_t,
// Number of interrupt identities
    pub nr_ids: u32,
// Number of guest interrupt identities
    pub nr_guest_ids: u32,
// Number of guest interrupt files across all HARTs
    pub nr_guest_files: u32,
// Per-CPU IMSIC addresses
    pub local: *mut imsic_local_config __percpu,
}

extern "C" {
    pub fn imsic_platform_acpi_probe(fwnode: *mut fwnode_handle) -> c_int;
}

