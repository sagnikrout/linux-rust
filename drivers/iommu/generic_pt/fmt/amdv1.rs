//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/fmt/amdv1.h
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
// Copyright (c) 2024-2025, NVIDIA CORPORATION & AFFILIATES
//
// AMD IOMMU v1 page table
//
// This is described in Section "2.2.3 I/O Page Tables for Host Translations"
// of the "AMD I/O Virtualization Technology (IOMMU) Specification"
//
// Note the level numbering here matches the core code, so level 0 is the same
// as mode 1.
//

//
// The IOMMUFD selftest uses the AMDv1 format with some alterations It
// uses a 2k page size to test cases where the CPU page size is not the
// same.
//

// The DTE only has these bits for the top phyiscal address
// PTE bits
//
// gcc 13 has a bug where it thinks the output of FIELD_GET() is an enum, make
// these defines to avoid it.
//
pub const AMDV1PT_FMT_NL_DEFAULT: c_int = 0;
pub const AMDV1PT_FMT_NL_SIZE: c_int = 7;
extern "C" {
    pub fn oalog2_mul(_arg: FIELD_GET(AMDV1PT_FMT_OA, _arg: entry), _arg: PT_GRANULE_LG2SZ) -> return;
}

// Returns the oa for the start of the contiguous entry
extern "C" {
    pub fn oalog2_mul(_arg: oa, _arg: PT_GRANULE_LG2SZ) -> return;
}

//
// Table 15: Page Table Level Parameters
// The top most level cannot have translation entries
//

// Body in pt_fmt_defaults.h
extern "C" {
    pub fn pt_table_item_lg2sz(pts: *const pt_state) -> c_uint;
}
extern "C" {
    pub fn ilog2(_arg: 1) -> return;
}
//
// The contiguous size is encoded in the length of a string of 1's in
// the low bits of the OA. Reverse the equation:
// code = log2_to_int(num_contig_lg2 + item_lg2sz -
// PT_GRANULE_LG2SZ - 1) - 1
// Which can be expressed as:
// num_contig_lg2 = oalog2_ffz(code) + 1 -
// item_lg2sz - PT_GRANULE_LG2SZ
//
// Assume the bit layout is correct and remove the masking. Reorganize
// the equation to move all the arithmetic before the ffz.
//
extern "C" {
    pub fn ffz_t(_arg: u32, _arg: code) -> return;
}

//
// Top entry covers bits [63:57] only, this is handled through
// max_vasz_lg2.
//

//
// Table 14: Example Page Size Encodings
// Address bits 51:32 can be used to encode page sizes greater than 4
// Gbytes. Address bits 63:52 are zero-extended.
//
// 512GB Pages are not supported due to a hardware bug.
// Otherwise every power of two size is supported.
//

// See amdv1pt_clear_entries()

//
// IR and IW are ANDed from the table levels along with the PTE. We
// always control permissions from the PTE, so always set IR and IW for
// tables.
//
extern "C" {
    pub fn pt_table_install64(_arg: pts, _arg: entry) -> return;
}

//
// gcc generates rep stos for the io-pgtable code, and this difference
// can show in microbenchmarks with larger contiguous page sizes.
// rep is slower for small cases.
//

extern "C" {
    pub fn try_cmpxchg64(_arg: tablep, _arg: &pts->entry, _arg: new) -> return;
}

// --- iommu

// The common struct is in the per-format common struct
//
// Ideally we'd have an IOMMU_ENCRYPTED flag set by higher levels to
// control this. For now if the tables use sme_set then so do the ptes.
//

// Matches what io_pgtable does

