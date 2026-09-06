//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/fmt/riscv.h
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
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES
//
// RISC-V page table
//
// This is described in Sections:
// 12.3. Sv32: Page-Based 32-bit Virtual-Memory Systems
// 12.4. Sv39: Page-Based 39-bit Virtual-Memory System
// 12.5. Sv48: Page-Based 48-bit Virtual-Memory System
// 12.6. Sv57: Page-Based 57-bit Virtual-Memory System
// of the "The RISC-V Instruction Set Manual: Volume II"
//
// This includes the contiguous page extension from:
// Chapter 13. "Svnapot" Extension for NAPOT Translation Contiguity,
// Version 1.0
//
// The table format is sign extended and supports leafs in every level. The spec
// doesn't talk a lot about levels, but level here is the same as i=LEVELS-1 in
// the spec.
//

// fsc.PPN is 44 bits wide, all PPNs are 4k aligned
// PTE bits
// Svnapot encodings for ppn[0]

extern "C" {
    pub fn oalog2_mul(_arg: FIELD_GET(RISCVPT_PPN, _arg: pts->entry), _arg: PT_GRANULE_LG2SZ) -> return;
}

extern "C" {
    pub fn oalog2_mul(_arg: FIELD_GET(RISCVPT_PPN, _arg: pts->entry), _arg: PT_GRANULE_LG2SZ) -> return;
}

// Body in pt_fmt_defaults.h
extern "C" {
    pub fn pt_table_item_lg2sz(pts: *const pt_state) -> c_uint;
}
extern "C" {
    pub fn ilog2(_arg: 16) -> return;
}
extern "C" {
    pub fn ilog2(_arg: 1) -> return;
}

extern "C" {
    pub fn ilog2(_arg: 16) -> return;
}
extern "C" {
    pub fn ilog2(_arg: 1) -> return;
}

// FIXME does riscv need this to be cmpxchg?

extern "C" {
    pub fn pt_table_install64(_arg: pts, _arg: entry) -> return;
}

// --- iommu

// The common struct is in the per-format common struct
// Caller must specify a supported combination of flags

//
// See Table 3. Encodings of iosatp.MODE field" for DC.tx.SXL = 0:
// 8 = Sv39 = top level 2
// 9 = Sv38 = top level 3
// 10 = Sv57 = top level 4
//

