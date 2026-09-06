//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iommu/generic_pt/fmt/x86_64.h
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
// x86 page table. Supports the 4 and 5 level variations.
//
// The 4 and 5 level version is described in:
// Section "4.4 4-Level Paging and 5-Level Paging" of the Intel Software
// Developer's Manual Volume 3
//
// Section "9.7 First-Stage Paging Entries" of the "Intel Virtualization
// Technology for Directed I/O Architecture Specification"
//
// Section "2.2.6 I/O Page Tables for Guest Translations" of the "AMD I/O
// Virtualization Technology (IOMMU) Specification"
//
// It is used by x86 CPUs, AMD and VT-d IOMMU HW.
//
// Note the 3 level format is very similar and almost implemented here. The
// reserved/ignored layout is different and there are functional bit
// differences.
//
// This format uses PT_FEAT_SIGN_EXTEND to have a upper/non-canonical/lower
// split. PT_FEAT_SIGN_EXTEND is optional as AMD IOMMU sometimes uses non-sign
// extended addressing with this page table format.
//
// The named levels in the spec map to the pts->level as:
// Table/PTE - 0
// Directory/PDE - 1
// Directory Ptr/PDPTE - 2
// PML4/PML4E - 3
// PML5/PML5E - 4
//

//
// For AMD the GCR3 Base only has these bits. For VT-d FSPTPTR is 4k
// aligned and is limited by the architected HAW
//
// Shared descriptor bits
// PDPTE/PDE

extern "C" {
    pub fn pt_table_install64(_arg: pts, _arg: entry) -> return;
}

// Bits marked Ignored/AVL in the specification
extern "C" {
    pub fn BIT(_arg: 9) -> return;
}
extern "C" {
    pub fn BIT(_arg: 11) -> return;
}
extern "C" {
    pub fn BIT_ULL(52: (bitnr - 2) +) -> return;
}
// Some bits in 8,6,4,3 are available in some entries

// --- iommu

// The common struct is in the per-format common struct
//
// Ideally we'd have an IOMMU_ENCRYPTED flag set by higher levels to
// control this. For now if the tables use sme_set then so do the ptes.
//

// AMD IOMMU PASID 0 formats with no SIGN_EXTEND

