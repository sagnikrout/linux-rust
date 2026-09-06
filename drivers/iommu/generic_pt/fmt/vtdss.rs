//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/fmt/vtdss.h
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
// Copyright (c) 2024, NVIDIA CORPORATION & AFFILIATES
//
// Intel VT-d Second Stange 5/4 level page table
//
// This is described in
// Section "3.7 Second-Stage Translation"
// Section "9.8 Second-Stage Paging Entries"
//
// Of the "Intel Virtualization Technology for Directed I/O Architecture
// Specification".
//
// The named levels in the spec map to the pts->level as:
// Table/SS-PTE - 0
// Directory/SS-PDE - 1
// Directory Ptr/SS-PDPTE - 2
// PML4/SS-PML4E - 3
// PML5/SS-PML5E - 4
//

// SSPTPTR is 4k aligned and limited by HAW
// Shared descriptor bits
// PDPTE/PDE

extern "C" {
    pub fn pt_table_install64(_arg: pts, _arg: entry) -> return;
}

extern "C" {
    pub fn try_cmpxchg64(_arg: tablep, _arg: &pts->entry, _arg: new) -> return;
}

// Bits marked Ignored in the specification
extern "C" {
    pub fn BIT(_arg: 10) -> return;
}
extern "C" {
    pub fn BIT_ULL(52: (bitnr - 1) +) -> return;
}
extern "C" {
    pub fn BIT_ULL(_arg: 63) -> return;
}
// Some bits in 9-3 are available in some entries

// --- iommu

// The common struct is in the per-format common struct
//
// VTDSS does not have a present bit, so we tell if any entry is present
// by checking for R or W.
//

//
// top_level = 2 = 3 level table aw=1
// top_level = 3 = 4 level table aw=2
// top_level = 4 = 5 level table aw=3
//

