//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/intel/pasid.h
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
// pasid.h - PASID idr, table and entry header
//
// Copyright (C) 2018 Intel Corporation
//
// Author: Lu Baolu <baolu.lu@linux.intel.com>
//
pub const PASID_MAX: c_uint = 0x100000;
pub const PASID_PTE_MASK: c_uint = 0x3F;
pub const PASID_PTE_PRESENT: c_int = 1;
pub const PASID_PTE_FPD: c_int = 2;

pub const PASID_PDE_SHIFT: c_int = 6;
pub const MAX_NR_PASID_BITS: c_int = 20;

//
// The PASID_FLAG_FL5LP flag Indicates using 5-level paging for first-
// level translation, otherwise, 4-level paging will be used.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasid_dir_entry {
    pub val: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasid_entry {
    pub val: [u64; 8],
}

// The representative of a PASID table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pasid_table {
    pub /: *mut *mut *mut void table; / pasid table pointer,
    pub /: *mut *mut u32 max_pasid; / max pasid,
}

// Get PRESENT bit of a PASID directory entry.
// Get PASID table from a PASID directory entry.
extern "C" {
    pub fn phys_to_virt(PDE_PFN_MASK: READ_ONCE(pde->val) &) -> return;
}
// Get PRESENT bit of a PASID table entry.
// Get FPD(Fault Processing Disable) bit of a PASID table entry
// Get PGTT field of a PASID table entry
extern "C" {
    pub fn READ_ONCE(_arg: *mut ptr) -> return;
}
//
// Setup the DID(Domain Identifier) field (Bit 64~79) of scalable mode
// PASID entry.
//
// Get domain ID value of a scalable mode PASID entry.
//
// Setup the SLPTPTR(Second Level Page Table Pointer) field (Bit 12~63)
// of a scalable mode PASID entry.
//
// Setup the AW(Address Width) field (Bit 2~4) of a scalable mode PASID
// entry.
//
// Setup the PGTT(PASID Granular Translation Type) field (Bit 6~8)
// of a scalable mode PASID entry.
//
// Enable fault processing by clearing the FPD(Fault Processing
// Disable) field (Bit 1) of a scalable mode PASID entry.
//
// Enable second level A/D bits by setting the SLADE (Second Level
// Access Dirty Enable) field (Bit 9) of a scalable mode PASID
// entry.
//
// Disable second level A/D bits by clearing the SLADE (Second Level
// Access Dirty Enable) field (Bit 9) of a scalable mode PASID
// entry.
//
// Checks if second level A/D bits specifically the SLADE (Second Level
// Access Dirty Enable) field (Bit 9) of a scalable mode PASID
// entry is set.
//
extern "C" {
    pub fn pasid_get_bits(9: &pe->val[0]) & (1 <<) -> return;
}
//
// Setup the SRE(Supervisor Request Enable) field (Bit 128) of a
// scalable mode PASID entry.
//
// Setup the WPE(Write Protect Enable) field (Bit 132) of a
// scalable mode PASID entry.
//
// Setup the P(Present) field (Bit 0) of a scalable mode PASID
// entry.
//
// Clear the Present (P) bit (bit 0) of a scalable-mode PASID table entry.
// This initiates the transition of the entry's ownership from hardware
// to software. The caller is responsible for fulfilling the invalidation
// handshake recommended by the VT-d spec, Section 6.5.3.3 (Guidance to
// Software for Invalidations).
//
// Setup Page Walk Snoop bit (Bit 87) of a scalable mode PASID
// entry.
//
// Setup the Page Snoop (PGSNP) field (Bit 88) of a scalable mode
// PASID entry.
//
// Setup the First Level Page table Pointer field (Bit 140~191)
// of a scalable mode PASID entry.
//
// Setup the First Level Paging Mode field (Bit 130~131) of a
// scalable mode PASID entry.
//
// Setup the Extended Access Flag Enable (EAFE) field (Bit 135)
// of a scalable mode PASID entry.
//
extern "C" {
    pub fn intel_pasid_alloc_table(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn intel_pasid_free_table(dev: *mut device);
}
extern "C" {
    pub fn intel_pasid_setup_sm_context(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn intel_pasid_teardown_sm_context(dev: *mut device);
}
