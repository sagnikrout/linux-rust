//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/generic_pt/common.h
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

//
// DOC: Generic Radix Page Table
//
// Generic Radix Page Table is a set of functions and helpers to efficiently
// parse radix style page tables typically seen in HW implementations. The
// interface is built to deliver similar code generation as the mm's pte/pmd/etc
// system by fully inlining the exact code required to handle each table level.
//
// Like the mm subsystem each format contributes its parsing implementation
// under common names and the common code implements the required algorithms.
//
// The system is divided into three logical levels:
//
// - The page table format and its manipulation functions
// - Generic helpers to give a consistent API regardless of underlying format
// - An algorithm implementation (e.g. IOMMU/DRM/KVM/MM)
//
// Multiple implementations are supported. The intention is to have the generic
// format code be re-usable for whatever specialized implementation is required.
// The generic code is solely about the format of the radix tree; it does not
// include memory allocation or higher level decisions that are left for the
// implementation.
//
// The generic framework supports a superset of functions across many HW
// implementations:
//
// - Entries comprised of contiguous blocks of IO PTEs for larger page sizes
// - Multi-level tables, up to 6 levels. Runtime selected top level
// - Runtime variable table level size (ARM's concatenated tables)
// - Expandable top level allowing dynamic sizing of table levels
// - Optional leaf entries at any level
// - 32-bit/64-bit virtual and output addresses, using every address bit
// - Dirty tracking
// - Sign extended addressing
//
// struct pt_common - struct for all page table implementations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_common {
//
// @top_of_table: Encodes the table top pointer and the top level in a
// single value. Must use READ_ONCE/WRITE_ONCE to access it. The lower
// bits of the aligned table pointer are used for the level.
//
    pub top_of_table: uintptr_t,
//
// @max_oasz_lg2: Maximum number of bits the OA can contain. Upper bits
// must be zero. This may be less than what the page table format
// supports, but must not be more.
//
    pub max_oasz_lg2: u8,
//
// @max_vasz_lg2: Maximum number of bits the VA can contain. Upper bits
// are 0 or 1 depending on pt_full_va_prefix(). This may be less than
// what the page table format supports, but must not be more. When
// PT_FEAT_DYNAMIC_TOP is set this reflects the maximum VA capability.
//
    pub max_vasz_lg2: u8,
//
// @features: Bitmap of `enum pt_features`
//
    pub features: c_uint,
}

// Encoding parameters for top_of_table
//
// enum pt_features - Features turned on in the table. Each symbol is a bit
// position.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pt_features {
//
// @PT_FEAT_DMA_INCOHERENT: Cache flush page table memory before
// assuming the HW can read it. Otherwise a SMP release is sufficient
// for HW to read it.
//
    PT_FEAT_DMA_INCOHERENT,
//
// @PT_FEAT_FULL_VA: The table can span the full VA range from 0 to
// PT_VADDR_MAX.
//
    PT_FEAT_FULL_VA,
//
// @PT_FEAT_DYNAMIC_TOP: The table's top level can be increased
// dynamically during map. This requires HW support for atomically
// setting both the table top pointer and the starting table level.
//
    PT_FEAT_DYNAMIC_TOP,
//
// @PT_FEAT_SIGN_EXTEND: The top most bit of the valid VA range sign
// extends up to the full pt_vaddr_t. This divides the page table into
// three VA ranges::
//
// 0         -> 2^N - 1             Lower
// 2^N       -> (MAX - 2^N - 1)     Non-Canonical
// MAX - 2^N -> MAX                 Upper
//
// In this mode pt_common::max_vasz_lg2 includes the sign bit and the
// upper bits that don't fall within the translation are just validated.
//
// If not set there is no sign extension and valid VA goes from 0 to 2^N
// - 1.
//
    PT_FEAT_SIGN_EXTEND,
//
// @PT_FEAT_FLUSH_RANGE: IOTLB maintenance is done by flushing IOVA
// ranges which will clean out any walk cache or any IOPTE fully
// contained by the range. The optimization objective is to minimize the
// number of flushes even if ranges include IOVA gaps that do not need
// to be flushed.
//
    PT_FEAT_FLUSH_RANGE,
//
// @PT_FEAT_FLUSH_RANGE_NO_GAPS: Like PT_FEAT_FLUSH_RANGE except that
// the optimization objective is to only flush IOVA that has been
// changed. This mode is suitable for cases like hypervisor shadowing
// where flushing unchanged ranges may cause the hypervisor to reparse
// significant amount of page table.
//
    PT_FEAT_FLUSH_RANGE_NO_GAPS,
//
// @PT_FEAT_DETAILED_GATHER: Fill in the struct iommu_iotlb_gather pt
// sub structure with information about which levels were changed.
//
    PT_FEAT_DETAILED_GATHER,
// private:
    PT_FEAT_FMT_START,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_amdv1 {
    pub common: pt_common,
}

//
// The memory backing the tables is encrypted. Use __sme_set() to adjust
// the page table pointers in the tree. This only works with
// CONFIG_AMD_MEM_ENCRYPT.
//
// The PTEs are set to prevent cache incoherent traffic, such as PCI no
// snoop. This is set either at creation time or before the first map
// operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_vtdss {
    pub common: pt_common,
}

//
// The PTEs are set to prevent cache incoherent traffic, such as PCI no
// snoop. This is set either at creation time or before the first map
// operation.
//
// Prevent creating read-only PTEs. Used to work around HW errata
// ERRATA_772415_SPR17.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_riscv_32 {
    pub common: pt_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_riscv_64 {
    pub common: pt_common,
}

//
// Support the 64k contiguous page size following the Svnapot extension.
//
// Support Svpbmt extension: encode page-based memory type (PBMT) in PTEs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pt_x86_64 {
    pub common: pt_common,
}

//
// The memory backing the tables is encrypted. Use __sme_set() to adjust
// the page table pointers in the tree. This only works with
// CONFIG_AMD_MEM_ENCRYPT.
//
